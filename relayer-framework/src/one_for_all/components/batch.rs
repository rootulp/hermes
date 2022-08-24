use crate::extras::batch::message_sender::SendMessagetoBatchWorker;
use crate::impls::message_senders::chain_sender::SendIbcMessagesToChain;
use crate::impls::message_senders::update_client::SendIbcMessagesWithUpdateClient;
use crate::impls::messages::skip_update_client::SkipUpdateClient;
use crate::impls::messages::wait_update_client::WaitUpdateClient;
use crate::impls::packet_relayers::top::TopRelayer;
use crate::one_for_all::impls::chain::OfaConsensusStateQuerier;
use crate::one_for_all::impls::relay::OfaUpdateClientMessageBuilder;
use crate::one_for_all::impls::status::OfaChainStatusQuerier;
use crate::one_for_all::traits::batch::OfaChainWithBatch;
use crate::one_for_all::traits::chain::{OfaChain, OfaIbcChain};
use crate::one_for_all::traits::components::batch::OfaBatchComponents;
use crate::one_for_all::traits::components::chain::{OfaChainComponents, OfaIbcChainComponents};
use crate::one_for_all::traits::components::relay::OfaRelayComponents;
use crate::one_for_all::traits::relay::OfaRelay;

pub struct BatchComponents;

impl<Chain> OfaChainComponents<Chain> for BatchComponents
where
    Chain: OfaChain,
{
    type ChainStatusQuerier = OfaChainStatusQuerier;
}

impl<Chain, Counterparty> OfaIbcChainComponents<Chain, Counterparty> for BatchComponents
where
    Chain: OfaIbcChain<Counterparty>,
    Counterparty: OfaIbcChain<Chain>,
{
    type ConsensusStateQuerier = OfaConsensusStateQuerier;
}

impl<Relay> OfaRelayComponents<Relay> for BatchComponents
where
    Relay: OfaRelay<Components = BatchComponents>,
    Relay::SrcChain: OfaChainWithBatch,
    Relay::DstChain: OfaChainWithBatch,
{
    type PacketRelayer = TopRelayer;

    type UpdateClientMessageBuilder =
        SkipUpdateClient<WaitUpdateClient<OfaUpdateClientMessageBuilder>>;

    type IbcMessageSender = SendMessagetoBatchWorker;
}

impl<Relay> OfaBatchComponents<Relay> for BatchComponents
where
    Relay: OfaRelay<Components = BatchComponents>,
    Relay::SrcChain: OfaChainWithBatch,
    Relay::DstChain: OfaChainWithBatch,
{
    type IbcMessageSenderForBatchWorker = SendIbcMessagesWithUpdateClient<SendIbcMessagesToChain>;
}