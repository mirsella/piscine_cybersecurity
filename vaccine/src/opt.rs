use std::{path::PathBuf, str::FromStr};
use structopt::StructOpt;

#[derive(Debug)]
pub enum HttpMethod {
    Get,
    Post,
}
impl FromStr for HttpMethod {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "GET" => Ok(HttpMethod::Get),
            "POST" => Ok(HttpMethod::Post),
            _ => Err("Unsupported HTTP method"),
        }
    }
}

#[derive(StructOpt, Debug)]
pub struct Opt {
    #[structopt(name = "URL")]
    pub url: String,

    #[structopt(short, long, default_value = "log.txt")]
    pub output: PathBuf,

    #[structopt(short = "X", long, default_value = "GET", possible_values = &["GET", "POST"])]
    pub http_method: HttpMethod,
}
