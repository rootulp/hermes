use crate::core::traits::stores::client_reader::HasAnyClientReader;
use crate::core::traits::stores::client_writer::HasAnyClientWriter;
use crate::one_for_all::traits::chain::OfaChain;
use crate::one_for_all::traits::components::OfaComponents;
use crate::one_for_all::types::chain::OfaChainWrapper;

impl<Chain> HasAnyClientReader for OfaChainWrapper<Chain>
where
    Chain: OfaChain,
{
    type AnyClientReader = <Chain::Components as OfaComponents<Chain>>::AnyClientReader;
}

impl<Chain> HasAnyClientWriter for OfaChainWrapper<Chain>
where
    Chain: OfaChain,
{
    type AnyClientWriter = <Chain::Components as OfaComponents<Chain>>::AnyClientWriter;
}
