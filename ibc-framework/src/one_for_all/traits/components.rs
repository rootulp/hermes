use crate::core::traits::stores::client_reader::AnyClientReader;
use crate::core::traits::stores::client_writer::AnyClientWriter;
use crate::one_for_all::traits::chain::OfaChain;
use crate::one_for_all::types::chain::OfaChainWrapper;

pub trait OfaComponents<Chain>
where
    Chain: OfaChain,
{
    type AnyClientReader: AnyClientReader<OfaChainWrapper<Chain>>;

    type AnyClientWriter: AnyClientWriter<OfaChainWrapper<Chain>>;
}
