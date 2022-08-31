use crate::traits::client::{AnyClientTypes, ClientTypes};

pub type ClientType<Context> = <Context as AnyClientTypes>::ClientType;

pub type AnyClientState<Context> = <Context as AnyClientTypes>::AnyClientState;

pub type ClientState<Context> = <Context as ClientTypes>::ClientState;

pub type AnyConsensusState<Context> = <Context as AnyClientTypes>::AnyConsensusState;

pub type ConsensusState<Context> = <Context as ClientTypes>::ConsensusState;

pub type AnyClientHeader<Context> = <Context as AnyClientTypes>::AnyClientHeader;

pub type ClientHeader<Context> = <Context as ClientTypes>::ClientHeader;
