use tracing::instrument;

use super::super::{
  document::Document, formats::FormatKind, meta::Meta, options::V1OriginalFormat,
};
use super::TransformerError;

/// Drops a field whose format wasn't requested, and warns when a requested one is missing.
fn coerce_field<T>(field: &mut Option<T>, requested: bool, name: &str, warn_on_remove: bool) {
  if !requested && field.is_some() {
    if warn_on_remove {
      tracing::warn!(
        field = name,
        "Removed field from Document because it wasn't in formats -- this is wasteful and indicates a bug."
      );
    }
    *field = None;
  } else if requested && field.is_none() {
    tracing::warn!(
      field = name,
      "Request had format, but there was no matching field in the result."
    );
  }
}

#[instrument(
  name = "transformers::coerce::coerce_fields_to_formats",
  skip(meta, document),
  err
)]
pub async fn coerce_fields_to_formats(
  meta: &Meta,
  mut document: Document,
) -> Result<Document, TransformerError> {
  use FormatKind::*;
  let has = |kind: FormatKind| meta.options.formats.contains(kind);
  let has_json = has(Json) || has(DeterministicJson);
  let has_answer = has(Question) || has(Query);

  coerce_field(&mut document.markdown, has(Markdown), "markdown", false);
  coerce_field(&mut document.raw_html, has(RawHtml), "rawHtml", false);
  coerce_field(&mut document.raw_base64, has(RawBase64), "rawBase64", false);
  coerce_field(&mut document.html, has(Html), "html", false);
  coerce_field(
    &mut document.screenshot,
    has(Screenshot),
    "screenshot",
    true,
  );
  coerce_field(&mut document.links, has(Links), "links", true);
  coerce_field(&mut document.images, has(Images), "images", true);

  // v1 requests keep the field named by v1OriginalFormat even without a json format.
  if !has_json {
    let v1_original_format = meta.internal_options.v1_original_format;
    if v1_original_format != Some(V1OriginalFormat::Extract) {
      coerce_field(&mut document.extract, false, "extract", true);
    }
    if v1_original_format != Some(V1OriginalFormat::Json) {
      coerce_field(&mut document.json, false, "json", true);
    }
  } else if document.extract.is_none() && document.json.is_none() {
    tracing::warn!("Request had format json, but there was no json field in the result.");
  }

  coerce_field(&mut document.summary, has(Summary), "summary", true);
  coerce_field(&mut document.answer, has_answer, "answer", true);
  coerce_field(
    &mut document.highlights,
    has(Highlights),
    "highlights",
    true,
  );
  coerce_field(&mut document.audio, has(Audio), "audio", false);
  coerce_field(&mut document.video, has(Video), "video", false);

  if meta.options.actions.is_empty() {
    document.actions = None;
  } else if let Some(actions) = &document.actions
    && actions.screenshots.is_empty()
    && actions.scrapes.is_empty()
    && actions.javascript_returns.is_empty()
    && actions.pdfs.is_empty()
  {
    document.actions = None;
  }

  Ok(document)
}
