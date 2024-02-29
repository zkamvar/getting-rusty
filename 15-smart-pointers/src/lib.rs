pub trait Messenger {
    // default messenger trait is just to send
    // NOTE: this is an _immutable_ reference to self
    // An example usage would be to send a message and include information about
    // the size of the object, but we wouldn't want to store the messages into
    // memory.
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

// Limit tracker is a specific kind of object that will track a given value
// for an object that has a `Messenger` trait. With this trait, messages can be
// sent to the object if the value is over the quota.
//
// Why is this useful? Imagine that we release this as a library because it's
// useful to have a limit tracker for various kinds of objects across libraries.
// A given object would use the `send()` method of the object with the Messenger
// trait to send a message externally.
impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &'a T, max: usize) -> LimitTracker<'a, T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over quota!");
        } else if percentage_of_max >= 0.9 {
            self.messenger
                .send("Urgent warning: You've used up over 90% of your quota!");
        } else if percentage_of_max >= 0.75 {
            self.messenger
                .send("Warning: You've used up over 75% of your quota!");
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::cell::RefCell;

    // mocking a messenger object. In a more realistic scenario, we sould likely
    // have an object that has a space for messages and implements the Messenger
    // trait.
    struct MockMessenger {
        // sent_messages: Vec<String>,
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                // sent_messages: vec![],
                sent_messages: RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for MockMessenger {
        // NOTE: method signature is _immutable_,
        // which matches the trait definition
        fn send(&self, message: &str) {
            self.sent_messages
                .borrow_mut() // This creates a mutable reference (a la &mut)
                // and returns a `RefMut<T>` type, which implements Deref
                .push(String::from(message));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);
        limit_tracker.set_value(80);

        assert_eq!(
            mock_messenger
                .sent_messages
                .borrow() // creates an immutable reference (a la &)
                // returning a `Ref<T>` value
                .len(),
            1
        );
    }
    #[test]
    fn it_is_silent_under_75_percent() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);
        limit_tracker.set_value(75);

        assert_eq!(
            mock_messenger
                .sent_messages
                .borrow() // creates an immutable reference (a la &)
                // returning a `Ref<T>` value
                .len(),
            1
        );
        limit_tracker.set_value(74);

        assert_eq!(
            mock_messenger
                .sent_messages
                .borrow() // creates an immutable reference (a la &)
                // returning a `Ref<T>` value
                .len(),
            1
        );
    }
}
