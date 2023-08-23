pub trait Messager {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messager> {
    messager: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
    where T: Messager
{
    pub fn new(messager: &T, max: usize) -> LimitTracker<T> {
        LimitTracker {
            messager,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;
        let percentage_of_max = self.value as f64 / self.max as f64;
        println!("max: {}, value: {}, percentage_of_max: {}", self.max, self.value, percentage_of_max);
        if percentage_of_max > 1.0 {
            self.messager.send("错误：当前参数已经超过最大值!");
        } else if percentage_of_max < 0.9 && percentage_of_max >= 0.75 {
            self.messager
                .send("警告: 当前剩余空间已经不足90%!");
        } else if percentage_of_max < 0.75 && percentage_of_max >= 0.2 {
            self.messager
                .send("警告: 当前剩余空间已经不足75%!");
        } else if percentage_of_max < 0.2 {
            self.messager
                .send("警告: 当前剩余空间已经不足20%!");
        }
    }
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use super::*;

    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: RefCell::new(vec![String::from("test")]),
            }
        }
    }

    impl Messager for MockMessenger {
        fn send(&self, message: &str) {
            self.sent_messages.borrow_mut().push(String::from(message));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);
        limit_tracker.set_value(80);
        println!("mock_messenger.sent_messages: {:?}", mock_messenger.sent_messages.borrow());
        limit_tracker.set_value(50);
        println!("mock_messenger.sent_messages: {:?}", mock_messenger.sent_messages.borrow());
        limit_tracker.set_value(10);
        println!("mock_messenger.sent_messages: {:?}", mock_messenger.sent_messages.borrow());
    }

    #[test]
    fn it_sends_an_over_90_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);
        limit_tracker.set_value(95);
        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }
}