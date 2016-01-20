use std::hash::Hash;
use std::collections::HashMap;
use timer_wheel::TimerWheel;

pub struct Requests<K, V> where K: Eq+Hash+Clone {
    requests: HashMap<K, (usize, V)>,
    timer_wheel: TimerWheel<K>
}

impl<K, V> Requests<K, V> where K: Eq+Hash+Clone {
    pub fn new(tick_time_ms: u64, timeout_ms: u64) -> Requests<K, V> {
        Requests {
            requests: HashMap::new(),
            timer_wheel: TimerWheel::new((timeout_ms / tick_time_ms) as usize)
        }
    }

    // There shouldn't be more than one outstanding request per client, as clients block waiting for
    // responses. Note that the timerwheel strategy will need to change if that ever becomes untrue.
    pub fn insert(&mut self, key: K, val: V) {
        let slot = self.timer_wheel.insert(key.clone());
        self.requests.insert(key.clone(), (slot, val));
    }

    pub fn remove(&mut self, key: &K) -> Option<V> {
        if let Some((slot, val)) = self.requests.remove(key) {
            self.timer_wheel.remove(key, slot);
            Some(val)
        } else {
            None
        }
    }

    // Expire each request at the current reading wheel slot. Run the callback function with each
    // key and value if the there is an outstanding request. The callback should perform side
    // effects for each request, such as sending a timeout response to the client and tracking stats.
    pub fn expire<F>(&mut self, mut callback: F) where F : FnMut(K, V) {
        let set = self.timer_wheel.expire();
        let mut iter = set.iter();
        while let Some(key) = iter.next() {
            println!("expire yo!!!");
            if let Some((_, val)) = self.requests.remove(&key) {
                callback((*key).clone(), val);
            }
        }
    }
}
