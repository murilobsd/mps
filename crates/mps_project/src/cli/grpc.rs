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

use std::path;

use clap::{value_parser, Arg, ArgMatches, Command};

use super::Error;
use crate::{adapter::incoming, MpsProjectConfig};

pub fn subcommand() -> Command {
    Command::new("grpc").about("Run grpc server").arg(
        Arg::new("config")
            .short('c')
            .long("config")
            .value_name("ARQUIVO")
            .help("Caminho do arquivo de configuração")
            .value_parser(value_parser!(path::PathBuf))
            .required(true),
    )
}

pub async fn run(matches: &ArgMatches) -> Result<(), Error> {
    let config_path: &path::PathBuf =
        matches.get_one("config").expect("`config` is required");
    let project_config = MpsProjectConfig::load(config_path)?;
    incoming::grpc::server(&project_config.grpc_server).await?;
    Ok(())
}
