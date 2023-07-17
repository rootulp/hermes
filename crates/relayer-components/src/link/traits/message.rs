use crate::chain::traits::types::height::HasHeightType;
use crate::chain::traits::types::message::HasMessageType;
use crate::core::traits::sync::Async;
use crate::std_prelude::*;

pub trait HasCounterpartyMessageHeight<Chain, Counterparty>: Async
where
    Chain: HasMessageType,
    Counterparty: HasHeightType,
{
    /**
       Get the height of the counterparty chain that an IBC message is based
       on when it is constructed to be sent to the self chain. If the message
       is not IBC-related, this would return `None`.

       This is used by the
       [`SendIbcMessagesWithUpdateClient`](crate::relay::impls::message_senders::update_client::SendIbcMessagesWithUpdateClient)
       message sender middleware to attach `UpdateClient` messages to the
       front of the message batch before sending it to the downstream
       message sender.

       The way this works is as follows: recall that the relayer relays IBC
       packets by constructing messages from one chain and send it to
       the other chain. In this case, we have IBC events happening on
       the `Counterparty` chain, which the relayer would construct
       messages targetting this self chain. So any IBC message that the self
       chain received would correspond to events happening on the `Counterparty`
       chain. With this method, we are thus getting the
       [`Counterparty::Height`](crate::chain::traits::types::height::HasHeightType::Height)
       and _not_ `Self::Height`.
    */
    fn counterparty_message_height(message: &Chain::Message) -> Option<Counterparty::Height>;
}
