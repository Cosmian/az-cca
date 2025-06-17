# Azure Confidential Container Attestation library

## Overview

Rust library to fetch SEV-SNP attestation, platform certificates, and UVM endorsemens through Microsoft sidecar's [attestation container](https://github.com/microsoft/confidential-sidecar-containers/tree/v2.10/cmd/attestation-container).

Default Unix Domain Socket (UDS) path is `/mnt/uds/attestation-container.sock`.

## Build

```console
cargo build
```

## Usage

```rust
use az_cca::fetch_attestation;

#[tokio::main]
async fn main() {
    let report_data = vec![0xffu8; 64];
    let attestation = fetch_attestation(
        Some("/mnt/uds/attestation-container.sock"),
        Some(&report_data),
    )
    .await;
    println!("{:?}", attestation);
}

```