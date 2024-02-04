use std::path::PathBuf;

use anyhow::{bail, Result};
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct Opt {
    #[structopt(name = "URL")]
    url: String,

    #[structopt(short, long, default_value = "log.txt")]
    output: PathBuf,

    #[structopt(short = "X", long, default_value = "GET")]
    http_method: String,
}

fn main() -> Result<()> {
    let opt = Opt::from_args();
    if opt.http_method != "GET" && opt.http_method != "POST" {
        bail!("Invalid HTTP method");
    }
    dbg!(opt);
    Ok(())
}
