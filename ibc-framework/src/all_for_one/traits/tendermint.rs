use crate::all_for_one::traits::base::AfoContext;
use crate::core::impls::clients::tendermint::TendermintClient;
use crate::core::impls::ibc::CosmosIbcTypes;

pub trait AfoTendermintOnlyContext:
    AfoContext<Client = TendermintClient, AnyClient = TendermintClient, IbcTypes = CosmosIbcTypes>
{
}
