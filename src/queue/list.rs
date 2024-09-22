use std::collections::BTreeMap;

/// A PriorityList is a list of given elements ordered by a priority.
/// Priorities start at 0 and can go up to 255 (unless specified otherwise).
///
/// No element can have the same priority.
/// An element will be overwritten if a new element with the same priority is added.
pub struct PriorityList<T> {
    elements: BTreeMap<u32, T>, // (priority, element)
}

impl<T> PriorityList<T>
where
    T: Clone,
{
    // Create a new PriorityList
    pub fn new() -> Self {
        PriorityList {
            elements: BTreeMap::new(),
        }
    }

    /// Pushes an element with a specific priority, shifting existing elements down if necessary.
    pub fn push(&mut self, element: T, priority: Option<u32>) {
        match priority {
            Some(new_priority) => {
                // Create a new map to handle shifting
                let mut new_elements = BTreeMap::new();

                // Shift existing items down if their priority is greater than or equal to the new priority
                for (existing_priority, existing_item) in self.elements.iter() {
                    if *existing_priority >= new_priority {
                        new_elements.insert(existing_priority + 1, existing_item.clone());
                    } else {
                        new_elements.insert(*existing_priority, existing_item.clone());
                    }
                }

                // Insert the new item at its specified priority
                new_elements.insert(new_priority, element);
                self.elements = new_elements; // Replace the original map
            }
            None => {
                // Find the next available priority
                let next_priority = self.elements.len() as u32;
                self.elements.insert(next_priority, element);
            }
        }
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
