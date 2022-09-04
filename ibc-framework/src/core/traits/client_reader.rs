use crate::core::traits::client::{
    ContainsClient, HasAnyClientTypes, HasClientTypeFor, HasClientTypes,
};
use crate::core::traits::error::HasError;
use crate::core::traits::ibc::HasIbcTypes;

pub trait AnyClientReader: HasAnyClientTypes + HasIbcTypes + HasError {
    fn get_client_type(&self, client_id: &Self::ClientId) -> Result<Self::ClientType, Self::Error>;

    fn get_any_client_state(
        &self,
        client_id: &Self::ClientId,
    ) -> Result<Self::AnyClientState, Self::Error>;

    fn get_latest_any_consensus_state(
        &self,
        client_id: &Self::ClientId,
    ) -> Result<Self::AnyConsensusState, Self::Error>;

    fn get_any_consensus_state_at_height(
        &self,
        client_id: &Self::ClientId,
        height: &Self::Height,
    ) -> Result<Option<Self::AnyConsensusState>, Self::Error>;

    fn get_any_consensus_state_after_height(
        &self,
        client_id: &Self::ClientId,
        height: &Self::Height,
    ) -> Result<Option<Self::AnyConsensusState>, Self::Error>;

    fn get_any_consensus_state_before_height(
        &self,
        client_id: &Self::ClientId,
        height: &Self::Height,
    ) -> Result<Option<Self::AnyConsensusState>, Self::Error>;
}

pub struct MismatchClientFormat<ClientType> {
    pub expected_client_type: ClientType,
}

pub trait ClientReader<Client>: HasError + HasIbcTypes + ContainsClient<Client>
where
    Client: HasClientTypes,
{
    fn get_client_state(
        &self,
        client_id: &Self::ClientId,
    ) -> Result<Client::ClientState, Self::Error>;

    fn get_consensus_state_at_height(
        &self,
        client_id: &Self::ClientId,
        height: &Self::Height,
    ) -> Result<Option<Client::ConsensusState>, Self::Error>;

    fn get_consensus_state_after_height(
        &self,
        client_id: &Self::ClientId,
        height: &Self::Height,
    ) -> Result<Option<Client::ConsensusState>, Self::Error>;

    fn get_consensus_state_before_height(
        &self,
        client_id: &Self::ClientId,
        height: &Self::Height,
    ) -> Result<Option<Client::ConsensusState>, Self::Error>;
}

impl<Context, Client> ClientReader<Client> for Context
where
    Client: HasClientTypes,
    Context: AnyClientReader + ContainsClient<Client> + HasClientTypeFor<Client>,
{
    fn get_client_state(
        &self,
        client_id: &Self::ClientId,
    ) -> Result<Client::ClientState, Self::Error> {
        let any_client_state = self.get_any_client_state(client_id)?;

        let client_state = Self::try_from_any_client_state(any_client_state)?;

        Ok(client_state)
    }

    fn get_consensus_state_at_height(
        &self,
        client_id: &Self::ClientId,
        height: &Self::Height,
    ) -> Result<Option<Client::ConsensusState>, Self::Error> {
        let m_consensus_state = self.get_any_consensus_state_at_height(client_id, height)?;

        match m_consensus_state {
            Some(any_consensus_state) => {
                let consensus_state = Self::try_from_any_consensus_state(any_consensus_state)?;

                Ok(Some(consensus_state))
            }
            None => Ok(None),
        }
    }

    fn get_consensus_state_after_height(
        &self,
        client_id: &Self::ClientId,
        height: &Self::Height,
    ) -> Result<Option<Client::ConsensusState>, Self::Error> {
        let m_consensus_state = self.get_any_consensus_state_after_height(client_id, height)?;

        match m_consensus_state {
            Some(any_consensus_state) => {
                let consensus_state = Self::try_from_any_consensus_state(any_consensus_state)?;

                Ok(Some(consensus_state))
            }
            None => Ok(None),
        }
    }

    fn get_consensus_state_before_height(
        &self,
        client_id: &Self::ClientId,
        height: &Self::Height,
    ) -> Result<Option<Client::ConsensusState>, Self::Error> {
        let m_consensus_state = self.get_any_consensus_state_before_height(client_id, height)?;

        match m_consensus_state {
            Some(any_consensus_state) => {
                let consensus_state = Self::try_from_any_consensus_state(any_consensus_state)?;

                Ok(Some(consensus_state))
            }
            None => Ok(None),
        }
    }
}
