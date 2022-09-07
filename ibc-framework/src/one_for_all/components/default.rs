use crate::core::impls::message_handlers::update_client::BaseUpdateClientMessageHandler;
use crate::one_for_all::impls::stores::{OfaClientReader, OfaClientWriter};
use crate::one_for_all::traits::chain::OfaChain;
use crate::one_for_all::traits::components::OfaChainComponents;

pub struct DefaultChainComponents;

impl<Chain> OfaChainComponents<Chain> for DefaultChainComponents
where
    Chain: OfaChain,
{
    type AnyClientReader = OfaClientReader;

    type AnyClientWriter = OfaClientWriter;

    type UpdateClientMessageHandler = BaseUpdateClientMessageHandler;
}
