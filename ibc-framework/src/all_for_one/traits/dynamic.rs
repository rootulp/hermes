use crate::all_for_one::traits::base::AfoContext;
use crate::core::impls::clients::dynamic::DynamicClient;
use crate::core::impls::clients::tendermint::TendermintClient;
use crate::core::impls::ibc::CosmosIbcTypes;

pub trait AfoDynamicTendermintContext:
    AfoContext<Client = TendermintClient, AnyClient = DynamicClient, IbcTypes = CosmosIbcTypes>
{
}
