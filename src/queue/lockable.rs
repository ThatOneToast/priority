use std::{collections::BTreeMap, sync::atomic::AtomicU32};

use super::prelude::{Prio, QueueFlag, Queueable};

pub struct LockableQueue<T>
where
    T: Queueable + Clone,
{
    map: BTreeMap<QueueFlag, T>,
    counter: AtomicU32,
}

impl<T> LockableQueue<T>
where
    T: Queueable + Clone,
{
    /// Creates a new LockableQueue.
    pub fn new() -> Self {
        Self {
            map: BTreeMap::new(),
            counter: AtomicU32::new(0),
        }
    }

    /// Pushes a prioritized item to the queue.
    pub fn push(&mut self, item: Prio<T>) {
        let priority = item.priority().expect("Priority must be set");
        let locked = item.locked;

        // Find the current highest identifier for items with the same priority
        let highest_identifier = self
            .map
            .keys()
            .filter(|flag| flag.priority == priority && flag.locked == locked)
            .map(|flag| flag.identifier)
            .max()
            .unwrap_or(0);

        // Use the highest identifier found (or 0 if none found) and increment it
        let new_identifier = highest_identifier + 1;

        // Create a new queue flag using the original priority, locked status, and the incremented identifier
        let new_queue_flag = QueueFlag::new(priority, locked, new_identifier);

        // Insert the new item with the updated queue flag
        self.map.insert(new_queue_flag, item.item);
    }

    /// Pops the next item in the queue.
    pub fn pop(&mut self) -> Option<T> {
        let lowest_flag = self.map.keys().next().cloned();

        if let Some(flag) = lowest_flag {
            let value = self.map.remove(&flag).unwrap();
            return Some(value);
        }
        None
    }

    /// Returns all elements in the queue as a Vec.
    pub fn get_elements(&self) -> Vec<T> {
        self.map.values().cloned().collect()
    }
}
