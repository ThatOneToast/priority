use std::{
    cell::{Cell, RefCell},
    cmp::Ordering,
    collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, LinkedList, VecDeque},
    rc::Rc,
    sync::Arc,
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};

use uuid::Uuid;

impl Queueable for &str {}
impl Queueable for String {}

impl Queueable for u8 {}
impl Queueable for u16 {}
impl Queueable for u32 {}
impl Queueable for u64 {}
impl Queueable for u128 {}

impl Queueable for f32 {}
impl Queueable for f64 {}

impl Queueable for i8 {}
impl Queueable for i16 {}
impl Queueable for i32 {}
impl Queueable for i64 {}
impl Queueable for i128 {}

impl Queueable for bool {}
impl Queueable for char {}

impl<T> Queueable for Option<T> where T: Queueable + Clone {}
impl<T> Queueable for Result<T, T> where T: Queueable + Clone {}

impl<T> Queueable for Vec<T> where T: Queueable + Clone {}
impl<T> Queueable for HashMap<T, T> where T: Queueable + Clone {}
impl<T> Queueable for BTreeMap<T, T> where T: Queueable + Clone {}
impl<T> Queueable for HashSet<T> where T: Queueable + Clone {}
impl<T> Queueable for BTreeSet<T> where T: Queueable + Clone {}
impl<T> Queueable for LinkedList<T> where T: Queueable + Clone {}
impl<T> Queueable for VecDeque<T> where T: Queueable + Clone {}
impl<T> Queueable for BinaryHeap<T> where T: Queueable + Clone {}

impl<T> Queueable for Box<T> where T: Queueable + Clone {}
impl<T> Queueable for Rc<T> where T: Queueable + Clone {}
impl<T> Queueable for Arc<T> where T: Queueable + Clone {}
impl<T> Queueable for Cell<T> where T: Queueable + Clone + std::marker::Copy {}
impl<T> Queueable for RefCell<T> where T: Queueable + Clone {}

impl Queueable for Duration {}
impl Queueable for Instant {}

impl<T1> Queueable for (T1,) where T1: Queueable + Clone {}

impl<T1, T2> Queueable for (T1, T2)
where
    T1: Queueable + Clone,
    T2: Queueable + Clone,
{
}

impl<T1, T2, T3> Queueable for (T1, T2, T3)
where
    T1: Queueable + Clone,
    T2: Queueable + Clone,
    T3: Queueable + Clone,
{
}

impl<T1, T2, T3, T4> Queueable for (T1, T2, T3, T4)
where
    T1: Queueable + Clone,
    T2: Queueable + Clone,
    T3: Queueable + Clone,
    T4: Queueable + Clone,
{
}

impl<T1, T2, T3, T4, T5> Queueable for (T1, T2, T3, T4, T5)
where
    T1: Queueable + Clone,
    T2: Queueable + Clone,
    T3: Queueable + Clone,
    T4: Queueable + Clone,
    T5: Queueable + Clone,
{
}

impl<T1, T2, T3, T4, T5, T6> Queueable for (T1, T2, T3, T4, T5, T6)
where
    T1: Queueable + Clone,
    T2: Queueable + Clone,
    T3: Queueable + Clone,
    T4: Queueable + Clone,
    T5: Queueable + Clone,
    T6: Queueable + Clone,
{
}

impl<T1, T2, T3, T4, T5, T6, T7> Queueable for (T1, T2, T3, T4, T5, T6, T7)
where
    T1: Queueable + Clone,
    T2: Queueable + Clone,
    T3: Queueable + Clone,
    T4: Queueable + Clone,
    T5: Queueable + Clone,
    T6: Queueable + Clone,
    T7: Queueable + Clone,
{
}

impl<T1, T2, T3, T4, T5, T6, T7, T8> Queueable for (T1, T2, T3, T4, T5, T6, T7, T8)
where
    T1: Queueable + Clone,
    T2: Queueable + Clone,
    T3: Queueable + Clone,
    T4: Queueable + Clone,
    T5: Queueable + Clone,
    T6: Queueable + Clone,
    T7: Queueable + Clone,
    T8: Queueable + Clone,
{
}

impl<T1, T2, T3, T4, T5, T6, T7, T8, T9> Queueable for (T1, T2, T3, T4, T5, T6, T7, T8, T9)
where
    T1: Queueable + Clone,
    T2: Queueable + Clone,
    T3: Queueable + Clone,
    T4: Queueable + Clone,
    T5: Queueable + Clone,
    T6: Queueable + Clone,
    T7: Queueable + Clone,
    T8: Queueable + Clone,
    T9: Queueable + Clone,
{
}

impl<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> Queueable
    for (T1, T2, T3, T4, T5, T6, T7, T8, T9, T10)
where
    T1: Queueable + Clone,
    T2: Queueable + Clone,
    T3: Queueable + Clone,
    T4: Queueable + Clone,
    T5: Queueable + Clone,
    T6: Queueable + Clone,
    T7: Queueable + Clone,
    T8: Queueable + Clone,
    T9: Queueable + Clone,
    T10: Queueable + Clone,
{
}

impl<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> Queueable
    for (T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11)
where
    T1: Queueable + Clone,
    T2: Queueable + Clone,
    T3: Queueable + Clone,
    T4: Queueable + Clone,
    T5: Queueable + Clone,
    T6: Queueable + Clone,
    T7: Queueable + Clone,
    T8: Queueable + Clone,
    T9: Queueable + Clone,
    T10: Queueable + Clone,
    T11: Queueable + Clone,
{
}

impl<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> Queueable
    for (T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12)
where
    T1: Queueable + Clone,
    T2: Queueable + Clone,
    T3: Queueable + Clone,
    T4: Queueable + Clone,
    T5: Queueable + Clone,
    T6: Queueable + Clone,
    T7: Queueable + Clone,
    T8: Queueable + Clone,
    T9: Queueable + Clone,
    T10: Queueable + Clone,
    T11: Queueable + Clone,
    T12: Queueable + Clone,
{
}

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

#[derive(Clone)]
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
