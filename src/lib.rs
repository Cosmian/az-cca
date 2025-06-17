pub mod attestation_container {
    tonic::include_proto!("attestation_container");
}

use std::path::PathBuf;

use hyper_util::rt::TokioIo;
#[allow(unused_imports)]
use prost::Message;
use tokio::net::UnixStream;
use tonic::transport::{Endpoint, Uri};
use tower::service_fn;

use attestation_container::{
    FetchAttestationReply, FetchAttestationRequest,
    attestation_container_client::AttestationContainerClient,
};

pub async fn fetch_attestation(
    uds_path: Option<&str>,
    report_data: Option<&[u8]>,
) -> Result<FetchAttestationReply, Box<dyn std::error::Error>> {
    let uds_path = if let Some(uds_path) = uds_path {
        PathBuf::from(uds_path)
    } else {
        PathBuf::from("/mnt/uds/attestation-container.sock")
    };
    let uds_path_r = std::sync::Arc::new(uds_path);
    let channel = Endpoint::try_from("http://[::]:50051")?
        .connect_with_connector(service_fn(move |_: Uri| {
            let uds_path_r = std::sync::Arc::clone(&uds_path_r);
            async move {
                Ok::<_, std::io::Error>(TokioIo::new(
                    UnixStream::connect(uds_path_r.as_ref()).await?,
                ))
            }
        }))
        .await?;
    let mut client = AttestationContainerClient::new(channel);

    let report_data = if let Some(bytes) = report_data {
        bytes.to_vec()
    } else {
        vec![0x00; 64]
    };

    let request = tonic::Request::new(FetchAttestationRequest { report_data });

    let response = client.fetch_attestation(request).await?;

    Ok(response.into_inner())
}
