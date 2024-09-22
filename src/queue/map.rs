use std::collections::BTreeMap;

pub struct PriorityMap<K, V> {
    map: BTreeMap<u32, (K, V)>,
    min_priority: u32,
    max_priority: u32,
}

impl<K: Ord, V> PriorityMap<K, V> {
    // Create a new PriorityMap with a default priority range (0-255)
    pub fn new() -> Self {
        PriorityMap {
            map: BTreeMap::new(),
            min_priority: 0,
            max_priority: 255,
        }
    }

    // Create a new PriorityMap with a custom priority range
    pub fn with_priority_range(min_priority: u32, max_priority: u32) -> Self {
        PriorityMap {
            map: BTreeMap::new(),
            min_priority,
            max_priority,
        }
    }
    
    /// Insert a new element with a specific priority
    /// If an element with the same priority already exists, it will be overwritten
    /// This is an unchecked operation, and will panic if the priority is out of range
    pub fn insert_unchecked(&mut self, key: K, value: V, priority: u32) {
        if priority < self.min_priority || priority > self.max_priority {
            panic!("Priority {} is out of range ({}-{})", priority, self.min_priority, self.max_priority);
        }
        
        self.map.insert(priority, (key, value));
    }
    
    /// Insert a new element with a specific priority
    /// If an element with the same priority already exists, the function will ignore insertion
    /// Returns true if the element was inserted, false if the element was not inserted
    /// False: The element was not inserted because the priority was out of range, or taken.
    /// True: The element was inserted because the priority was available.
    pub fn insert_checked(&mut self, key: K, value: V, priority: u32) -> bool {
        if priority < self.min_priority || priority > self.max_priority {
            return false
        }
        if self.map.contains_key(&priority) {
            false
        } else {
            self.map.insert(priority, (key, value));
            true
        }
    }
    
    /// Inserts the element at the next available priority
    /// Returns the priority of the inserted element
    pub fn insert_next(&mut self, key: K, value: V) -> u32 {
        // Find the first available priority
        let next_priority = self.find_next_available_priority();
        self.insert_unchecked(key, value, next_priority);
        next_priority
    }
    
    /// Helper function to find the next available priority
    pub fn find_next_available_priority(&self) -> u32 {
        let mut used_priorities: Vec<u32> = self.map.keys().cloned().collect();
        used_priorities.sort(); // Sort to find the first gap

        // Find the first gap in priorities within the specified range
        for priority in self.min_priority..=self.max_priority {
            if !used_priorities.contains(&priority) {
                return priority;
            }
        }

        panic!("No available priority in the range ({}-{})", self.min_priority, self.max_priority);
    }
    
    /// Returns all elements in priority order
    pub fn get_elements(&self) -> Vec<(&K, &V)> {
        self.map.iter().map(|(_, (key, value))| (key, value)).collect()
    }

    /// Retrieve the element with the specified priority
    pub fn get(&self, priority: u32) -> Option<&V> {
        self.map.get(&priority).map(|(_, value)| value)
    }
    
    /// Remove the element with the specified priority
    pub fn remove(&mut self, priority: u32) -> Option<V> {
        self.map.remove(&priority).map(|(_, value)| value)
    }
    
    /// Check if the map is empty
    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }
    
    /// Get the number of elements
    pub fn len(&self) -> usize {
        self.map.len()
    }
    
    /// Returns the total number of priorities
    pub fn priority_len(&self) -> usize {
        self.max_priority as usize + 1
    }
    
    /// Returns the priority range
    pub fn priority_range(&self) -> (u32, u32) {
        (self.min_priority, self.max_priority)
    }
    
    /// Pop the element with the lowest priority and return it
    pub fn pop_next(&mut self) -> Option<(K, V)> {
        // Get the first (lowest) priority in the BTreeMap
        let first_key = *self.map.keys().next()?;
        self.map.remove(&first_key)
    }
}
