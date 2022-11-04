use alloc::collections::LinkedList;

use crate::std_prelude::*;
use crate::types::cell::Cell;
use crate::types::once::{new_channel_once, ReceiverOnce, SenderOnce};

struct Channel<T: 'static> {
    send_queue: Cell<LinkedList<SenderOnce<T>>>,
    recv_queue: Cell<LinkedList<ReceiverOnce<T>>>,
}

pub struct Sender<T: 'static> {
    channel: Channel<T>,
}

pub struct Receiver<T: 'static> {
    channel: Channel<T>,
}

pub fn new_channel<T>() -> (Sender<T>, Receiver<T>) {
    let send_queue = Cell::new(LinkedList::new());
    let recv_queue = Cell::new(LinkedList::new());

    let channel = Channel {
        send_queue,
        recv_queue,
    };

    let sender = Sender {
        channel: channel.clone(),
    };

    let receiver = Receiver { channel };

    (sender, receiver)
}

impl<T: Send + Sync + 'static> Sender<T> {
    pub fn send(&self, val: T) {
        let has_sender = !self.channel.send_queue.borrow().is_empty();
        if has_sender {
            let sender = self.channel.send_queue.borrow_mut().pop_front().unwrap();
            sender.send(val);
        } else {
            let (sender, receiver) = new_channel_once();
            self.channel.recv_queue.borrow_mut().push_back(receiver);
            sender.send(val);
        }
    }
}

impl<T> Receiver<T> {
    pub async fn recv(&self) -> T {
        let has_receiver = !self.channel.recv_queue.borrow().is_empty();
        if has_receiver {
            let receiver = self.channel.recv_queue.borrow_mut().pop_front().unwrap();
            receiver.await
        } else {
            let (sender, receiver) = new_channel_once();
            self.channel.send_queue.borrow_mut().push_back(sender);
            receiver.await
        }
    }
}

impl<T> Clone for Channel<T> {
    fn clone(&self) -> Self {
        Self {
            send_queue: self.send_queue.clone(),
            recv_queue: self.recv_queue.clone(),
        }
    }
}
