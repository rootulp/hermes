use crate::std_prelude::*;
use crate::types::cell::Cell;
use crate::types::state_change::StateChangeFlag;
use crate::types::task::TaskSpawner;
use crate::utils::future::new_future;

#[derive(Clone)]
pub struct OnceChannelBuilder {
    spawner: TaskSpawner,
    flag: StateChangeFlag,
}

pub struct ReceiverOnce<T: 'static> {
    cell: Cell<Option<T>>,
}

pub struct SenderOnce<T: 'static> {
    cell: Cell<Option<T>>,
    spawner: TaskSpawner,
}

impl OnceChannelBuilder {
    pub fn new(flag: &StateChangeFlag, spawner: &TaskSpawner) -> Self {
        Self {
            spawner: spawner.clone(),
            flag: flag.clone(),
        }
    }

    pub fn new_channel<T>(&self) -> (SenderOnce<T>, ReceiverOnce<T>) {
        let cell = Cell::new(&self.flag);
        let sender = SenderOnce {
            cell: cell.clone(),
            spawner: self.spawner.clone(),
        };
        let receiver = ReceiverOnce { cell };
        (sender, receiver)
    }
}

impl<T> ReceiverOnce<T> {
    pub async fn recv(self) -> T {
        let cell = self.cell;
        new_future(move || {
            let has_val = cell.borrow().is_some();

            if has_val {
                cell.borrow_mut().take()
            } else {
                None
            }
        })
        .await
    }
}

impl<T: Send + Sync + 'static> SenderOnce<T> {
    pub fn send(self, val: T) {
        let spawner = self.spawner;
        let cell = self.cell;
        spawner.spawn(async move { *cell.borrow_mut() = Some(val) });
    }
}
