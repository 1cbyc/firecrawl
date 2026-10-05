use chrono::{DateTime, Utc};
use serde::Serialize;
use serde_json::Value;
use url::Url;

use super::{
  parsers::{PdfBlockItem, PdfPage},
  raw_page::{RawPageActions, ScrapeProxy},
};
use std::{collections::HashMap, fmt::Display};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum DocumentMetadataCacheState {
  Hit,
  Miss,
}

impl Display for DocumentMetadataCacheState {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::Hit => f.write_str("hit"),
      Self::Miss => f.write_str("miss"),
    }
  }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DocumentMetadata {
  pub title: Option<String>,
  pub scrape_id: String,
  pub source_url: String,
  pub url: Url,
  pub status_code: u16,
  pub num_pages: Option<u32>,
  pub total_pages: Option<u32>,
  pub content_type: String,
  pub timezone: Option<String>,
  pub proxy_used: ScrapeProxy,
  pub cache_state: DocumentMetadataCacheState,
  pub cached_at: Option<DateTime<Utc>>,
  pub index_id: Option<String>,
  pub credits_used: Option<u64>,
  pub concurrency_limited: bool,
  pub concurrency_queue_duration_ms: Option<u64>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub postprocessors_used: Option<Vec<String>>,

  pub extra: HashMap<String, Value>,
}

#[derive(Debug, Serialize)]
pub struct DocumentAttribute {
  pub selector: String,
  pub attribute: String,
  pub values: Vec<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Document {
  #[serde(skip_serializing_if = "Option::is_none")]
  pub markdown: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub html: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub raw_html: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub raw_base64: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub links: Option<Vec<String>>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub images: Option<Vec<String>>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub screenshot: Option<Url>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub audio: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub video: Option<String>,
  // videos:
  #[serde(skip_serializing_if = "Option::is_none")]
  pub extract: Option<Value>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub json: Option<Value>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub summary: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub answer: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub highlights: Option<String>,
  // branding:
  // product:
  // menu:
  #[serde(skip_serializing_if = "Option::is_none")]
  pub pages: Option<Vec<PdfPage>>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub blocks: Option<Vec<PdfBlockItem>>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub warning: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub attributes: Option<Vec<DocumentAttribute>>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub actions: Option<RawPageActions>,
  // change_tracking:
  pub metadata: DocumentMetadata,
}

impl Document {
  pub fn append_warning(&mut self, warning: impl AsRef<str>) {
    let inner = self.warning.take();
    if let Some(mut inner) = inner {
      inner += " ";
      inner += warning.as_ref();
      self.warning.replace(inner);
    } else {
      self.warning.replace(warning.as_ref().to_string());
    }
  }
}
