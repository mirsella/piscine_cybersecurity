use anyhow::{anyhow, bail, Context, Result};
use clap::ValueEnum;
use std::str::FromStr;
use url::Url;

pub struct Site {
    pub form: Form,
    pub url: Url,
}

impl Site {
    pub fn new(form: Form, url: Url) -> Self {
        Self { form, url }
    }
    /// for now only the first field of the form is used, dumb approach
    pub fn submit(&self, payload: &str) -> Result<String> {
        let mut data = vec![(self.form.fields[0].as_str(), payload)];
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
        let fields: Vec<_> = form
            .as_tag()
            .unwrap()
            .query_selector(value.parser(), "input")
            .ok_or(anyhow!("no input on form"))?
            .filter_map(|input| {
                input
                    .get(value.parser())
                    .and_then(|node| get_attribute(node, "name").ok())
            })
            .collect();
        if fields.is_empty() {
            bail!("no valid input fields found on form")
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

#[derive(Debug, Default, PartialEq, ValueEnum, Clone)]
pub enum HttpMethod {
    #[default]
    Auto,
    Get,
    Post,
}
impl ToString for HttpMethod {
    fn to_string(&self) -> String {
        match self {
            HttpMethod::Auto => "AUTO".to_string(),
            HttpMethod::Get => "GET".to_string(),
            HttpMethod::Post => "POST".to_string(),
        }
    }
}
impl FromStr for HttpMethod {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "get" => Ok(HttpMethod::Get),
            "post" => Ok(HttpMethod::Post),
            _ => Err("Unsupported HTTP method"),
        }
    }
}
