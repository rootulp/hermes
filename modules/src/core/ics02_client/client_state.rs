use core::marker::{Send, Sync};
use core::time::Duration;

use ibc_proto::google::protobuf::Any;
use serde::{Deserialize, Serialize};
use tendermint_proto::Protobuf;

use ibc_proto::ibc::core::client::v1::IdentifiedClientState;

use crate::clients::ics07_tendermint::client_state::{
    ClientState as TendermintClientState, UpgradeOptions as TendermintUpgradeOptions,
};
use crate::core::ics02_client::client_type::ClientType;
use crate::core::ics02_client::error::Error;
use crate::core::ics02_client::trust_threshold::TrustThreshold;
use crate::core::ics24_host::error::ValidationError;
use crate::core::ics24_host::identifier::{ChainId, ClientId};
#[cfg(any(test, feature = "mocks"))]
use crate::mock::client_state::MockClientState;
use crate::prelude::*;
use crate::Height;

pub const TENDERMINT_CLIENT_STATE_TYPE_URL: &str = "/ibc.lightclients.tendermint.v1.ClientState";
pub const MOCK_CLIENT_STATE_TYPE_URL: &str = "/ibc.mock.ClientState";

pub trait ClientState: Clone + core::fmt::Debug + Send + Sync {
    /// Client-specific options for upgrading the client
    type UpgradeOptions;

    /// Return the chain identifier which this client is serving (i.e., the client is verifying
    /// consensus states from this chain).
    fn chain_id(&self) -> ChainId;

    /// Type of client associated with this state (eg. Tendermint)
    fn client_type(&self) -> ClientType;

    /// Latest height of consensus state
    fn latest_height(&self) -> Height;

    /// Freeze status of the client
    fn is_frozen(&self) -> bool {
        self.frozen_height().is_some()
    }

    /// Frozen height of the client
    fn frozen_height(&self) -> Option<Height>;

