use core::time::Duration;
use ibc::core::ics02_client::client_type::ClientType;
use ibc::core::ics02_client::msgs::update_client::MsgUpdateClient;
use ibc::core::ics23_commitment::merkle::MerkleProof;
use ibc::core::ics24_host::identifier::{ChannelId, ClientId, ConnectionId, PortId};
use ibc::events::IbcEvent;
use ibc::signer::Signer;
use ibc::timestamp::Timestamp;
use ibc::Height;
use ibc_framework::core::traits::client::HasAnyClientTypes;
use ibc_framework::one_for_all::traits::chain::OfaChainTypes;
use ibc_proto::google::protobuf::Any;

use crate::one_for_all::traits::chain::OfaCosmosChain;
use crate::one_for_all::types::chain::OfaCosmosChainWrapper;

impl<Chain> OfaChainTypes for OfaCosmosChainWrapper<Chain>
where
    Chain: OfaCosmosChain,
{
    type Error = Chain::Error;

    type Event = IbcEvent;

    type Height = Height;

    type Timestamp = Timestamp;

    type Duration = Duration;

    type Message = Any;

    type MessageType = String;

    type Signer = Signer;

    type ClientId = ClientId;

    type ConnectionId = ConnectionId;

    type ChannelId = ChannelId;

    type Port = PortId;

    type MerkleProof = MerkleProof;

    type ClientType = ClientType;

    type AnyClientState = <Chain::AnyClient as HasAnyClientTypes>::AnyClientState;

    type AnyConsensusState = <Chain::AnyClient as HasAnyClientTypes>::AnyConsensusState;

    type AnyClientHeader = <Chain::AnyClient as HasAnyClientTypes>::AnyClientHeader;

    type AnyMisbehavior = <Chain::AnyClient as HasAnyClientTypes>::AnyMisbehavior;

    type UpdateClientMessage = MsgUpdateClient;
}
