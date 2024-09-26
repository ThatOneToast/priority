pub mod queue;
pub mod prelude;



#[cfg(test)]
pub mod tests {
    use crate::queue::{prelude::Prio, sized::SizedQueue};

        

    pub mod lockable_queue {
        use crate::queue::{prelude::{LockStatus, Prio}, Queue};

        

        #[test]
        fn new() {
            let mut queue: Queue<String> = Queue::new();

            queue.push(Prio::new(
                "first".to_string(),
                Some(1),
                LockStatus::Unlocked,
            ));
            queue.push(Prio::new("third".to_string(), Some(0), LockStatus::Locked));
            queue.push(Prio::new(
                "second".to_string(),
                Some(2),
                LockStatus::Unlocked,
            ));

            assert_eq!(
                vec!["third", "first", "second"],
                queue
                    .get_elements()
                    .iter()
                    .map(|item| item.clone())
                    .collect::<Vec<_>>()
            );

            queue.push(Prio::wolock("fourth".to_string(), Some(0)));

            assert_eq!(
                vec!["third", "fourth", "first", "second"],
                queue
                    .get_elements()
                    .iter()
                    .map(|item| item.clone())
                    .collect::<Vec<_>>()
            );
        }

        #[test]
        fn test_lockable_queue() {
            let mut queue = Queue::new();

            queue.push(Prio::new("first", Some(2), LockStatus::Unlocked)); // Priority 2
            queue.push(Prio::new("second", Some(1), LockStatus::Locked)); // Priority 1 (locked)
            queue.push(Prio::new("third", Some(1), LockStatus::Unlocked)); // Priority 1 (unlocked)
            queue.push(Prio::new("fourth", Some(0), LockStatus::Locked)); // Highest priority (locked)

            let elements = queue.get_elements();

            // Expected order: ["fourth", "second", "third", "first"]
            assert_eq!(vec!["fourth", "second", "third", "first"], elements);

            assert_eq!("fourth".to_string(), queue.pop().unwrap());

            assert_eq!(vec!["second", "third", "first"], queue.get_elements());

            queue.push(Prio::new("fifth", Some(0), LockStatus::Locked));

            assert_eq!(
                vec!["fifth", "second", "third", "first"],
                queue.get_elements()
            );

            queue.push(Prio::new("sixth", Some(0), LockStatus::Locked));

            assert_eq!(
                vec!["fifth", "sixth", "second", "third", "first"],
                queue.get_elements()
            );

            queue.push(Prio::new("seventh", Some(0), LockStatus::Locked));

            assert_eq!(
                vec!["fifth", "sixth", "seventh", "second", "third", "first"],
                queue.get_elements()
            );
        }

        #[test]
        fn test_unlocked_lockable_queue() {
            let mut queue = Queue::new();

            queue.push(Prio::new("first", Some(2), LockStatus::Unlocked)); // Priority 2
            queue.push(Prio::new("second", Some(1), LockStatus::Unlocked)); // Priority 1 (unlocked)
            queue.push(Prio::new("third", Some(1), LockStatus::Unlocked)); // Priority 1 (unlocked)
            queue.push(Prio::new("fourth", Some(0), LockStatus::Unlocked)); // Highest priority (unlocked)

            assert_eq!(
                vec!["fourth", "second", "third", "first"],
                queue
                    .get_elements()
            );

            queue.push(Prio::new("fifth", Some(0), LockStatus::Unlocked));

            assert_eq!(
                vec!["fourth", "fifth", "second", "third", "first"],
                queue
                    .get_elements()
            );

            queue.push(Prio::new("sixth", Some(0), LockStatus::Unlocked));

            assert_eq!(
                vec!["fourth", "fifth", "sixth", "second", "third", "first"],
                queue
                    .get_elements()
            );

            queue.push(Prio::new("seventh", Some(0), LockStatus::Unlocked));

            assert_eq!(
                vec!["fourth", "fifth", "sixth", "seventh", "second", "third", "first"],
                queue
                    .get_elements()
            );
        }
    }
    
    #[test]
    fn test_percentage_full() {
        let mut queue = SizedQueue::<String>::new(10);
        
        queue.push(Prio::wolock("first".to_string(), Some(0))).unwrap();
        queue.push(Prio::wolock("second".to_string(), Some(0))).unwrap();
        queue.push(Prio::wolock("third".to_string(), Some(0))).unwrap();
        queue.push(Prio::wolock("fourth".to_string(), Some(0))).unwrap();
        queue.push(Prio::wolock("fifth".to_string(), Some(0))).unwrap();
        queue.push(Prio::wolock("sixth".to_string(), Some(0))).unwrap();
        
        assert_eq!(queue.percentage_full().round(), 60.0);
    }

    pub mod queue_tuples {
        use crate::queue::{Queue, prelude::Prio};


        #[test]
        fn test_tuple_of_2() {
            let mut queue: Queue<(u8, String)> = Queue::new();

            queue.push(Prio::wolock((1, "first".to_string()), Some(0)));
            queue.push(Prio::wolock((2, "second".to_string()), Some(0)));

            let elements = queue.get_elements();
            assert_eq!(
                elements,
                vec![(1, "first".to_string()), (2, "second".to_string())]
            );
        }

        #[test]
        fn test_tuple_of_6() {
            let mut queue: Queue<(u8, u8, u8, u8, u8, u8)> = Queue::new();

            queue.push(Prio::wolock((1, 2, 3, 4, 5, 6), Some(0)));
            queue.push(Prio::wolock((7, 8, 9, 10, 11, 12), Some(0)));

            let elements = queue.get_elements();
            assert_eq!(elements, vec![(1, 2, 3, 4, 5, 6), (7, 8, 9, 10, 11, 12)]);
        }

        #[test]
        fn test_tuple_of_8() {
            let mut queue: Queue<(u8, u8, u8, u8, u8, u8, u8, u8)> = Queue::new();

            queue.push(Prio::wolock((1, 2, 3, 4, 5, 6, 7, 8), Some(0)));
            queue.push(Prio::wolock((9, 10, 11, 12, 13, 14, 15, 16), Some(0)));

            let elements = queue.get_elements();
            assert_eq!(
                elements,
                vec![(1, 2, 3, 4, 5, 6, 7, 8), (9, 10, 11, 12, 13, 14, 15, 16)]
            );
        }

        #[test]
        fn test_tuple_of_12() {
            let mut queue: Queue<(u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8)> = Queue::new();

            queue.push(Prio::wolock((1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12), Some(0)));
            queue.push(Prio::wolock((13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24), Some(0)));

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

        use crate::queue::{Queue, prelude::Prio};


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
            queue.push(Prio::wolock(list1.clone(), Some(0)));
            queue.push(Prio::wolock(list2.clone(), Some(0)));

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

        use crate::queue::{Queue, prelude::Prio};


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
            queue.push(Prio::wolock(map1.clone(), Some(0)));
            queue.push(Prio::wolock(map2.clone(), Some(0)));

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
