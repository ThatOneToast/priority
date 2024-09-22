use std::collections::BTreeMap;

/// A PriorityList is a list of given elements ordered by a priority.
/// Priorities start at 0 and can go up to 255 (unless specified otherwise).
///
/// No element can have the same priority.
/// An element will be overwritten if a new element with the same priority is added.
pub struct PriorityList<T> {
    elements: BTreeMap<u32, T>, // (priority, element)
    min_priority: u32,
    max_priority: u32,
}

impl<T> PriorityList<T> {
    // Create a new PriorityList with a default priority range (0-255)
    pub fn new() -> Self {
        PriorityList {
            elements: BTreeMap::new(),
            min_priority: 0,
            max_priority: 255,
        }
    }

    // Create a new PriorityList with a custom priority range
    pub fn with_priority_range(min_priority: u32, max_priority: u32) -> Self {
        PriorityList {
            elements: BTreeMap::new(),
            min_priority,
            max_priority,
        }
    }

    /// Append an element with a specific priority, overwriting any element with the same priority
    /// This is an unchecked operation, and will panic if the priority is out of range
    pub fn append_unchecked(&mut self, element: T, priority: u32) {
        if priority < self.min_priority || priority > self.max_priority {
            panic!(
                "Priority {} is out of range ({}-{})",
                priority, self.min_priority, self.max_priority
            );
        }
        self.elements.insert(priority, element); // Automatically replaces the element at this priority
    }
    
    /// Append an element to the next available priority
    /// returns true if the element was appended, false if the element was not appended
    /// False: The element was not appended because the priority was out of range, or taken.
    /// True: The element was appended because the priority was available.
    pub fn append_checked(&mut self, element: T, priority: u32) -> bool {
        if priority < self.min_priority || priority > self.max_priority {
            return false
        }
        if self.elements.contains_key(&priority) {
            false
        } else {
            self.elements.insert(priority, element);
            true
        }
    }

    /// Append an element to the next available priority
    /// Returns the priority of the appended element
    pub fn append_next(&mut self, element: T) -> u32 {
        let next_priority = self.find_next_available_priority();
        self.append_unchecked(element, next_priority);
        next_priority
    }

    /// Helper function to find the next available priority
    /// Will panic if no available priority is found
    fn find_next_available_priority(&self) -> u32 {
        for priority in self.min_priority..=self.max_priority {
            if !self.elements.contains_key(&priority) {
                return priority;
            }
        }
        panic!(
            "No available priority in the range ({}-{})",
            self.min_priority, self.max_priority
        );
    }

    /// Retrieve all elements in priority order
    pub fn get_elements(&self) -> Vec<&T> {
        self.elements.values().collect()
    }

    /// Check if the list is empty
    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }

    /// Get the number of elements
    pub fn len(&self) -> usize {
        self.elements.len()
    }

    /// Get the total number of priorities available
    pub fn priority_len(&self) -> usize {
        self.max_priority as usize - self.min_priority as usize + 1
    }

    /// Get the priority range
    pub fn priority_range(&self) -> (u32, u32) {
        (self.min_priority, self.max_priority)
    }

    /// Retrieve the element at the specified priority
    pub fn get_element_at_priority(&self, priority: u32) -> Option<&T> {
        self.elements.get(&priority)
    }

    /// Remove the element at the specified priority
    pub fn remove_element_at_priority(&mut self, priority: u32) -> Option<T> {
        self.elements.remove(&priority)
    }

    /// Pop the next element in priority order (lowest priority) and remove it
    pub fn pop_next(&mut self) -> Option<T> {
        let first_key = *self.elements.keys().next()?;
        self.elements.remove(&first_key)
    }

    /// Pop the last element in priority order (highest priority) and remove it
    pub fn pop_last(&mut self) -> Option<T> {
        let last_key = *self.elements.keys().next_back()?;
        self.elements.remove(&last_key)
    }
}
