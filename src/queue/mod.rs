pub mod prelude;
pub mod sized;

use std::{
    collections::BTreeMap,
    sync::{atomic::AtomicU32, LazyLock, Mutex, RwLock},
};

use prelude::{LockStatus, Prio, QueueFlag, Queueable};

/// This queue holds data in order of a Priority, Lock, Time order.
/// 
/// Pushing to a queue, your element have the `queueable` and `clone` traits.
/// Elements should be wrapped in a `Prio` struct, which has many helper methods for generating set priorities.
/// Using prio you can also set your own lock status, and priority numbders.
///
/// # Example Usage
/// ```
/// use priority::queue::{Queue, prelude::Prio};
///
/// let mut queue: Queue<String> = Queue::new();
///
/// queue.push(Prio::new("first".to_string(), Some(1), LockStatus::Unlocked));
/// queue.push(Prio::new("second".to_string(), Some(2), LockStatus::Unlocked));
/// queue.push(Prio::new("third".to_string(), Some(1), LockStatus::Locked));
///
/// assert_eq!(vec!["third", "first", "second"], queue.get_elements());
///
/// queue.push(Prio::wlip("fourth".to_string()));
///
/// assert_eq!(vec!["third", "fourth", "first", "second"], queue.get_elements());
/// ```
pub struct Queue<T>(BTreeMap<QueueFlag, T>, AtomicU32)
where
    T: Queueable + Clone;

impl<T> Queue<T>
where
    T: Queueable + Clone,
{
    /// Creates a new LockableQueue.
    pub fn new() -> Self {
        Self(BTreeMap::new(), AtomicU32::new(0))
    }

    /// Pushes a prioritized item to the queue.
    /// Returns a unique identifier that can be used to track and retrieve this element in the queue
    pub fn push(&mut self, item: Prio<T>) -> u32 {
        let priority = item.priority().expect("Priority must be set");
        let locked = item.locked;

        let new_identifier = self.1.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        let new_queue_flag = QueueFlag::new(priority, locked, new_identifier);

        self.0.insert(new_queue_flag, item.item);

        return new_identifier;
    }
    
    /// Pushes a non-prioritized item to the queue.
    /// No item pushed through this function will be locked
    pub fn push_non_prio(&mut self, item: T) -> u32 {
        let new_identifier = self.1.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        let new_queue_flag = QueueFlag::new(self.size() as u32 + 1, LockStatus::Unlocked, new_identifier);
        
        self.0.insert(new_queue_flag, item);
        
        return new_identifier;
        
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
    pub fn peek(&self) -> Option<(&T, u32)> {
        let lowest_flag = self.0.keys().next().cloned();
        let element = self.0.get(&lowest_flag.unwrap());
        let identifier = lowest_flag.unwrap().identifier;
        return element.map(|e| (e, identifier));
    }

    /// Returns all elements in the queue as a Vec.
    pub fn get_elements(&self) -> Vec<T> {
        self.0.values().cloned().collect()
    }

    /// Returns the size of the queue
    pub fn size(&self) -> usize {
        self.0.len()
    }

    /// Generates a new thread-safe LockableQueue
    pub const fn thread_safe_new() -> LazyLock<RwLock<Mutex<Queue<T>>>> {
        LazyLock::new(|| RwLock::new(Mutex::new(Queue::<T>::new()))) 
    }
}
