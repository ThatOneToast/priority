pub mod queue;

#[cfg(test)]
pub mod tests {

    pub mod prio_list {
        use crate::queue::list::PriorityList;

        #[test]
        fn new() {
            let mut list: PriorityList<String> = PriorityList::new();

            list.push("a".to_string(), None);
            list.push("c".to_string(), None);
            list.push("b".to_string(), Some(1));

            assert_eq!(vec!["a", "b", "c"], list.get_elements());

            list.push("b".to_string(), Some(2));
            assert_eq!(vec!["a", "b", "b", "c"], list.get_elements());
        }

        #[test]
        fn pop_next() {
            let mut list: PriorityList<String> = PriorityList::new();

            list.push("a".to_string(), 0.into());
            list.push("b".to_string(), 1.into());
            list.push("c".to_string(), 2.into());

            assert_eq!(Some("a".to_string()), list.pop_next());
            assert_eq!(Some("b".to_string()), list.pop_next());
            assert_eq!(Some("c".to_string()), list.pop_next());
        }
    }

    pub mod prio_map {
        use crate::queue::map::PriorityMap;

        #[test]
        fn new() {
            let mut map: PriorityMap<String, u8> = PriorityMap::new();

            map.push("first".into(), 1, None);
            map.push("zero".into(), 0, Some(0));

            assert_eq!(
                vec![(&"zero".into(), &0), (&"first".into(), &1)],
                map.get_elements()
            );
        }

        #[test]
        fn pop_next() {
            let mut map: PriorityMap<String, u8> = PriorityMap::new();

            map.push("first".into(), 1, None);
            map.push("zero".into(), 0, Some(0));

            assert_eq!(Some(("zero".to_string(), 0)), map.pop());
            assert_eq!(Some(("first".to_string(), 1)), map.pop());
        }
    }

    pub mod queue {
        use crate::queue::{prelude::{LockStatus, Prio}, Queue};

        #[test]
        fn new() {
            let mut queue: Queue<String> = Queue::new();

            queue.push("first".to_string());
            queue.push("third".to_string());
            queue.push("second".to_string());
            assert_eq!(vec!["first", "third", "second"], queue.get_elements());
        }

        #[test]
        fn pop() {
            let mut queue: Queue<String> = Queue::new();

            queue.push("first".to_string());
            queue.push("third".to_string());
            queue.push("second".to_string());

            assert_eq!(Some("first".into()), queue.pop());
            assert_eq!(Some("third".into()), queue.pop());
            assert_eq!(Some("second".into()), queue.pop());
            assert_eq!(None, queue.pop());
        }

        #[test]
        fn insert_with_priority() {
            let mut queue: Queue<String> = Queue::new();

            queue.push("first".to_string());
            queue.push("second".to_string());
            queue.push_prio(Prio::new("third".to_string(), 0.into(), LockStatus::Unlocked));

            assert_eq!(vec!["third", "first", "second"], queue.get_elements());
        }
    }
    
    pub mod lockable_queue {
        use crate::queue::{lockable::LockableQueue, prelude::{LockStatus, Prio}};
    
    
        #[test]
        fn new() {
            let mut queue: LockableQueue<String> = LockableQueue::new();
    
            queue.push(Prio::new("first".to_string(), Some(1), LockStatus::Unlocked));
            queue.push(Prio::new("third".to_string(), Some(0), LockStatus::Locked));
            queue.push(Prio::new("second".to_string(), Some(2), LockStatus::Unlocked));
            
    
            
            assert_eq!(vec!["third", "first", "second"], queue.get_elements().iter().map(|item| item.clone()).collect::<Vec<_>>());
            
            queue.push(Prio::wolock("fourth".to_string(), Some(0)));

            assert_eq!(vec!["third", "fourth", "first", "second"], queue.get_elements().iter().map(|item| item.clone()).collect::<Vec<_>>());
            
        }
        
        #[test]
        fn test_lockable_queue() {
            let mut queue = LockableQueue::new();
            
            queue.push(Prio::new("first", Some(2), LockStatus::Unlocked)); // Priority 2
            queue.push(Prio::new("second", Some(1), LockStatus::Locked));  // Priority 1 (locked)
            queue.push(Prio::new("third", Some(1), LockStatus::Unlocked)); // Priority 1 (unlocked)
            queue.push(Prio::new("fourth", Some(0), LockStatus::Locked));  // Highest priority (locked)
        
            let elements = queue.get_elements();
            
            // Expected order: ["fourth", "second", "third", "first"]
            assert_eq!(vec!["fourth", "second", "third", "first"], elements);
            
            assert_eq!("fourth".to_string(), queue.pop().unwrap());
            
            assert_eq!(vec!["second", "third", "first"], queue.get_elements());
            
            queue.push(Prio::new("fifth", Some(0), LockStatus::Locked));
            
            assert_eq!(vec!["fifth", "second", "third", "first"], queue.get_elements());
            
            queue.push(Prio::new("sixth", Some(0), LockStatus::Locked));
            
            assert_eq!(vec!["fifth", "sixth", "second", "third", "first"], queue.get_elements());
            
            queue.push(Prio::new("seventh", Some(0), LockStatus::Locked));
            
            assert_eq!(vec!["fifth", "sixth", "seventh", "second", "third", "first"], queue.get_elements());
            
        }
        
