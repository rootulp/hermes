use http::uri::Uri;
use ibc_proto::ibc::applications::fee::v1::query_client::QueryClient;
use ibc_proto::ibc::applications::fee::v1::{
    QueryCounterpartyPayeeRequest, QueryIncentivizedPacketsForChannelRequest,
};
use ibc_proto::ibc::apps::fee::v1::{
    IdentifiedPacketFees as QueriedIdentifiedPacketFees, QueryIncentivizedPacketRequest,
    QueryIncentivizedPacketsRequest,
};
use ibc_relayer_types::applications::ics29_fee::packet_fee::IdentifiedPacketFees;
use ibc_relayer_types::core::ics24_host::identifier::{ChannelId, PortId};
use ibc_relayer_types::signer::Signer;
use tonic::Code;

use crate::error::Error;

pub async fn query_counterparty_payee(
    grpc_address: &Uri,
    channel_id: &ChannelId,
    address: &Signer,
) -> Result<Option<String>, Error> {
    let mut client = QueryClient::connect(grpc_address.clone())
        .await
        .map_err(Error::grpc_transport)?;

    let request = QueryCounterpartyPayeeRequest {
        channel_id: channel_id.to_string(),
        relayer: address.to_string(),
    };

    let result = client.counterparty_payee(request).await;

    match result {
        Ok(response) => {
            let counterparty_payee = response.into_inner().counterparty_payee;

            Ok(Some(counterparty_payee))
        }
        Err(e) => {
            if e.code() == Code::NotFound {
                Ok(None)
            } else {
                Err(Error::grpc_status(e))
            }
        }
    }
}

pub async fn query_incentivized_packets(
    grpc_address: &Uri,
    channel_id: &ChannelId,
    port_id: &PortId,
) -> Result<Vec<IdentifiedPacketFees>, Error> {
    let mut client = QueryClient::connect(grpc_address.clone())
        .await
        .map_err(Error::grpc_transport)?;

    let request = QueryIncentivizedPacketsForChannelRequest {
        channel_id: channel_id.to_string(),
        port_id: port_id.to_string(),
        pagination: None,
        query_height: 0,
    };

    let response = client
        .incentivized_packets_for_channel(request)
        .await
        .map_err(Error::grpc_status)?;

    let raw_packets = response.into_inner().incentivized_packets;

    let packets = raw_packets
        .into_iter()
        .map(IdentifiedPacketFees::try_from)
        .collect::<Result<_, _>>()
        .map_err(Error::ics29)?;

    Ok(packets)
}

/// Uses the GRPC client to retrieve incentivized packet
pub async fn query_incentivized_packet(
    grpc_address: &Uri,
    request: QueryIncentivizedPacketRequest,
) -> Result<QueriedIdentifiedPacketFees, Error> {
    let mut client = QueryClient::connect(grpc_address.clone())
        .await
        .map_err(Error::grpc_transport)?;

    let request = tonic::Request::new(request);

    let response = client.incentivized_packet(request).await;

    // Querying for an account might fail, i.e. if the account doesn't actually exist
    let resp_account = match response
        .map_err(Error::grpc_status)?
        .into_inner()
        .incentivized_packet
    {
        Some(incentivized_packet) => incentivized_packet,
        None => {
            return Err(Error::event());
        }
    };

    Ok(resp_account)
}

/// Uses the GRPC client to retrieve incentivized packets
pub async fn query_all_incentivized_packets(
    grpc_address: &Uri,
    request: QueryIncentivizedPacketsRequest,
) -> Result<Vec<QueriedIdentifiedPacketFees>, Error> {
    let mut client = QueryClient::connect(grpc_address.clone())
        .await
        .map_err(Error::grpc_transport)?;

    let request = tonic::Request::new(request);

    let response = client.incentivized_packets(request).await;

    // Querying for an account might fail, i.e. if the account doesn't actually exist
    let resp_account = response
        .map_err(Error::grpc_status)?
        .into_inner()
        .incentivized_packets;

    Ok(resp_account)
}
