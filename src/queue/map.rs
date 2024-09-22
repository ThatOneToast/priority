use std::collections::BTreeMap;

pub struct PriorityMap<K, V> {
    map: BTreeMap<u32, (K, V)>,
}

impl<K, V> PriorityMap<K, V>
where
    K: Clone,
    V: Clone,
{
    pub fn new() -> Self {
        Self {
            map: BTreeMap::new(),
        }
    }

    /// Inserts a key-value pair with an optional priority.
    pub fn push(&mut self, key: K, value: V, priority: Option<u32>) {
        match priority {
            Some(new_priority) => {
                // Clone the current map to make adjustments
                let mut new_map = self.map.clone();

                // Shift existing items down if their priority is greater than or equal to the new priority
                for (slot, _) in self.map.iter() {
                    if *slot >= new_priority {
                        // Increment key in new_map
                        let current_value = new_map.remove(slot).unwrap();
                        new_map.insert(slot + 1, current_value);
                    }
                }

                // Insert the new item at its specified priority
                new_map.insert(new_priority, (key, value));
                self.map = new_map; // Replace the original map
            }
            None => {
                let slot = self.map.len() as u32;
                self.map.insert(slot, (key, value));
            }
        }
    }

    pub fn get_elements(&self) -> Vec<(&K, &V)> {
        self.map.iter().map(|(_, (k, v))| (k, v)).collect()
    }

    /// Returns and removes the next element in the queue.
    pub fn pop(&mut self) -> Option<(K, V)> {
        // Find the lowest priority key
        let lowest_key = self.map.keys().next().cloned();

        // If a key exists, remove and return the corresponding value
        if let Some(key) = lowest_key {
            let value = self.map.remove(&key).unwrap();
            // Optionally, you can also shift the remaining items down if needed
            // but this is not strictly necessary for a simple pop function
            return Some(value);
        }
        None
    }
}
