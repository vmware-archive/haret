/// Implementation of a state based OR-Set with Delta Mutation as described in
/// "Efficient State-based CRDTs by Delta-Mutation" by Almeida et. al
/// http://gsd.di.uminho.pt/members/cbm/ps/delta-crdt-draft16may2014.pdf

use std::collections::{HashMap};
use std::hash::Hash;
use std::option::Option;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Dot {
    actor: String,
    counter: u64
}

#[derive(Debug, Clone)]
pub enum Delta<T> {
    Add { element: T, dot: Dot },
    Remove { element: T, dots: Vec<Dot> }
}

/// Since we never remove tombstones from `removes`, we don't bother storing the
/// identical dots in `adds`. This reduces the amount of memory required, and it
/// also increases efficiency for membership existence checks by only requiring
/// checking `adds` for emptyness instead of requiring comparison between `adds`
/// and `removes`. It does however, increase the cost of joins.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ORSet<T: Eq + Hash> {
    name: String,
    counter: u64,
    adds: HashMap<T, Vec<Dot>>,
    removes: HashMap<T, Vec<Dot>>
}

impl<T: Eq + Hash + Clone> ORSet<T> {
    fn new(name: String) -> ORSet<T> {
        ORSet {
            name: name,
            counter: 0,
            adds: HashMap::new(),
            removes: HashMap::new()
        }
    }

    fn seen(&mut self, element: &T) -> Option<Vec<Dot>> {
        match self.adds.get(element) {
            None => None,
            Some(dots) => Some(dots.clone())
        }
    }

    fn add(&mut self, element: T) -> Delta<T> {
        self.counter += 1;
        let dot = Dot {actor: self.name.clone(), counter: self.counter};
        let delta = Delta::Add { element: element.clone(), dot: dot.clone() };
        let adds = self.adds.entry(element).or_insert(Vec::new());
        (*adds).push(dot);
        delta
    }

    fn remove(&mut self, element: T, seen: Vec<Dot>) -> Delta<T> {
        let removes = self.removes.entry(element.clone()).or_insert(Vec::new());
        for dot in &seen {
            removes.push(dot.clone());
            self.adds.remove(&element);
        }
        Delta::Remove { element: element, dots: seen}
    }

    // No overloaded functions in Rust. This feels wrong...
    /// Returns true if the state was mutated, false otherwise
    fn join_state(&mut self, from: ORSet<T>) -> bool {
        let mut mutated = false;
        for (element, dots) in from.removes.iter() {
            if self.join_remove(element.clone(), dots.clone()) {
                mutated = true;
            }
        }

        for (element, dots) in from.adds.iter() {
            for dot in dots {
                if self.join_add(element.clone(), dot.clone()) {
                    mutated = true;
                }
            }
        }
        mutated
    }

    /// Returns true if the state was mutated, false otherwise
    fn join(&mut self, delta: Delta<T>) -> bool {
        match delta {
            Delta::Add {element, dot} => self.join_add(element, dot),
            Delta::Remove { element, dots } => self.join_remove(element, dots)
        }
    }

    /// Returns true if the state was mutated, false otherwise
    fn join_add(&mut self, element: T, dot: Dot) -> bool {
        let adds = self.adds.entry(element.clone()).or_insert(Vec::new());
        if !adds.contains(&dot) {
            if let Some(removes) = self.removes.get(&element) {
                if removes.contains(&dot) {
                    false
                } else {
                    adds.push(dot);
                    true
                }
            } else {
                adds.push(dot);
                true
            }
        } else {
            false
        }
    }

    /// Returns true if the state was mutated, false otherwise
    fn join_remove(&mut self, element: T, mut dots: Vec<Dot>) -> bool {
        let adds = self.adds.entry(element.clone()).or_insert(Vec::new());
        let removes = self.removes.entry(element).or_insert(Vec::new());
        let mut mutated = false;
        while let Some(dot) = dots.pop() {
            if !removes.contains(&dot) {
                adds.retain(|x| *x != dot);
                removes.push(dot);
                mutated = true;
            }
        }
        return mutated
    }

    fn contains(&self, element: &T) -> bool {
        if let Some(adds) = self.adds.get(element) {
            if adds.is_empty() {
                false
            } else {
                true
            }
        } else {
            false
        }
    }

    fn elements(&self) -> Vec<T> {
        self.adds.iter().fold(Vec::new(), |mut acc, (elem, dots)| {
            if dots.is_empty() {
                acc
            } else {
                acc.push(elem.clone());
                acc
            }
        })
    }
}

#[cfg(test)]
mod tests {

    use quickcheck::{quickcheck, Arbitrary, Gen};
    use rand::Rng;
    use super::*;
    use std::collections::{HashMap};


    fn assert_add() -> ORSet<String> {
        let mut orset = ORSet::new("node1".to_string());
        let add = orset.add("dog".to_string());

        // Test that our add mutator is correct
        if let Delta::Add {element, dot} = add.clone() {
            assert_eq!(element, "dog".to_string());
            assert_eq!(dot.counter, 1);
            assert_eq!(dot.actor, "node1".to_string());
        } else {
            panic!("Invalid Delta: Not an Add");
        }

        // Check contents of our orset
        assert_eq!(true, orset.contains(&"dog".to_string()));
        assert_eq!(false, orset.contains(&"cat".to_string()));

        // Check that adds are idempotent
        assert_eq!(false, orset.join(add));
        orset
    }

