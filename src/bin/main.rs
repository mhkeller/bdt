// Copyright 2022 Andy Grove
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use bdt::utils::{parse_filename, register_table};
use bdt::{Error};
use datafusion::prelude::*;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "bdt", about = "Boring Data Tool")]
enum Command {
    /// View schema of a file
    Schema {
        #[structopt(parse(from_os_str))]
        filename: PathBuf,
    }
}

#[tokio::main]
async fn main() {
    let cmd = Command::from_args();
    if let Err(e) = execute_command(cmd).await {
        println!("{:?}", e);
        std::process::exit(-1);
    }
}

async fn execute_command(cmd: Command) -> Result<(), Error> {
    let config = SessionConfig::new().with_information_schema(true);
    let ctx = SessionContext::new_with_config(config);
    match cmd {
        Command::Schema { filename } => {
            let filename = parse_filename(&filename)?;
            let _ = register_table(&ctx, "t", filename).await?;
            let sql = "SELECT column_name, data_type \
                                FROM information_schema.columns WHERE table_name = 't'";
            let df = ctx.sql(sql).await?;
            df.show().await?;
        }
    }
    Ok(())
}
