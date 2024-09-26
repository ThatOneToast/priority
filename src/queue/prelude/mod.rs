use std::{
    cmp::Ordering,
    time::{SystemTime, UNIX_EPOCH},
};


pub trait Queueable: Clone {}



/// QueueFlags are designed to be used behing the scenes
/// They hold more information on their spot in the queue.
#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub struct QueueFlag {
    /// Whether or not the item can be shifted
    pub locked: LockStatus,
    /// The priority of the item in which it will try to populate.
    pub priority: u32,
    /// This is an identifier hash.
    pub identifier: u32,
    /// time of which this item was added to the queue
    pub timestamp: u64,
}

impl QueueFlag {
    pub fn new(priority: u32, lock: LockStatus, id: u32) -> Self {
        let now = SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards");
        Self {
            priority,
            identifier: id,
            locked: lock,
            timestamp: now.as_secs(), // Set the timestamp
        }
    }
}

impl PartialOrd for QueueFlag {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for QueueFlag {
    fn cmp(&self, other: &Self) -> Ordering {
        // Compare by priority first
        match self.priority.cmp(&other.priority) {
            Ordering::Equal => {
                // Prioritize locked items before unlocked
                match (self.locked, other.locked) {
                    (LockStatus::Locked, LockStatus::Unlocked) => Ordering::Less,
                    (LockStatus::Unlocked, LockStatus::Locked) => Ordering::Greater,
                    _ => {
                        // Fallback to timestamp if both are locked/unlocked
                        match self.timestamp.cmp(&other.timestamp) {
                            Ordering::Equal => self.identifier.cmp(&other.identifier),
                            other => other,
                        }
                    }
                }
            }
            other => other,
        }
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub enum LockStatus {
    Unlocked,
    Locked,
}



#[derive(Clone, Debug)]
pub struct Prio<T>
where
    T: Queueable + Clone,
{
    pub item: T,
    priority: Option<u32>,
    pub locked: LockStatus,
}

impl<T> Queueable for Prio<T> where T: Queueable + Clone {}

impl<T> Prio<T>
where
    T: Queueable + Clone,
{
    pub fn new(item: T, priority: Option<u32>, lock: LockStatus) -> Self {
        Self {
            item,
            priority,
            locked: lock,
        }
    }

    /// Creates a new prio with a lock and a instant priority ( 0 )
    pub fn wlip(item: T) -> Self {
        Self {
            item,
            priority: Some(0),
            locked: LockStatus::Locked,
        }
    }

    /// Creates a new prio with no lock and a instant priority ( 0 )
    pub fn wolip(item: T) -> Self {
        Self {
            item,
            priority: Some(0),
            locked: LockStatus::Unlocked,
        }
    }

    /// Creates a new prio with a lock.
    pub fn wlock(item: T, priority: Option<u32>) -> Self {
        Self {
            item,
            priority,
            locked: LockStatus::Locked,
        }
    }

    /// Creates a new prio with no lock.
    pub fn wolock(item: T, priority: Option<u32>) -> Self {
        Self {
            item,
            priority,
            locked: LockStatus::Unlocked,
        }
    }

    pub fn priority(&self) -> Option<u32> {
        self.priority
    }
}