    /// Helper function to verify the upgrade client procedure.
    /// Resets all fields except the blockchain-specific ones,
    /// and updates the given fields.
    fn upgrade(
        self,
        upgrade_height: Height,
        upgrade_options: Self::UpgradeOptions,
        chain_id: ChainId,
    ) -> Self;
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum AnyUpgradeOptions {
    Tendermint(TendermintUpgradeOptions),

    #[cfg(any(test, feature = "mocks"))]
    Mock(()),
}

impl AnyUpgradeOptions {
    fn into_tendermint(self) -> TendermintUpgradeOptions {
        match self {
            Self::Tendermint(options) => options,

            #[cfg(any(test, feature = "mocks"))]
            Self::Mock(_) => {
                panic!("cannot downcast AnyUpgradeOptions::Mock to Tendermint::UpgradeOptions")
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum GenericClientState<Extra> {
    Tendermint(TendermintClientState),

    #[cfg(any(test, feature = "mocks"))]
    Mock(MockClientState),

    Extra(Extra),
}

impl<Extra> From<TendermintClientState> for GenericClientState<Extra> {
    fn from(state: TendermintClientState) -> Self {
        GenericClientState::Tendermint(state)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum NoExtra {}

pub type AnyClientState = GenericClientState<NoExtra>;

pub trait SomeClientState {
    fn trust_threshold(&self) -> Option<TrustThreshold>;

    fn max_clock_drift(&self) -> Duration;

    fn refresh_period(&self) -> Option<Duration>;

    fn expired(&self, elapsed_since_latest: Duration) -> bool;
}

impl<Extra: SomeClientState> SomeClientState for GenericClientState<Extra> {
    fn trust_threshold(&self) -> Option<TrustThreshold> {
        match self {
            Self::Tendermint(state) => Some(state.trust_level),

            #[cfg(any(test, feature = "mocks"))]
            Self::Mock(_) => None,

            Self::Extra(extra) => extra.trust_threshold(),
        }
    }

    fn max_clock_drift(&self) -> Duration {
        match self {
            Self::Tendermint(state) => state.max_clock_drift,

            #[cfg(any(test, feature = "mocks"))]
            Self::Mock(_) => Duration::new(0, 0),

            Self::Extra(extra) => extra.max_clock_drift(),
        }
    }

    fn refresh_period(&self) -> Option<Duration> {
        match self {
            Self::Tendermint(tm_state) => tm_state.refresh_time(),

            #[cfg(any(test, feature = "mocks"))]
            Self::Mock(mock_state) => mock_state.refresh_time(),

            Self::Extra(extra) => extra.refresh_period(),
        }
    }

    fn expired(&self, elapsed_since_latest: Duration) -> bool {
        match self {
            Self::Tendermint(tm_state) => tm_state.expired(elapsed_since_latest),

            #[cfg(any(test, feature = "mocks"))]
            Self::Mock(mock_state) => mock_state.expired(elapsed_since_latest),

            Self::Extra(extra) => extra.expired(elapsed_since_latest),
        }
    }
}

impl SomeClientState for NoExtra {
    fn trust_threshold(&self) -> Option<TrustThreshold> {
        match *self {}
    }

    fn max_clock_drift(&self) -> Duration {
        match *self {}
    }

    fn refresh_period(&self) -> Option<Duration> {
        match *self {}
    }

    fn expired(&self, _elapsed_since_latest: Duration) -> bool {
        match *self {}
    }
}

impl<Extra> Protobuf<Any> for GenericClientState<Extra>
where
    Extra: TryFrom<Any, Error = Error>,
    Extra: Protobuf<Any>,
    Any: From<Extra>,
{
}

impl Protobuf<Any> for NoExtra {}

impl<Extra> TryFrom<Any> for GenericClientState<Extra>
where
    Extra: TryFrom<Any, Error = Error>,
{
    type Error = Error;

    fn try_from(raw: Any) -> Result<Self, Self::Error> {
        match raw.type_url.as_str() {
            "" => Err(Error::empty_client_state_response()),

            TENDERMINT_CLIENT_STATE_TYPE_URL => Ok(GenericClientState::Tendermint(
                TendermintClientState::decode_vec(&raw.value)
                    .map_err(Error::decode_raw_client_state)?,
            )),

            #[cfg(any(test, feature = "mocks"))]
            MOCK_CLIENT_STATE_TYPE_URL => Ok(GenericClientState::Mock(
                MockClientState::decode_vec(&raw.value).map_err(Error::decode_raw_client_state)?,
            )),

            _ => {
                let extra = Extra::try_from(raw)?;
                Ok(GenericClientState::Extra(extra))
            }
        }
    }
}

impl TryFrom<Any> for NoExtra {
    type Error = Error;
    fn try_from(raw: Any) -> Result<Self, Error> {
        Err(Error::unknown_client_state_type(raw.type_url))
    }
}

impl<Extra> From<GenericClientState<Extra>> for Any
where
    Any: From<Extra>,
{
    fn from(value: GenericClientState<Extra>) -> Self {
        match value {
            GenericClientState::Tendermint(value) => Any {
                type_url: TENDERMINT_CLIENT_STATE_TYPE_URL.to_string(),
                value: value
                    .encode_vec()
                    .expect("encoding to `Any` from `AnyClientState::Tendermint`"),
            },
            #[cfg(any(test, feature = "mocks"))]
            GenericClientState::Mock(value) => Any {
                type_url: MOCK_CLIENT_STATE_TYPE_URL.to_string(),
                value: value
                    .encode_vec()
                    .expect("encoding to `Any` from `AnyClientState::Mock`"),
            },
            GenericClientState::Extra(extra) => Any::from(extra),
        }
    }
}

impl From<NoExtra> for Any {
    fn from(value: NoExtra) -> Self {
        match value {}
    }
}

impl<Extra> ClientState for GenericClientState<Extra>
where
    Extra: ClientState<UpgradeOptions = AnyUpgradeOptions>,
{
    type UpgradeOptions = AnyUpgradeOptions;

    fn chain_id(&self) -> ChainId {
        match self {
            Self::Tendermint(tm_state) => tm_state.chain_id(),

            #[cfg(any(test, feature = "mocks"))]
            Self::Mock(mock_state) => mock_state.chain_id(),

            Self::Extra(extra) => extra.chain_id(),
        }
    }

    fn client_type(&self) -> ClientType {
        match self {
            Self::Tendermint(state) => state.client_type(),

            #[cfg(any(test, feature = "mocks"))]
            Self::Mock(state) => state.client_type(),

            Self::Extra(extra) => extra.client_type(),
        }
    }

    fn latest_height(&self) -> Height {
        match self {
            Self::Tendermint(tm_state) => tm_state.latest_height(),

            #[cfg(any(test, feature = "mocks"))]
            Self::Mock(mock_state) => mock_state.latest_height(),

            Self::Extra(extra) => extra.latest_height(),
        }
    }

    fn frozen_height(&self) -> Option<Height> {
        match self {
            Self::Tendermint(tm_state) => tm_state.frozen_height(),

            #[cfg(any(test, feature = "mocks"))]
            Self::Mock(mock_state) => mock_state.frozen_height(),

            Self::Extra(extra) => extra.frozen_height(),
        }
    }

    fn upgrade(
        self,
        upgrade_height: Height,
        upgrade_options: Self::UpgradeOptions,
        chain_id: ChainId,
    ) -> Self {
        match self {
            GenericClientState::Tendermint(tm_state) => {
                let upgraded_state =
                    tm_state.upgrade(upgrade_height, upgrade_options.into_tendermint(), chain_id);
                GenericClientState::Tendermint(upgraded_state)
            }

            #[cfg(any(test, feature = "mocks"))]
            GenericClientState::Mock(mock_state) => {
                let upgraded_state = mock_state.upgrade(upgrade_height, (), chain_id);
                GenericClientState::Mock(upgraded_state)
            }

            GenericClientState::Extra(extra) => {
                let upgraded_state = extra.upgrade(upgrade_height, upgrade_options, chain_id);
                GenericClientState::Extra(upgraded_state)
            }
        }
    }
}

impl ClientState for NoExtra {
    type UpgradeOptions = AnyUpgradeOptions;

    fn chain_id(&self) -> ChainId {
        match *self {}
    }

    fn client_type(&self) -> ClientType {
        match *self {}
    }

    fn latest_height(&self) -> Height {
        match *self {}
    }

    fn frozen_height(&self) -> Option<Height> {
        match *self {}
    }

    fn upgrade(
        self,
        _upgrade_height: Height,
        _upgrade_options: Self::UpgradeOptions,
        _chain_id: ChainId,
    ) -> Self {
        match self {}
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub struct IdentifiedAnyClientState {
    pub client_id: ClientId,
    pub client_state: AnyClientState,
}

impl IdentifiedAnyClientState {
    pub fn new(client_id: ClientId, client_state: AnyClientState) -> Self {
        IdentifiedAnyClientState {
            client_id,
            client_state,
        }
    }
}

impl Protobuf<IdentifiedClientState> for IdentifiedAnyClientState {}

impl TryFrom<IdentifiedClientState> for IdentifiedAnyClientState {
    type Error = Error;

    fn try_from(raw: IdentifiedClientState) -> Result<Self, Self::Error> {
        Ok(IdentifiedAnyClientState {
            client_id: raw.client_id.parse().map_err(|e: ValidationError| {
                Error::invalid_raw_client_id(raw.client_id.clone(), e)
            })?,
            client_state: raw
                .client_state
                .ok_or_else(Error::missing_raw_client_state)?
                .try_into()?,
        })
    }
}

impl From<IdentifiedAnyClientState> for IdentifiedClientState {
    fn from(value: IdentifiedAnyClientState) -> Self {
        IdentifiedClientState {
            client_id: value.client_id.to_string(),
            client_state: Some(value.client_state.into()),
        }
    }
}

#[cfg(test)]
mod tests {

    use ibc_proto::google::protobuf::Any;
    use test_log::test;

    use crate::clients::ics07_tendermint::client_state::test_util::get_dummy_tendermint_client_state;
    use crate::clients::ics07_tendermint::header::test_util::get_dummy_tendermint_header;
    use crate::core::ics02_client::client_state::AnyClientState;

    #[test]
    fn any_client_state_serialization() {
        let tm_client_state = get_dummy_tendermint_client_state(get_dummy_tendermint_header());

        let raw: Any = tm_client_state.clone().into();
        let tm_client_state_back = AnyClientState::try_from(raw).unwrap();
        assert_eq!(tm_client_state, tm_client_state_back);
    }
}
