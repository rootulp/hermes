use crate::types::channel::ChannelBuilder;
use crate::types::once::OnceChannelBuilder;
use crate::types::state_change::StateChangeFlag;
use crate::types::task::TaskSpawner;

pub struct TestRuntime {
    pub spawner: TaskSpawner,
    pub channel: ChannelBuilder,
    pub once_channel: OnceChannelBuilder,
}

impl TestRuntime {
    pub fn new() -> Self {
        let flag = StateChangeFlag::new();
        let spawner = TaskSpawner::new(&flag);
        let once_channel = OnceChannelBuilder::new(&flag, &spawner);
        let channel = ChannelBuilder::new(&flag, &once_channel);

        Self {
            spawner,
            channel,
            once_channel,
        }
    }
}
