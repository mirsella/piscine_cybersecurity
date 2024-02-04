mod opt;

use anyhow::{bail, Context, Result};
use opt::{HttpMethod, Opt};
use structopt::StructOpt;

fn main() -> Result<()> {
    let opt = Opt::from_args();
    let body = match opt.http_method {
        HttpMethod::Get => ureq::get(&opt.url),
        HttpMethod::Post => ureq::post(&opt.url),
    }
    .call()?
    .into_string()?;
    let html = tl::parse(&body, Default::default()).context("parsing body")?;
    let Some(form) = html.query_selector("form").and_then(|mut i| i.next()) else {
        bail!("No form found on {}", opt.url);
    };
    dbg!(form);
    Ok(())
}
