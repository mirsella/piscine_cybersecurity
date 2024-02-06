use anyhow::{anyhow, bail, Context, Result};
use clap::ValueEnum;
use log2::{trace, warn};
use strum::{Display, EnumString};
use url::Url;

#[derive(Debug)]
pub struct Site {
    pub form: Form,
    pub url: Url,
    pub db: Db,
}

impl Site {
    pub fn new(form: Form, url: Url) -> Self {
        Self {
            form,
            url,
            db: Default::default(),
        }
    }
    /// for now only the first field of the form is used, dumb approach
    pub fn submit(&self, payload: impl AsRef<str>) -> Result<String> {
        let mut data = vec![(self.form.fields[0].as_str(), payload.as_ref())];
        for field in self.form.fields.iter().skip(1) {
            data.push((field.as_str(), ""));
        }
        let res = match self.form.method {
            HttpMethod::Get | HttpMethod::Auto => ureq::get,
            HttpMethod::Post => ureq::post,
        }(self.url.as_str())
        .send_form(&data)
        .context("sending form")?;
        Ok(res.into_string()?)
    }
}

/// fields is guaranteed to have at least one element
#[derive(Debug)]
#[non_exhaustive]
pub struct Form {
    pub endpoint: String,
    pub method: HttpMethod,
    pub fields: Vec<String>,
}

impl TryFrom<tl::VDom<'_>> for Form {
    type Error = anyhow::Error;

    fn try_from(value: tl::VDom) -> anyhow::Result<Self> {
        let form = value
            .query_selector("form")
            .and_then(|mut i| i.next())
            .ok_or(anyhow!("form not found"))?
            .get(value.parser())
            .unwrap();
        let endpoint = get_attribute(form, "action")
            .context("couldn't find the action attribute on the form")?;
        let method = get_attribute(form, "method")
            .map_or_else(|_| Default::default(), |s| s.parse().unwrap_or_default());
        if method == HttpMethod::Auto {
            warn!("no method found, using GET. this is a sign this site will probably not work");
        }
        let fields: Vec<_> = form
            .as_tag()
            .unwrap()
            .query_selector(value.parser(), "input[type='text'], textarea")
            .ok_or(anyhow!("invalid query selector"))?
            .filter_map(|input| {
                input.get(value.parser()).and_then(|node| {
                    trace!("found input {:?}", node.as_tag().map(|t| t.name()));
                    get_attribute(node, "name").ok()
                })
            })
            .collect();
        if fields.is_empty() {
            bail!("no valid input fields found on form. either a invalid form format or because of missing javascript")
        }
        Ok(Self {
            endpoint,
            method,
            fields,
        })
    }
}

fn get_attribute(tag: &tl::Node, name: &str) -> Result<String> {
    tag.as_tag()
        .ok_or(anyhow!("Node is not a Node::tag"))?
        .attributes()
        .iter()
        .find(|(k, _)| k == name)
        .and_then(|(_, v)| v.map(|v| v.to_string()))
        .ok_or(anyhow!("{} attribute not found", name))
}

#[derive(Debug, Default, PartialEq, ValueEnum, Clone, Display, EnumString)]
#[strum(ascii_case_insensitive)]
pub enum HttpMethod {
    #[default]
    Auto,
    Get,
    Post,
}

#[derive(Debug, Default, EnumString, Display, Copy, Clone)]
#[strum(ascii_case_insensitive)]
pub enum Db {
    #[default]
    Unknown,
    Mysql,
    Sqlite,
}
