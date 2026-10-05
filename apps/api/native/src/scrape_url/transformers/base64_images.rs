use regex::regex;
use tracing::instrument;

use super::super::{document::Document, meta::Meta};
use super::TransformerError;

fn replace_base64_images(markdown: &str) -> String {
  regex!(r"(!\[.*?\])\(data:image/.*?;base64,.*?\)")
    .replace_all(markdown, "$1(<Base64-Image-Removed>)")
    .into_owned()
}

#[instrument(
  name = "transformers::base64_images::remove_base64_images",
  skip(meta, document),
  err
)]
pub async fn remove_base64_images(
  meta: &Meta,
  mut document: Document,
) -> Result<Document, TransformerError> {
  if meta.options.remove_base64_images
    && let Some(markdown) = document.markdown.as_deref()
  {
    document.markdown = Some(replace_base64_images(markdown));
  }

  Ok(document)
}
