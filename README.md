# Welcome

Priority adds 3 new major types onto your battle belt.

## Queues

Queues can consist of any type that implements the `Queueable` trait.

Example:
```rust

let mut queue: Queue<String> = Queue::new();

queue.push("first".to_string());
queue.push("second".to_string());
queue.push_prio(Prioritized::new("third".to_string(), 0.into()));

```

pushing with prio will push other elements down the line so your element can take it's place.


