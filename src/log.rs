use std::collections::VecDeque;
use std::borrow::Cow;

// label messages by turn so they can be grouped
pub type Message = (usize, Cow<'static, str>);

const MESSAGE_BUFFER_SIZE: usize = 512;

pub struct Log {
    turn_count: usize,
    messages: VecDeque<Message>
}

impl Log {
    pub fn new() -> Log {
        Log {
            turn_count: 1,
            messages: VecDeque::with_capacity(MESSAGE_BUFFER_SIZE)
        }
    }

    // return a VecDeque starting at the most recent message and going backwards
    pub fn recent_messages(&self) -> &VecDeque<Message> {
        &self.messages
    }

    // this trait bound allows us to pass in both &'static str and String
    pub fn tell<S>(&mut self, message: S) where S: Into<Cow<'static, str>> {
        while self.messages.len() >= MESSAGE_BUFFER_SIZE {
            self.messages.pop_back();
        }

        self.messages.push_front((self.turn_count, message.into()))
    }

    pub fn end_turn(&mut self) {
        self.turn_count += 1
    }
}
