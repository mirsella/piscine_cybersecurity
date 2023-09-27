use anyhow::Result;
use scraper::{Html, Selector};
use std::{fs, io::Write, thread};
use url::Url;

use clap::Parser;

#[derive(Debug, Parser, Clone)]
struct Args {
    #[clap(
        short,
        long,
        default_value = "false",
        help = "recursively download all images"
    )]
    recursive: bool,
    #[clap(
        short = 'l',
        long,
        default_value = "5",
        help = "maximum depth of recursion"
    )]
    depth: u32,
    #[clap(
        short,
        long,
        default_value = "./data",
        help = "where to store downloaded images"
    )]
    path: String,
    #[clap(
        short,
        long,
        default_value = "false",
        help = "scrap in parallel using thread.
might need to increase the system open file limit: `ulimit -n 500000`"
    )]
    threaded: bool,
    url: String,
}

const IMG_EXTENSIONS: [&str; 5] = ["jpg", "jpeg", "png", "gif", "bmp"];
fn download_image(args: &Args, recursivity: u32) -> Result<()> {
    println!(
        "Downloading images {}from {} to {}",
        if args.recursive { "recursively " } else { "" },
        args.url,
        args.path
    );
    let mut file_count = recursivity * 50000;
    let res = ureq::get(&args.url).call()?;
    let base_url = Url::parse(res.get_url())?;
    let doc = Html::parse_document(&res.into_string()?);
    let imgsselector = Selector::parse("img").unwrap();
    let imgs = doc.select(&imgsselector);
    for img in imgs {
        let mut src = img.value().attr("src").unwrap_or("").to_string();
        if src.is_empty() {
            continue;
        } else if src.starts_with("//") {
            src = format!("http:{}", src);
        } else if src.starts_with('/') {
            src = format!("{}{}", base_url, src);
        }
        println!("Found image: {:?}", src);
        if IMG_EXTENSIONS.iter().any(|&e| src.ends_with(e)) {
            match ureq::get(&src).call() {
                Err(e) => {
                    eprintln!("Error request {src}: {e}");
                    continue;
                }
                Ok(res) => {
                    let mut file = fs::File::create(format!(
                        "{}/{}-{}.{}",
                        args.path,
                        src.split('/').last().unwrap_or("noname"),
                        file_count,
                        src.split('.').last().unwrap_or("unknown")
                    ))?;
                    file_count += 1;
                    let mut bytes = Vec::new();
                    res.into_reader().read_to_end(&mut bytes)?;
                    file.write_all(&bytes)?;
                }
            }
        }
    }

    if args.recursive && recursivity < args.depth {
        let selector = Selector::parse("a[href^='http']").unwrap();
        let elements = doc.select(&selector);
        let mut handles = Vec::new();
        for el in elements {
            let mut cloned_args = args.clone();
            let mut src = el.value().attr("href").unwrap_or("").to_string();
            if src.is_empty() {
                continue;
            } else if src.starts_with("//") {
                src = format!("http:{}", src);
            } else if src.starts_with('/') {
                src = format!("{}{}", base_url, src);
            }
            cloned_args.url = src;
            if args.threaded {
                let handle = thread::spawn(move || {
                    if let Err(e) = download_image(&cloned_args, recursivity + 1) {
                        eprintln!("Error: {}", e);
                    };
                });
                handles.push(handle);
            } else if let Err(e) = download_image(&cloned_args, recursivity + 1) {
                eprintln!("Error: {}", e);
            }
        }
        for handle in handles {
            handle.join().unwrap();
        }
    }
    Ok(())
}

fn main() -> Result<()> {
    let args = Args::parse();
    fs::create_dir_all(&args.path)?;
    download_image(&args, 0)
}