        #[test]
        fn test_unlocked_lockable_queue() {
            let mut queue = LockableQueue::new();
            
            queue.push(Prio::new("first", Some(2), LockStatus::Unlocked)); // Priority 2
            queue.push(Prio::new("second", Some(1), LockStatus::Unlocked));  // Priority 1 (unlocked)
            queue.push(Prio::new("third", Some(1), LockStatus::Unlocked)); // Priority 1 (unlocked)
            queue.push(Prio::new("fourth", Some(0), LockStatus::Unlocked));  // Highest priority (unlocked)
            
            assert_eq!(vec!["fourth", "second", "third", "first"], queue.get_elements().iter().map(|item| item.clone()).collect::<Vec<_>>());
            
            queue.push(Prio::new("fifth", Some(0), LockStatus::Unlocked));
            
            assert_eq!(vec!["fourth", "fifth", "second", "third", "first"], queue.get_elements().iter().map(|item| item.clone()).collect::<Vec<_>>());
            
            queue.push(Prio::new("sixth", Some(0), LockStatus::Unlocked));
            
            assert_eq!(vec!["fourth", "fifth", "sixth", "second", "third", "first"], queue.get_elements().iter().map(|item| item.clone()).collect::<Vec<_>>());
            
            queue.push(Prio::new("seventh", Some(0), LockStatus::Unlocked));
            
            assert_eq!(vec!["fourth", "fifth", "sixth", "seventh", "second", "third", "first"], queue.get_elements().iter().map(|item| item.clone()).collect::<Vec<_>>());
            
        }
        
    
        
    }


    pub mod queue_tuples {
        use crate::queue::Queue;


        #[test]
        fn test_tuple_of_2() {
            let mut queue: Queue<(u8, String)> = Queue::new();

            queue.push((1, "first".to_string()));
            queue.push((2, "second".to_string()));

            let elements = queue.get_elements();
            assert_eq!(
                elements,
                vec![(1, "first".to_string()), (2, "second".to_string())]
            );
        }

        #[test]
        fn test_tuple_of_6() {
            let mut queue: Queue<(u8, u8, u8, u8, u8, u8)> = Queue::new();

            queue.push((1, 2, 3, 4, 5, 6));
            queue.push((7, 8, 9, 10, 11, 12));

            let elements = queue.get_elements();
            assert_eq!(elements, vec![(1, 2, 3, 4, 5, 6), (7, 8, 9, 10, 11, 12)]);
        }

        #[test]
        fn test_tuple_of_8() {
            let mut queue: Queue<(u8, u8, u8, u8, u8, u8, u8, u8)> = Queue::new();

            queue.push((1, 2, 3, 4, 5, 6, 7, 8));
            queue.push((9, 10, 11, 12, 13, 14, 15, 16));

            let elements = queue.get_elements();
            assert_eq!(
                elements,
                vec![(1, 2, 3, 4, 5, 6, 7, 8), (9, 10, 11, 12, 13, 14, 15, 16)]
            );
        }

        #[test]
        fn test_tuple_of_12() {
            let mut queue: Queue<(u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8)> = Queue::new();

            queue.push((1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12));
            queue.push((13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24));

            let elements = queue.get_elements();
            assert_eq!(
                elements,
                vec![
                    (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12),
                    (13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24)
                ]
            );
        }
    }

    pub mod queue_linked_list {
        use std::collections::LinkedList;

        use crate::queue::Queue;

        #[test]
        fn test_linked_list() {
            let mut queue: Queue<LinkedList<u8>> = Queue::new();

            // Create two linked lists
            let mut list1: LinkedList<u8> = LinkedList::new();
            list1.push_back(1);
            list1.push_back(2);
            list1.push_back(3);

            let mut list2: LinkedList<u8> = LinkedList::new();
            list2.push_back(4);
            list2.push_back(5);
            list2.push_back(6);

            // Push both linked lists into the queue
            queue.push(list1.clone());
            queue.push(list2.clone());

            // Get the elements from the queue
            let elements = queue.get_elements();

            // Check if the elements are correct
            let mut expected_list1 = LinkedList::new();
            expected_list1.push_back(1);
            expected_list1.push_back(2);
            expected_list1.push_back(3);

            let mut expected_list2 = LinkedList::new();
            expected_list2.push_back(4);
            expected_list2.push_back(5);
            expected_list2.push_back(6);

            assert_eq!(elements, vec![expected_list1, expected_list2]);
        }
    }
    
    pub mod queue_btree_map {
        use std::collections::BTreeMap;

        use crate::queue::Queue;

        
        #[test]
        fn test_btree_map() {
            let mut queue: Queue<BTreeMap<u8, u8>> = Queue::new();
    
            // Create two BTreeMaps
            let mut map1: BTreeMap<u8, u8> = BTreeMap::new();
            map1.insert(1, 10);
            map1.insert(2, 20);
            map1.insert(3, 30);
    
            let mut map2: BTreeMap<u8, u8> = BTreeMap::new();
            map2.insert(4, 40);
            map2.insert(5, 50);
            map2.insert(6, 60);
    
            // Push both BTreeMaps into the queue
            queue.push(map1.clone());
            queue.push(map2.clone());
    
            // Get the elements from the queue
            let elements = queue.get_elements();
    
            // Create expected maps to compare with
            let mut expected_map1 = BTreeMap::new();
            expected_map1.insert(1, 10);
            expected_map1.insert(2, 20);
            expected_map1.insert(3, 30);
    
            let mut expected_map2 = BTreeMap::new();
            expected_map2.insert(4, 40);
            expected_map2.insert(5, 50);
            expected_map2.insert(6, 60);
    
            // Ensure the queue contains both maps
            assert_eq!(elements, vec![expected_map1, expected_map2]);
        }
    }
}
