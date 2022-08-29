use core::marker::PhantomData;

use crate::aliases::client::{
    AnyClientHeader, AnyClientState, AnyConsensusState, ClientHeader, ClientState, ConsensusState,
};
use crate::traits::chain::{ClientReaderContext, HasClient};
use crate::traits::client::ClientContext;

pub trait AnyClientUpdateVerifier<Chain>
where
    Chain: HasClient,
{
    fn try_update_client_state(
        chain: &Chain,
        client_id: &Chain::ClientId,
        new_client_header: &AnyClientHeader<Chain::ClientContext>,
    ) -> Result<
        (
            AnyClientState<Chain::ClientContext>,
            AnyConsensusState<Chain::ClientContext>,
        ),
        Chain::Error,
    >;
}

pub trait ClientUpdateVerifier<Chain>
where
    Chain: HasClient,
    Chain::ClientContext: ClientContext<Self::ClientTag>,
{
    type ClientTag;

    fn try_update_client_state(
        chain: &Chain,
        client_id: &Chain::ClientId,
        new_client_header: &ClientHeader<Chain::ClientContext, Self::ClientTag>,
    ) -> Result<
        (
            ClientState<Chain::ClientContext, Self::ClientTag>,
            ConsensusState<Chain::ClientContext, Self::ClientTag>,
        ),
        Chain::Error,
    >;
}

pub struct MismatchClientHeaderFormat<ClientType> {
    pub expected_client_type: ClientType,
}

pub struct CombineClientUpdateVerifier<Verifier, NextVerifiers>(
    pub PhantomData<(Verifier, NextVerifiers)>,
);

impl<Chain, Verifier, NextVerifiers, Client> AnyClientUpdateVerifier<Chain>
    for CombineClientUpdateVerifier<Verifier, NextVerifiers>
where
    Chain: HasClient<ClientContext = Client>,
    Chain: ClientReaderContext,
    Client: ClientContext<Verifier::ClientTag>,
    Verifier: ClientUpdateVerifier<Chain>,
    NextVerifiers: AnyClientUpdateVerifier<Chain>,
    Chain::Error: From<MismatchClientHeaderFormat<Client::ClientType>>,
{
    fn try_update_client_state(
        chain: &Chain,
        client_id: &Chain::ClientId,
        new_client_header: &AnyClientHeader<Chain::ClientContext>,
    ) -> Result<
        (
            AnyClientState<Chain::ClientContext>,
            AnyConsensusState<Chain::ClientContext>,
        ),
        Chain::Error,
    > {
        let client_type = chain.get_client_type(client_id)?;

        if client_type == Client::CLIENT_TYPE {
            let m_client_header = Client::try_from_any_client_header(new_client_header);

            match m_client_header {
                Some(in_client_header) => {
                    let (new_client_state, new_consensus_state) =
                        Verifier::try_update_client_state(chain, client_id, in_client_header)?;

                    Ok((
                        Client::to_any_client_state(new_client_state),
                        Client::to_any_consensus_state(new_consensus_state),
                    ))
                }
                None => Err(MismatchClientHeaderFormat {
                    expected_client_type: client_type,
                }
                .into()),
            }
        } else {
            NextVerifiers::try_update_client_state(chain, client_id, new_client_header)
        }
    }
}
