pub trait Messenger {
    fn send(&self, msg: &str);
    //no need to define the function as public
}

pub struct LimitTracker<'a, T: Messenger> {//fro some reason we have to put lifetime parameter
    messenger: &'a T,
    value: usize,
    max: usize,
}

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

    pub fn set_val (&mut self, value: usize) {
        self.value = value;
        
        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: you are over your quota");
        } else if percentage_of_max >= 0.9 {
            self.messenger.send("Warning: you've used up 90% of your quota");
        } else if percentage_of_max >= 0.75 {
            self.messenger.send("Warning: you've used up 75% of yoru quota");
        }
    }
}

//we need our mock object to implement the Messenger trait

#[cfg(test)]
mod tests {
    use super::*;

    // struct MockMessenger {
    //     sent_messages: Vec<String>;
    // }

    // impl MockMessenger {
    //     fn new() -> MockMessenger {
    //         MockMessenger {
    //             sent_messages: vec![],
    //         }
    //     }
    // }

    // impl Messenger for MockMessenger {
    //     fn send(&self, message: &str) {
    //         self.sent_messages.push(String::from(message));
    //     }
    // }

    // #[test]
    // fn send_75_percent_warning() {
    //     let mock_messenger = MockMessenger::new();
    //     let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

    //     limit_tracker.set_value(80);

    //     assert_eq!(mock_messenger.sent_messages.len(), 1);
    // }

    use std::cell::RefCell;

    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: RefCell::new(vec![])
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            self.sent_messages.borrow_mut().push(String::from(message));
        }//the RefCell<T> has a borrow_mut() method
    }//it is valid bc at runtime there will be only one mutable reference
    //so this is gonna pass 

    #[test]
    fn send_75_percent_warning() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_val(80);

        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
        //also manually borrow() the value
    }



    //this isn't gonna compile because we're trying to mutate the 
    //sent messages field in the MockMessenger even though send
    //takes in an immutable reference to self. but we also can't 
    //change it to a mutable reference because we need to keep the 
    //Messenger trait having the exact signature it will have in real life; 
    //the fact that we want to mutate our MockMessenger object is just a 
    //quirk of wanting to save all the messages for testing purposes.

    //the solution is RefCell<T>
}
