use std::collections::HashMap;
struct Logger {
    last_logged: HashMap<String, i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Logger {
    /** Initialize your data structure here. */
    fn new() -> Self {
        Self {
            last_logged: HashMap::new(),
        }
    }

    /** Returns true if the message should be printed in the given timestamp, otherwise returns false.
    If this method returns false, the message will not be printed.
    The timestamp is in seconds granularity. */
    fn should_print_message(&mut self, timestamp: i32, message: String) -> bool {
        match self.last_logged.get(&message) {
            Some(t) if t + 10 <= timestamp => {
                self.last_logged.insert(message, timestamp);
                true
            }
            Some(_) => false,
            None => {
                self.last_logged.insert(message, timestamp);
                true
            }
        }
    }
}
