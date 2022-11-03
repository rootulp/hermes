use alloc::collections::VecDeque;

use crate::std_prelude::*;
use crate::types::cell::Cell;
use crate::types::once::{OnceChannelBuilder, ReceiverOnce, SenderOnce};
use crate::types::state_change::StateChangeFlag;

#[derive(Clone)]
pub struct ChannelBuilder {
    builder: OnceChannelBuilder,
    flag: StateChangeFlag,
}

struct Channel<T: 'static> {
    send_queue: Cell<VecDeque<SenderOnce<T>>>,
    recv_queue: Cell<VecDeque<ReceiverOnce<T>>>,
    builder: OnceChannelBuilder,
}

pub struct Sender<T: 'static> {
    channel: Channel<T>,
}

pub struct Receiver<T: 'static> {
    channel: Channel<T>,
}

impl ChannelBuilder {
    pub fn new(flag: &StateChangeFlag, builder: &OnceChannelBuilder) -> Self {
        Self {
            builder: builder.clone(),
            flag: flag.clone(),
        }
    }

    pub fn new_channel<T>(&self) -> (Sender<T>, Receiver<T>) {
        let send_queue = Cell::new(&self.flag);
        let recv_queue = Cell::new(&self.flag);

        let channel = Channel {
            send_queue,
            recv_queue,
            builder: self.builder.clone(),
        };

        let sender = Sender {
            channel: channel.clone(),
        };

        let receiver = Receiver { channel };

        (sender, receiver)
    }
}

impl<T: Send + Sync + 'static> Sender<T> {
    pub fn send(&self, val: T) {
        let has_sender = !self.channel.send_queue.borrow().is_empty();
        if has_sender {
            let sender = self.channel.send_queue.borrow_mut().pop_front().unwrap();
            sender.send(val);
        } else {
            let (sender, receiver) = self.channel.builder.new_channel();
            self.channel.recv_queue.borrow_mut().push_back(receiver);
            sender.send(val);
        }
    }
}

impl<T: Send + 'static> Receiver<T> {
    pub async fn recv(&self) -> T {
        let has_receiver = !self.channel.recv_queue.borrow().is_empty();
        if has_receiver {
            let receiver = self.channel.recv_queue.borrow_mut().pop_front().unwrap();
            receiver.recv().await
        } else {
            let (sender, receiver) = self.channel.builder.new_channel();
            self.channel.send_queue.borrow_mut().push_back(sender);
            receiver.recv().await
        }
    }
}

impl<T> Clone for Channel<T> {
    fn clone(&self) -> Self {
        Self {
            send_queue: self.send_queue.clone(),
            recv_queue: self.recv_queue.clone(),
            builder: self.builder.clone(),
        }
    }
}