    fn assert_successful_remove(orset: &ORSet<String>) -> ORSet<String> {
        let mut orset = orset.clone();
        let seen = orset.seen(&"dog".to_string()).unwrap();
        let remove = orset.remove("dog".to_string(), seen);

        // Test that we have a proper remove mutator
        if let Delta::Remove {dots, ..} = remove.clone() {
            assert_eq!(dots.len(), 1);
            assert_eq!(dots[0].actor, "node1".to_string());
            assert_eq!(dots[0].counter, 1);
        } else {
            panic!("Invalid Delta: Not a Remove");
        }

        // Check that removes are idempotent
        assert_eq!(false, orset.join(remove));
        assert_eq!(0, orset.elements().len());
        orset
    }

    fn assert_deltas(orset: ORSet<String>) {
        let mut orset = orset;

        let delta = Delta::Add {element: "dog".to_string(),
                                dot: Dot {actor: "node2".to_string(), counter: 1}};

        // Check that joining an Add mutator mutates the set
        assert_eq!(true, orset.join(delta.clone()));
        assert_eq!(1, orset.elements().len());

        // Check that add delta mutators are idempotent
        assert_eq!(false, orset.join(delta));

        let delta = Delta::Remove {element: "dog".to_string(),
                                   dots: vec![Dot {actor: "node2".to_string(),
                                                   counter: 1}]};

        // Check that joining a Remove mutator mutates the set
        assert_eq!(true, orset.join(delta.clone()));
        assert_eq!(0, orset.elements().len());

        // Check that remove mutators are idempotent
        assert_eq!(false, orset.join(delta));
    }

    #[test]
    fn basic() {
        let mut orset = assert_add();
        let orset2 = assert_successful_remove(&orset);

        // Ensure that states are mutated correctly
        assert_eq!(true, orset.join_state(orset2.clone()));
        assert_eq!(orset, orset2);

        // Ensure that joining states is idempotent
        assert_eq!(false, orset.join_state(orset2.clone()));
        assert_eq!(orset, orset2);

        assert_deltas(orset);
    }

    fn oneof<G: Gen, T: Clone>(g: &mut G, range: Vec<T>) -> T {
        let index = g.gen_range(0, range.len());
        let ref x = range[index];
        x.clone()
    }

    /// Create n strings of s appended with an incrementing integer from 1 to n
    fn n_strings(s: &str, n: u32) -> Vec<String> {
        let mut strings = Vec::new();
        for i in 1..n+1 {
            let mut string = s.to_string();
            string.push_str(&(i.to_string()));
            strings.push(string)
        }
        strings
    }

    #[derive(Debug, Clone)]
    enum Op {
        Add {element: String, node: String},
        Remove {element: String, node: String}
    }

    // Quickcheck generators must implement the Arbitrary trait
    impl Arbitrary for Op {
        fn arbitrary<G: Gen>(g: &mut G) -> Op {
            let elements = n_strings("element", 10);
            let element = oneof(g, elements);
            let nodes = n_strings("node", 4);
            let node = oneof(g, nodes);
            if bool::arbitrary(g) {
                Op::Add {element: element, node: node}
            } else {
                Op::Remove {element: element, node: node}
            }
        }
    }

    type ORSetsMap = HashMap<String, ORSet<String>>;
    type MutatorsMap = HashMap<String, Vec<Delta<String>>>;

    fn add_op(element: String, node: String, orsets: &mut ORSetsMap,
              mutators: &mut MutatorsMap) {
        let mut orset =
            orsets.entry(node.clone()).or_insert(ORSet::new(node.clone()));
        let delta = orset.add(element);
        let mut mutator = mutators.entry(node).or_insert(Vec::new());
        mutator.push(delta)
    }

    fn remove_op(element: String, node: String, orsets: &mut ORSetsMap,
                 mutators: &mut MutatorsMap) {
        match orsets.get_mut(&node) {
            None => (),
            Some(orset) => {
                match orset.seen(&element) {
                    None => (),
                    Some(seen) => {
                        let delta = orset.remove(element, seen);
                        let mut mutator = mutators.get_mut(&node).unwrap();
                        mutator.push(delta)
                    }
                }
            }
        }
    }

    #[test]
    fn prop_joins_equivalent() {
        fn prop(ops: Vec<Op>) -> bool {
            let mut orsets: ORSetsMap = HashMap::new();
            let mut mutators: MutatorsMap = HashMap::new();
            for op in ops {
                match op {
                    Op::Add {element, node} => {
                        add_op(element, node, &mut orsets, &mut mutators);
                    }
                    Op::Remove {element, node} => {
                        remove_op(element, node, &mut orsets, &mut mutators);
                    }
                }
            }
            let mut orsets2 = orsets.clone();
            join_deltas(&mut orsets, mutators);
            join_states(&mut orsets2);
            orsets_are_logically_equal(orsets, orsets2)
        }
        quickcheck(prop as fn(Vec<Op>) -> bool);
    }

    fn orsets_are_logically_equal(map: ORSetsMap, map2: ORSetsMap) -> bool {
        let mut equal = true;
        for orset in map.values() {
            for orset2 in map2.values() {
                if orset.elements().sort() != orset2.elements().sort() {
                    equal = false;
                }
            }
        }
        equal
    }

    fn join_deltas(orsets: &mut ORSetsMap, mutators: MutatorsMap) {
        // Apply each mutator to every node that isn't the creator of the delta
        for (node, deltas) in mutators.iter() {
            for (node2, mut orset) in orsets.iter_mut() {
                if node != node2 {
                    for delta in deltas {
                        (*orset).join(delta.clone());
                    }
                }
            }
        }
    }

    fn join_states(orsets: &mut ORSetsMap) {
        let orsets2 = orsets.clone();
        for (node, mut orset) in orsets.iter_mut() {
            for (node2, orset2) in orsets2.iter() {
                if node != node2 {
                    orset.join_state(orset2.clone());
                }
            }
        }
    }
}
