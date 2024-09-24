use std::{
    collections::BTreeMap,
    sync::atomic::AtomicU32,
};

use super::prelude::{Prio, QueueFlag, Queueable};


/// A SizedQueue is a queue that has a maximum element limit.
/// If the queue is full, new elements added will be rejected, and an error will be returned.
///
/// You can expand this queue by calling the `expand` method.
/// Once a queue is expanded, it cannot be shrunk back down.
///
/// # Similiarities with `Queue`
/// #
///
/// The same mechanics as Queue are applied here to the Size version.
///
/// Locks and priority rules are are still applied here, just a little head cap.
pub struct SizedQueue<T>(BTreeMap<QueueFlag, T>, AtomicU32, u32)
where
    T: Queueable + Clone;

impl<T> SizedQueue<T>
where
    T: Queueable + Clone,
{
    /// Creates a new LockableQueue.
    pub fn new(size: u32) -> Self {
        Self(BTreeMap::new(), AtomicU32::new(0), size)
    }

    /// Pushes a prioritized item to the queue.
    /// Returns a unique identifier that can be used to track and retrieve this element in the queue
    pub fn push(&mut self, item: Prio<T>) -> Result<u32, String> {
        if self.0.len() as u32 >= self.2 {
            return Err("Queue is full".to_string());
        }

        let priority = item.priority().expect("Priority must be set");
        let locked = item.locked;

        let new_identifier = self.1.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        let new_queue_flag = QueueFlag::new(priority, locked, new_identifier);

        self.0.insert(new_queue_flag, item.item);

        return Ok(new_identifier);
    }

    // Naive linear search through the entire queue
    pub fn retrieve_by_id_linear(&self, id: u32) -> Result<T, String> {
        for flag in self.0.keys() {
            if flag.identifier == id {
                return Ok(self.0.get(flag).cloned().unwrap());
            }
        }
        Err(format!("ID {} not found in the queue", id))
    }

    pub fn retrieve_by_id(&self, id: u32) -> Result<T, String> {
        // Check if the queue is empty
        if self.0.is_empty() {
            return Err("Queue is empty".to_string());
        }

        // Get the smallest and largest IDs in the queue
        let first_flag = self.0.keys().next().unwrap(); // Get the first (smallest) QueueFlag
        let last_flag = self.0.keys().next_back().unwrap(); // Get the last (largest) QueueFlag

        let first_id = first_flag.identifier;
        let last_id = last_flag.identifier;

        // Check if the id is out of bounds
        if id < first_id || id > last_id {
            return Err(format!(
                "ID {} is not in the range of current queue ({} to {})",
                id, first_id, last_id
            ));
        }

        // Determine whether to start from the front or the back based on proximity
        if id - first_id <= last_id - id {
            // Start from the beginning and search forwards
            for flag in self.0.keys() {
                if flag.identifier == id {
                    return Ok(self.0.get(flag).cloned().unwrap());
                }
            }
        } else {
            // Start from the end and search backwards
            for flag in self.0.keys().rev() {
                if flag.identifier == id {
                    return Ok(self.0.get(flag).cloned().unwrap());
                }
            }
        }

        Err(format!("ID {} not found in the queue", id))
    }

    /// Pops the next item in the queue.
    pub fn pop(&mut self) -> Option<T> {
        let lowest_flag = self.0.keys().next().cloned();

        if let Some(flag) = lowest_flag {
            let value = self.0.remove(&flag).unwrap();
            return Some(value);
        }
        None
    }
    
    /// Peeks the next item in the queue.
    /// Returns a reference to the next item in the queue.
    pub fn peek(&self) -> Option<&T> {
        let lowest_flag = self.0.keys().next().cloned();
        return self.0.get(&lowest_flag.unwrap());
    }

    /// Returns all elements in the queue as a Vec.
    pub fn get_elements(&self) -> Vec<T> {
        self.0.values().cloned().collect()
    }

    /// Returns the size of the queue
    pub fn size(&self) -> usize {
        self.0.len()
    }

    /// Returns the maximum size of the queue
    pub fn max_size(&self) -> u32 {
        self.2
    }
    
    /// Increases the maximum size of the queue
    ///
    /// Once a queue is expanded, it cannot be shrunk back down.
    pub fn expand(&mut self, size: usize) {
        self.2 += size as u32;
    }
    
    /// Returns a percentage of the queue's size
    pub fn percentage_full(&self) -> f32 {
        (self.0.len() as f32 / self.2 as f32) * 100.0
    }


}
