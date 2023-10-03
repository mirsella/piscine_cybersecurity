use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;

const FOLDER: &str = "infection";

#[derive(Parser)]
#[clap(version)]
struct Args {
    #[clap(short, long, value_name = "KEY", help = "Reverse the infection")]
    reverse: Option<String>,
    #[clap(short, long, help = "Don't print encrypted files")]
    silent: bool,
}
fn main() -> Result<()> {
    let args = Args::parse();
    let folder = dirs::home_dir().unwrap().join(FOLDER);
    if let Some(key) = args.reverse {
        reverse(&folder, key)?;
    } else {
        let key = infect(&folder)?;
    }
    Ok(())
}

fn reverse(folder: &PathBuf, key: String) -> Result<()> {
    todo!()
}

fn infect(folder: &PathBuf) -> Result<String> {
    todo!()
}
