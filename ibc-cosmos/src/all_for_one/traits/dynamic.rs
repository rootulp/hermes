use ibc_framework::all_for_one::traits::base::AfoContext;

use crate::clients::dynamic::client::{
    DynClientHeader, DynClientState, DynConsensusState, DynMisbehavior,
};
use crate::clients::tendermint::client::TendermintClient;

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
