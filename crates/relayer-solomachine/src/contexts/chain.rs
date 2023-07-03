use ibc_relayer::chain::handle::ChainHandle;
use ibc_relayer_all_in_one::one_for_all::types::runtime::OfaRuntimeWrapper;
use ibc_relayer_all_in_one::one_for_all::types::telemetry::OfaTelemetryWrapper;
use ibc_relayer_runtime::tokio::context::TokioRuntimeContext;
use ibc_relayer_types::core::ics24_host::identifier::ChainId;

#[derive(Clone)]
pub struct SolomachineChain<Handle: ChainHandle> {
    pub handle: Handle,
    pub chain_id: ChainId,
    pub runtime: OfaRuntimeWrapper<TokioRuntimeContext>,
    pub telemetry: OfaTelemetryWrapper<SolomachineTelemetry>,
}

impl<Handle: ChainHandle> SolomachineChain<Handle> {}