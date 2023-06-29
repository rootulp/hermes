use core::time::Duration;

use ibc_relayer::client_state::AnyClientState;
use ibc_relayer_types::core::ics03_connection::version::Version;
use ibc_relayer_types::core::ics23_commitment::commitment::CommitmentPrefix;
use ibc_relayer_types::proofs::Proofs;

pub struct CosmosInitConnectionOptions {
    pub delay_period: Duration,
    pub connection_version: Version,
}

pub struct CosmosConnectionOpenInitPayload {
    pub commitment_prefix: CommitmentPrefix,
}

pub struct CosmosConnectionOpenTryPayload {
    pub commitment_prefix: CommitmentPrefix,
    pub proofs: Proofs,
    pub client_state: Option<AnyClientState>,
    pub versions: Vec<Version>,
    pub delay_period: Duration,
}