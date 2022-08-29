use crate::traits::client::{AnyClientContext, ClientContext};

pub type ClientType<Context> = <Context as AnyClientContext>::ClientType;

pub type AnyClientState<Context> = <Context as AnyClientContext>::AnyClientState;

pub type ClientState<Context, ClientTag> = <Context as ClientContext<ClientTag>>::ClientState;

pub type AnyConsensusState<Context> = <Context as AnyClientContext>::AnyConsensusState;

pub type ConsensusState<Context, ClientTag> = <Context as ClientContext<ClientTag>>::ConsensusState;

pub type AnyClientHeader<Context> = <Context as AnyClientContext>::AnyClientHeader;

pub type ClientHeader<Context, ClientTag> = <Context as ClientContext<ClientTag>>::ClientHeader;
