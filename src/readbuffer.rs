// Copyright 2015 Nathan Sizemore <nathanrsizemore@gmail.com>
//
// This Source Code Form is subject to the
// terms of the Mozilla Public License, v.
// 2.0. If a copy of the MPL was not
// distributed with this file, You can
// obtain one at
// http://mozilla.org/MPL/2.0/.
//
// This Source Code Form is "Incompatible
// With Secondary Licenses", as defined by
// the Mozilla Public License, v. 2.0.


//! ReadBuffer crate.


use super::message::Message;

#[derive(Clone)]
pub struct ReadBuffer {
    /// Current message
    c_msg: Message,
    /// Current bytes remaining for next read
    c_remaining: u16,
    /// Current buffer
    c_buffer: Vec<u8>,
    /// Queue of messages created during last read
    queue: Vec<Message>
}


impl ReadBuffer {

    /// Creates a new ReadBuffer
    pub fn new() -> ReadBuffer {
        ReadBuffer {
            c_msg: Message::new(),
            c_remaining: 2u16,
            c_buffer: Vec::<u8>::with_capacity(2),
            queue: Vec::<Message>::new()
        }
    }

    /// Returns the number of remaining u8 needed to fill the current buffer
    pub fn remaining(&self) -> u16 {
        self.c_remaining.clone()
    }

    /// Pushes elem onto buffer
    pub fn push(&mut self, elem: u8) {
        self.c_buffer.push(elem);
        self.c_remaining -= 1;
    }

    /// Sets the buffer's capacity to size
    pub fn set_capacity(&mut self, size: u16) {
        self.c_remaining = size;
        self.c_buffer = Vec::<u8>::with_capacity(size as usize);
    }

    /// Calculates the payload len from the current buffer
    pub fn calc_payload_len(&mut self) {
        let mut len = 0u16;
        len = len | self.c_buffer[0] as u16;
        len = (len << 8) | self.c_buffer[1] as u16;
        self.c_msg.len = len;
    }

    /// Returns the total length of the expected payload
    pub fn payload_len(&self) -> u16 {
        self.c_msg.len.clone()
    }

    /// Pushes the current message into the buffer's message queue and resets
    /// to default state
    pub fn reset(&mut self) {
        self.queue.push(self.c_msg.clone());
        self.c_msg = Message::new();
        self.c_remaining = 2u16;
        self.c_buffer = Vec::<u8>::with_capacity(2);
    }

    /// Returns the length of internal message queue
    pub fn queue_len(&self) -> usize {
        self.queue.len()
    }

    /// Returns a mutable reference to the internal message queue
    pub fn queue_as_mut(&mut self) -> &mut Vec<Message> {
        &mut self.queue
    }

    /// Drains the queue returning a Vec<Vec<u8>> representing each payload
    pub fn drain_queue(&mut self) -> Vec<Vec<u8>> {
        let mut buffer = Vec::<Vec<u8>>::with_capacity(self.queue.len());
        let mut pos = self.queue.len() - 1;
        while let Some(msg) = self.queue.pop() {
            buffer.insert(pos, msg.payload);
            pos -= 1;
        }
        buffer
    }
}
