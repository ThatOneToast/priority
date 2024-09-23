use prelude::Prio;

use crate::queue::prelude::Queueable;
use std::collections::BTreeMap;

pub mod list;
pub mod map;
pub mod prelude;
pub mod lockable;

pub struct Queue<T>
where
    T: Queueable + Clone,
{
    map: BTreeMap<u32, T>,
    pub next_priority: u32,
}

impl<T> Queue<T>
where
    T: Queueable + Clone,
{
    /// Creates a new Queue.
    pub fn new() -> Self {
        Self {
            map: BTreeMap::new(),
            next_priority: 0,
        }
    }

    /// Removes the highest priority item from the queue
    pub fn pop(&mut self) -> Option<T> {
        // Search for the lowest non-empty priority key
        let mut keys_to_remove: Vec<u32> = self.map.keys().cloned().collect();
        keys_to_remove.sort();

        for key in keys_to_remove {
            // If there is an item at the current key, remove it and return
            if let Some(item) = self.map.remove(&key) {
                return Some(item);
            }
        }

        // If no item is found, return None
        None
    }

    /// Pushes a prioritized item to the queue and shifts items at and after the given priority back by one.
    pub fn push_prio(&mut self, item: Prio<T>) {
        if let Some(priority) = item.priority() {
            // Step 1: Collect keys that need to be shifted (in reverse order)
            let keys_to_shift: Vec<u32> = self
                .map
                .keys()
                .filter(|&&key| key >= priority)
                .cloned()
                .collect();

            // Step 2: Shift each of those keys back by 1 (in reverse order to avoid overwriting)
            for key in keys_to_shift.into_iter().rev() {
                if let Some(existing_item) = self.map.remove(&key) {
                    self.map.insert(key + 1, existing_item);
                }
            }

            // Step 3: Insert the new item at its specified priority
            self.map.insert(priority, item.item);
        }
    }

    /// Pushes a non-prioritized item to the queue.
    pub fn push(&mut self, item: T) {
        self.map.insert(self.next_priority, item);
        self.next_priority += 1; // Increment for the next non-prioritized item
    }

    /// Shifts items in the queue to fill gaps.
    pub fn shift(&mut self) {
        let mut new_map: BTreeMap<u32, T> = BTreeMap::new();
        let mut index = 0;

        // Collect items in order
        for (_, value) in self.map.iter() {
            new_map.insert(index, value.clone());
            index += 1;
        }

        // Update the original map
        self.map = new_map;
    }

    /// Returns all elements in the queue as a Vec.
    pub fn get_elements(&self) -> Vec<T> {
        // Collect values based on their keys
        self.map.values().cloned().collect()
    }
}
