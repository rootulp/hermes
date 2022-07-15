use crate::alloc::string::ToString;

use ibc_proto::google::protobuf::Any;
use ibc_proto::ibc::mock::Header as RawMockHeader;
use serde_derive::{Deserialize, Serialize};
use tendermint_proto::Protobuf;

use crate::core::ics02_client::client_consensus::AnyConsensusState;
use crate::core::ics02_client::client_type::ClientType;
use crate::core::ics02_client::error::Error;
use crate::core::ics02_client::header::{AnyHeader, Header};
use crate::mock::client_state::MockConsensusState;
use crate::timestamp::Timestamp;
use crate::Height;

pub const MOCK_HEADER_TYPE_URL: &str = "/ibc.mock.Header";

#[derive(Copy, Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
pub struct MockHeader {
    pub height: Height,
    pub timestamp: Timestamp,
}

impl Default for MockHeader {
    fn default() -> Self {
        Self {
            height: Height::new(0, 1).unwrap(),
            timestamp: Default::default(),
        }
    }
}

impl Protobuf<RawMockHeader> for MockHeader {}

impl TryFrom<RawMockHeader> for MockHeader {
    type Error = Error;

    fn try_from(raw: RawMockHeader) -> Result<Self, Self::Error> {
        Ok(MockHeader {
            height: raw
                .height
                .and_then(|raw_height| raw_height.try_into().ok())
                .ok_or_else(Error::missing_raw_header)?,

            timestamp: Timestamp::from_nanoseconds(raw.timestamp)
                .map_err(Error::invalid_packet_timestamp)?,
        })
    }
}

impl From<MockHeader> for RawMockHeader {
    fn from(value: MockHeader) -> Self {
        RawMockHeader {
            height: Some(value.height.into()),
            timestamp: value.timestamp.nanoseconds(),
        }
    }
}

impl Protobuf<Any> for MockHeader {}

impl TryFrom<Any> for MockHeader {
    type Error = Error;

    fn try_from(raw: Any) -> Result<Self, Error> {
        match raw.type_url.as_str() {
            MOCK_HEADER_TYPE_URL => <MockHeader as Protobuf<Any>>::decode_vec(&raw.value)
                .map_err(Error::invalid_raw_header),
            _ => Err(Error::unknown_header_type(raw.type_url)),
        }
    }
}

impl From<MockHeader> for Any {
    fn from(value: MockHeader) -> Self {
        Any {
            type_url: MOCK_HEADER_TYPE_URL.to_string(),
            value: <MockHeader as Protobuf<Any>>::encode_vec(&value)
                .expect("encoding to `Any` from `MockHeader`"),
        }
    }
}

impl MockHeader {
    pub fn height(&self) -> Height {
        self.height
    }

    pub fn new(height: Height) -> Self {
        Self {
            height,
            timestamp: Timestamp::now(),
        }
    }

    pub fn with_timestamp(self, timestamp: Timestamp) -> Self {
        Self { timestamp, ..self }
    }
}

impl From<MockHeader> for AnyHeader {
    fn from(mh: MockHeader) -> Self {
        Self::Mock(mh)
    }
}

impl Header for MockHeader {
    fn client_type(&self) -> ClientType {
        ClientType::Mock
    }

    fn height(&self) -> Height {
        self.height
    }

    fn timestamp(&self) -> Timestamp {
        self.timestamp
    }
}

impl From<MockHeader> for AnyConsensusState {
    fn from(h: MockHeader) -> Self {
        AnyConsensusState::Mock(MockConsensusState::new(h))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_any() {
        let header = MockHeader::new(Height::new(1, 10).unwrap()).with_timestamp(Timestamp::none());
        let bytes = header.wrap_any().encode_vec().unwrap();

        assert_eq!(
            &bytes,
            &[
                10, 16, 47, 105, 98, 99, 46, 109, 111, 99, 107, 46, 72, 101, 97, 100, 101, 114, 18,
                6, 10, 4, 8, 1, 16, 10
            ]
        );
    }
}
