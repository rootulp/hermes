use crate::all_for_one::traits::base::AfoContext;
use crate::core::impls::clients::dynamic::{
    DynClientHeader, DynClientState, DynConsensusState, DynMisbehavior,
};
use crate::core::impls::clients::tendermint::TendermintClient;

pub trait AfoDynamicTendermintContext:
    AfoContext<
    ClientHandler = TendermintClient,
    ClientState = DynClientState,
    ConsensusState = DynConsensusState,
    ClientHeader = DynClientHeader,
    Misbehavior = DynMisbehavior,
>
{
}
