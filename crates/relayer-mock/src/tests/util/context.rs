use alloc::string::{String, ToString};
use std::sync::Arc;

use crate::base::types::runtime::MockRuntimeContext;
use crate::contexts::chain::MockChainContext;
use crate::contexts::relay::MockRelayContext;
use crate::util::clock::MockClock;

pub fn build_mock_relay_context() -> (
    MockRelayContext,
    Arc<MockChainContext>,
    Arc<MockChainContext>,
) {
    let clock = Arc::new(MockClock::default());
    let runtime = MockRuntimeContext::new(clock.clone());
    let src_chain = Arc::new(MockChainContext::new("chain1".to_string(), clock.clone()));
    let dst_chain = Arc::new(MockChainContext::new("chain2".to_string(), clock));
    let relay = MockRelayContext::new(
        src_chain.clone(),
        dst_chain.clone(),
        String::from("client_src_to_dst"),
        String::from("client_dst_to_src"),
        runtime,
    );

    (relay, src_chain, dst_chain)
}
