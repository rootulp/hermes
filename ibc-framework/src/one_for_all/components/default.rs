use crate::one_for_all::impls::stores::{OfaClientReader, OfaClientWriter};
use crate::one_for_all::traits::chain::OfaChain;
use crate::one_for_all::traits::components::OfaComponents;

pub struct DefaultComponents;

impl<Chain> OfaComponents<Chain> for DefaultComponents
where
    Chain: OfaChain,
{
    type AnyClientReader = OfaClientReader;

    type AnyClientWriter = OfaClientWriter;
}
