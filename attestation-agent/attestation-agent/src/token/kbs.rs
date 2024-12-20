// Copyright (c) 2024 Alibaba Cloud
//
// SPDX-License-Identifier: Apache-2.0
//

use crate::config::kbs::KbsConfig;

use super::GetToken;
use anyhow::*;
use async_trait::async_trait;
use kbs_protocol::{evidence_provider::NativeEvidenceProvider, KbsClientBuilder};
use serde::Serialize;
use log::{info};

#[derive(Serialize)]
struct Message {
    token: String,
    tee_keypair: String,
}

#[derive(Default)]
pub struct KbsTokenGetter {
    kbs_host_url: String,
    cert: Option<String>,
}

#[async_trait]
impl GetToken for KbsTokenGetter {
    async fn get_token(&self) -> Result<Vec<u8>> {
        info!("Initializing evidence provider...");
        let evidence_provider = Box::new(NativeEvidenceProvider::new()?);
        info!("Building KBS client with URL: {}", self.kbs_host_url);
        let mut builder =
            KbsClientBuilder::with_evidence_provider(evidence_provider, &self.kbs_host_url);

        if let Some(cert) = &self.cert {
            info!("Adding KBS certificate: {}", cert);
            builder = builder.add_kbs_cert(cert);
        }

        let mut client = builder.build()?;
        info!("Requesting token from KBS...");
        if let Err(err) = client.get_token().await {
            // Handle the error
            info!("Error fetching token in kbs.rs: {:?}", err);
            return Err(err); // Or handle it accordingly
        }
        let (token, tee_keypair) = client.get_token().await.unwrap(); // Safe here because error is handled
        
        // let (token, tee_keypair) = client.get_token().await?;
        info!("Received token: {}", token.content);
        let message = Message {
            token: token.content,
            tee_keypair: tee_keypair.to_pkcs1_pem()?.to_string(),
        };
        info!("Serializing token and TEE keypair into JSON...");
        let res = serde_json::to_vec(&message)?;
        info!("Serialized message: {:?}", res);
        Ok(res)
    }
}

impl KbsTokenGetter {
    pub fn new(config: &KbsConfig) -> Self {
        Self {
            kbs_host_url: config.url.clone(),
            cert: config.cert.clone(),
        }
    }
}
