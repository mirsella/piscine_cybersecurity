mod injection;
mod site;

use std::path::PathBuf;

use crate::site::HttpMethod;
use anyhow::{Context, Result};
use clap::Parser;
use log2::{debug, info, warn};
use site::{Form, Site};
use url::Url;

#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Args {
    #[arg(name = "URL")]
    pub url: Url,

    #[arg(short, long, default_value = "log.txt")]
    pub output: PathBuf,

    #[arg(short = 'X', long, value_enum, default_value_t)]
    pub http_method: HttpMethod,

    #[arg(short, long, default_value = "debug")]
    pub verbose: log2::level,
}

fn main() -> Result<()> {
    let args = Args::parse();
    info!("running on {} with {:?}", args.url, args.http_method);
    let _log2 = log2::open(&args.output.to_string_lossy())
        .tee(true)
        .module(true)
        .level(args.verbose)
        .start();
    info!("logging to {:?}", args.output);
    let body = ureq::get(args.url.as_str()).call()?.into_string()?;
    let html = tl::parse(&body, Default::default()).context("parsing html")?;
    let mut form: Form = html.try_into()?;
    debug!("detected this form: {:?}", form);
    if args.http_method != HttpMethod::Auto {
        warn!("overriding form method with {:?}", args.http_method);
        form.method = args.http_method;
    };
    let site = Site::new(form, args.url);
    injection::test(&site)?;
    Ok(())
}
