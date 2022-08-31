use ibc::core::ics23_commitment::merkle::MerkleProof;
use ibc::core::ics24_host::identifier::ClientId;
use ibc::timestamp::Timestamp;
use ibc::Height;
use ibc_proto::google::protobuf::Any;

use crate::core::traits::ibc::IbcTypes;

pub struct CosmosIbcTypes;

impl IbcTypes for CosmosIbcTypes {
    type Height = Height;

    type Timestamp = Timestamp;

    type Message = Any;

    type ClientId = ClientId;

    type MerkleProof = MerkleProof;
}
