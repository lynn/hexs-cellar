use std::collections::VecDeque;

// label messages by turn so they can be grouped
pub type Message = (usize, String);

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

    pub fn tell(&mut self, message: String) {
        while self.messages.len() >= MESSAGE_BUFFER_SIZE {
            self.messages.pop_back();
        }

        self.messages.push_front((self.turn_count, message))
    }

    pub fn end_turn(&mut self) {
        self.turn_count += 1
    }
}
