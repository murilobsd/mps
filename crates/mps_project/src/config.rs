// Copyright (c) 2023 Murilo Ijanc' <mbsd@m0x.ru>
//
// Permission to use, copy, modify, and distribute this software for any
// purpose with or without fee is hereby granted, provided that the above
// copyright notice and this permission notice appear in all copies.
//
// THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
// WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
// MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR
// ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
// WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
// ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
// OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.

use std::path::Path;

use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct DatabaseConfig {
    pub uri: String,
    pub max_pool: usize,
}

#[derive(Debug, Clone, Deserialize)]
pub struct KafkaConfig {
    pub ip: String,
    pub port: u16,
    pub timeout: u64,
    pub topic: String,
}

impl KafkaConfig {
    pub fn address(&self) -> String {
        format!("{}:{}", self.ip, self.port)
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct MpsProjectConfig {
    pub log_level: String,
    pub database: DatabaseConfig,
    pub kafka: KafkaConfig,
    pub grpc_server: crate::grpc::GrpcConfig,
}

impl MpsProjectConfig {
    pub fn load<P: AsRef<Path>>(
        config_path: P,
    ) -> Result<Self, crate::Error> {
        Ok(mps_config::load(config_path)?)
    }
}
