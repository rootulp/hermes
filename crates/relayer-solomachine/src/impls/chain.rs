use async_trait::async_trait;

use ibc_relayer_all_in_one::one_for_all::traits::chain::OfaChain;
use ibc_relayer::chain::handle::ChainHandle;
use ibc_relayer_runtime::tokio::context::TokioRuntimeContext;
use ibc_relayer_runtime::tokio::logger::tracing::TracingLogger;
use ibc_relayer_types::core::ics24_host::identifier::{
    ChainId, ChannelId, ClientId, ConnectionId, PortId,
};
use ibc_relayer_types::Height;
use ibc_relayer_types::timestamp::Timestamp;

use crate::contexts::chain::SolomachineChain;
use crate::types::error::Error;

#[async_trait]
impl<Chain> OfaChain for SolomachineChain<Chain>
where 
    Chain: ChainHandle,
{
    type Error = Error;

    type Runtime = TokioRuntimeContext;

    type Logger = TracingLogger;

    type Height = Height;

    type Timestamp = Timestamp;

    // type Telemetry = ();

    // type Message = SolomachineMessage;

    // type Event = Arc<SolomachineEvent>;

    type ChainId = ChainId;

    type ClientId = ClientId;

    type ConnectionId = ConnectionId;

    type ChannelId = ChannelId;

    type PortId = PortId;

    type Sequence = Sequence;

    // type ConsensusState = SolomachineConsensusState;

    type ChainStatus = ChainStatus;


}