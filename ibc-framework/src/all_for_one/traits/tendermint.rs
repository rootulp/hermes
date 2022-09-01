use ibc::core::ics23_commitment::merkle::MerkleProof;
use ibc::core::ics24_host::identifier::ClientId;
use ibc::timestamp::Timestamp;
use ibc::Height;
use ibc_proto::google::protobuf::Any;

use crate::all_for_one::traits::base::AfoContext;
use crate::core::impls::clients::tendermint::{
    TendermintClientHeader, TendermintClientState, TendermintConsensusState, TendermintMisbehavior,
};

pub trait AfoTendermintOnlyContext:
    AfoContext<
    Height = Height,
    Timestamp = Timestamp,
    Message = Any,
    ClientId = ClientId,
    MerkleProof = MerkleProof,
    ClientState = TendermintClientState,
    ConsensusState = TendermintConsensusState,
    ClientHeader = TendermintClientHeader,
    Misbehavior = TendermintMisbehavior,
    AnyClientState = TendermintClientState,
    AnyConsensusState = TendermintConsensusState,
    AnyClientHeader = TendermintClientHeader,
    AnyMisbehavior = TendermintMisbehavior,
>
{
}
