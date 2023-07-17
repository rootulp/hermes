use core::fmt::Display;

use crate::chain::traits::types::chain::HasChainTypes;
use crate::chain::traits::types::height::HasHeightType;
use crate::chain::traits::types::message::HasMessageType;
use crate::core::traits::sync::Async;
use crate::std_prelude::*;

/**
   The abstract types for a chain context when it is used for IBC
   communication with a `Counterparty` chain context.

   In contrast to [`HasChainTypes`], this trait is parameterized by a
   `Counterparty` chain context. Additionally, the `Counterparty` chain context
   is arequired to implement
   [`HasChainTypes`].

   Because of the `Counterparty` parameter, the associated types
   in this trait are going to be different when used with different
   counterparty chain contexts. In other words, the type
   `<ChainA as HasIbcChainTypes<ChainB>>::ClientId` is different from
   `<ChainA as HasIbcChainTypes<ChainC>>::ClientId` if `ChainB` and `ChainC`
   are different.

   This is intentional, as we want to distinguish IBC identifiers associated
   with different chains and avoid accidentally mixing them up. This is
   particularly useful when implementing the relayer, because we cannot
   for example accidentally use a `ChannelId` from `SrcChain` to `DstChain`
   as a `ChannelId` from `DstChain` to `SrcChain`.

   Having the IBC chain types parameterized on the counterparty chain also
   allows a chain context to decide on different concrete types depending
   on which counterparty chain it is. For example, a Cosmos chain context
   connected with a non-Cosmos chain context may want to use different
   `ClientId` type, as compared to connecting to a Cosmos chain.

   Note that even when a chain context implements `HasIbcChainTypes`, it is
   _not_ expected to have access to resources on the counterparty chain. That
   would require access to the counterparty chain context, which is implemented
   separately from the self chain context. Instead, operations that require
   access to two chain contexts are handled by the
   [relay context](crate::relay).
*/
pub trait HasIbcTypes<Chain, Counterparty> {
    /**
       The client ID of the counterparty chain, that is stored on the self
       chain.
    */
    type ClientId: Display + Async;

    /**
       The connection ID of the counterparty chain, that is stored on the self
       chain.
    */
    type ConnectionId: Display + Async;

    /**
       The channel ID of the counterparty chain, that is stored on the self
       chain.
    */
    type ChannelId: Display + Async;

    /**
       The port ID of the counterparty chain, that is stored on the self
       chain.
    */
    type PortId: Display + Async;

    /**
       The IBC packet sequence for the packet that is sent from the self chain
       to the counterparty chain.

       Note that for sequences of packets that are sent from the counterparty
       chain to self, the `Counterparty::Sequence` will be used.
    */
    type Sequence: Display + Async;
}

pub trait HasCounterpartyIbcTypes<Chain, Counterparty>:
    HasIbcTypes<
    Counterparty,
    Chain,
    ClientId = Self::CounterpartyClientId,
    ConnectionId = Self::CounterpartyConnectionId,
    ChannelId = Self::CounterpartyChannelId,
    PortId = Self::CounterpartyPortId,
    Sequence = Self::CounterpartySequence,
>
{
    type CounterpartyClientId: Display + Async;

    type CounterpartyConnectionId: Display + Async;

    type CounterpartyChannelId: Display + Async;

    type CounterpartyPortId: Display + Async;

    type CounterpartySequence: Display + Async;
}

impl<Link, Chain, Counterparty> HasCounterpartyIbcTypes<Chain, Counterparty> for Link
where
    Link: HasIbcTypes<Counterparty, Chain>,
{
    type CounterpartyClientId = Link::ClientId;

    type CounterpartyConnectionId = Link::ConnectionId;

    type CounterpartyPortId = Link::PortId;

    type CounterpartySequence = Link::Sequence;
}
pub trait HasTwoWayIbcTypes<Chain, Counterparty>:
    HasIbcTypes<
        Chain,
        Counterparty,
        ClientId = Self::TargetClientId,
        ConnectionId = Self::TargetConnectionId,
        ChannelId = Self::TargetChannelId,
        PortId = Self::TargetPortId,
        Sequence = Self::TargetSequence,
    > + HasCounterpartyIbcTypes<Chain, Counterparty>
{
    type TargetClientId: Display + Async;

    type TargetConnectionId: Display + Async;

    type TargetChannelId: Display + Async;

    type TargetPortId: Display + Async;

    type TargetSequence: Display + Async;
}

impl<Link, Chain, Counterparty> HasTwoWayIbcTypes<Chain, Counterparty> for Link
where
    Link: HasIbcTypes<Chain, Counterparty> + HasIbcTypes<Chain, Counterparty>,
{
    type TargetClientId = <Link as HasIbcTypes<Chain, Counterparty>>::ClientId;

    type TargetConnectionId = <Link as HasIbcTypes<Chain, Counterparty>>::ConnectionId;

    type TargetPortId = <Link as HasIbcTypes<Chain, Counterparty>>::PortId;

    type TargetSequence = <Link as HasIbcTypes<Chain, Counterparty>>::Sequence;
}
