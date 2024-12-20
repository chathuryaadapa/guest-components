// Copyright (c) 2024 Alibaba Cloud
//
// SPDX-License-Identifier: Apache-2.0
//

use anyhow::Result;
use async_trait::async_trait;
use strum::EnumString;
use log::{info};

#[cfg(feature = "kbs")]
info!("build feature as kbs")
pub mod kbs;

#[cfg(feature = "coco_as")]
pub mod coco_as;

#[derive(EnumString, Clone, Copy)]
pub enum TokenType {
    #[cfg(feature = "kbs")]
    #[strum(serialize = "kbs")]
    Kbs,

    #[cfg(feature = "coco_as")]
    #[strum(serialize = "coco_as")]
    CoCoAS,
}

#[async_trait]
pub trait GetToken {
    async fn get_token(&self) -> Result<Vec<u8>>;
}
