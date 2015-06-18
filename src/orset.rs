/// Implementation of a state based OR-Set with Delta Mutation as described in 
/// "Efficient State-based CRDTs by Delta-Mutation" by Almeida et. al
/// http://gsd.di.uminho.pt/members/cbm/ps/delta-crdt-draft16may2014.pdf

use std::collections::{HashMap};
use std::hash::Hash;
use std::option::Option;

#[derive(Debug, Clone, Eq, PartialEq)]
struct Dot {
    actor: String,
    counter: u64
}

#[derive(Debug, Clone)]
enum Delta<T> {
    Add { element: T, dot: Dot },
    Remove { element: T, dots: Vec<Dot> }
}

/// Since we never remove tombstones from `removes`, we don't bother storing the
/// identical dots in `adds`. This reduces the amount of memory required, and it
/// also increases efficiency for membership existence checks by only requiring
/// checking `adds` for emptyness instead of requiring comparison between `adds`
/// and `removes`. It does however, increase the cost of joins.
#[derive(Debug, Clone, Eq, PartialEq)]
struct ORSet<T: Eq + Hash> {
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

    fn add(&mut self, element: T) -> Delta<T> {
        self.counter += 1;
        let dot = Dot {actor: self.name.clone(), counter: self.counter};
        let delta = Delta::Add { element: element.clone(), dot: dot.clone() };
        let adds = self.adds.entry(element).or_insert(Vec::new());
        (*adds).push(dot);
        delta
    }

    // Don't do anything if there are no adds to remove
    fn remove(&mut self, element: T) -> Option<Delta<T>> {
        match self.adds.get_mut(&element) {
            None => None,
            Some(adds) => {
                let removes = self.removes.entry(element.clone()).or_insert(Vec::new());
                let mut dots = Vec::new();
                while let Some(dot) = adds.pop() {
                    removes.push(dot.clone());
                    dots.push(dot);
                }
                Some(Delta::Remove { element: element, dots: dots })
            }
        }
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

#[test]
fn basic() {
    let mut orset = ORSet::new("node1".to_string());
    let add = orset.add("dog".to_string());
    let add2 = add.clone();

    // save a copy for later
    let mut orset1 = orset.clone();

    // Test that our add mutator is correct
    if let Delta::Add {element, dot} = add {
        assert_eq!(element, "dog".to_string());
        assert_eq!(dot.counter, 1);
        assert_eq!(dot.actor, "node1".to_string());
    } else {
        panic!("Invalid Delta: Not an Add");
    }
    
    // Check contents of our orset
    assert_eq!(true, orset.contains(&"dog".to_string()));
    assert_eq!(false, orset.contains(&"cat".to_string()));
    assert_eq!(false, orset.join(add2));

    let remove = orset.remove("dog".to_string());
    let remove2 = remove.clone().unwrap();

    // Save another copy for later
    let orset2 = orset.clone();

    // Test that we have a proper remove mutator
    if let Some(Delta::Remove {dots, ..}) = remove {
        assert_eq!(dots.len(), 1);
        assert_eq!(dots[0].actor, "node1".to_string());
        assert_eq!(dots[0].counter, 1);
    } else {
        panic!("Invalid Delta: Not a Remove");
    }

    // Check that removes are idempotent
    // We already removed this, and we try to join the resulting delta mutator from the state
    assert_eq!(false, orset.join(remove2));

    // Ensure that joining two states is the same as directly mutating the states
    assert_eq!(true, orset1.join_state(orset2));
    assert_eq!(orset, orset1);

    assert_eq!(0, orset.elements().len());

    let delta = Delta::Add {element: "dog".to_string(),
                            dot: Dot {actor: "node2".to_string(), counter: 1}};

    // Check that joining an Add mutator mutates the set
    assert_eq!(true, orset.join(delta));

    assert_eq!(1, orset.elements().len());

    let delta = Delta::Remove {element: "dog".to_string(),
                               dots: vec![Dot {actor: "node2".to_string(),
                                               counter: 1}]};

    // Check that joining a Remove mutator mutates the set
    assert_eq!(true, orset.join(delta));

    // Ensure that joining two states is the same as joining the deltas
    assert_eq!(true, orset1.join_state(orset.clone()));
    assert_eq!(orset, orset1);
}
