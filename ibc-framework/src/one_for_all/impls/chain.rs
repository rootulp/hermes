use crate::core::traits::client::{HasAnyClientMethods, HasAnyClientTypes};
use crate::core::traits::error::HasError;
use crate::core::traits::event::HasEventTypes;
use crate::core::traits::host::{HasHostMethods, HasHostTypes};
use crate::core::traits::ibc::HasIbcTypes;
use crate::core::traits::message::{HasMessageMethods, HasMessageTypes};
use crate::one_for_all::traits::chain::OfaChain;
use crate::one_for_all::types::chain::OfaChainWrapper;

impl<Chain> HasError for OfaChainWrapper<Chain>
where
    Chain: OfaChain,
{
    type Error = Chain::Error;
}

impl<Chain> HasEventTypes for OfaChainWrapper<Chain>
where
    Chain: OfaChain,
{
    type Event = Chain::Event;
}

impl<Chain> HasHostTypes for OfaChainWrapper<Chain>
where
    Chain: OfaChain,
{
    type Height = Chain::Height;

    type Timestamp = Chain::Timestamp;

    type Duration = Chain::Duration;
}

impl<Chain> HasHostMethods for OfaChainWrapper<Chain>
where
    Chain: OfaChain,
{
    fn host_height(&self) -> Self::Height {
        self.chain.host_height()
    }

    fn host_timestamp(&self) -> Self::Timestamp {
        self.chain.host_timestamp()
    }

    fn add_duration(time: &Self::Timestamp, duration: &Self::Duration) -> Self::Timestamp {
        Chain::add_duration(time, duration)
    }
}

impl<Chain> HasIbcTypes for OfaChainWrapper<Chain>
where
    Chain: OfaChain,
{
    type ClientId = Chain::ClientId;

    type ConnectionId = Chain::ConnectionId;

    type ChannelId = Chain::ChannelId;

    type Port = Chain::Port;

    type MerkleProof = Chain::MerkleProof;
}

impl<Chain> HasMessageTypes for OfaChainWrapper<Chain>
where
    Chain: OfaChain,
{
    type Message = Chain::Message;

    type MessageType = Chain::MessageType;

    type Signer = Chain::Signer;
}

impl<Chain> HasMessageMethods for OfaChainWrapper<Chain>
where
    Chain: OfaChain,
{
    fn message_type(message: &Self::Message) -> &Self::MessageType {
        Chain::message_type(message)
    }

    fn message_signer(message: &Self::Message) -> &Self::Signer {
        Chain::message_signer(message)
    }
}

impl<Chain> HasAnyClientTypes for OfaChainWrapper<Chain>
where
    Chain: OfaChain,
{
    type ClientType = Chain::ClientType;

    type AnyClientState = Chain::AnyClientState;

    type AnyConsensusState = Chain::AnyConsensusState;

    type AnyClientHeader = Chain::AnyClientHeader;

    type AnyMisbehavior = Chain::AnyMisbehavior;
}

impl<Chain> HasAnyClientMethods for OfaChainWrapper<Chain>
where
    Chain: OfaChain,
{
    fn client_state_type(client_state: &Self::AnyClientState) -> Self::ClientType {
        Chain::client_state_type(client_state)
    }

    fn client_state_is_frozen(client_state: &Self::AnyClientState) -> bool {
        Chain::client_state_is_frozen(client_state)
    }

    fn client_state_trusting_period(client_state: &Self::AnyClientState) -> Self::Duration {
        Chain::client_state_trusting_period(client_state)
    }

    fn client_state_latest_height(client_state: &Self::AnyClientState) -> Self::Height {
        Chain::client_state_latest_height(client_state)
    }

    fn consensus_state_timestamp(consensus_state: &Self::AnyConsensusState) -> Self::Timestamp {
        Chain::consensus_state_timestamp(consensus_state)
    }

    fn client_header_height(client_header: &Self::AnyClientHeader) -> Self::Height {
        Chain::client_header_height(client_header)
    }
}
