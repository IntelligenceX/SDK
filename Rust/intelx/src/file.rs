//! File operations: `/file/preview`, `/file/view`, `/file/read`.

use std::path::Path;

use crate::client::IntelXClient;
use crate::error::{IntelXError, Result, api_error_from_status};

/// `type` parameter for [`IntelXClient::file_read`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileReadType {
    /// No content disposition. Returns the raw binary file.
    Raw = 0,
    /// Content disposition; may fix line endings to CRLF for text files.
    ContentDisposition = 1,
}

/// Parameters for [`IntelXClient::file_preview`], mirroring the Python SDK's `FILE_PREVIEW()`.
#[derive(Debug, Clone)]
pub struct FilePreviewParams {
    /// Low-level content type (`Item::item_type`).
    pub ctype: i32,
    /// High-level media type (`Item::media`).
    pub mediatype: i32,
    /// Preview format: `0` = text, `1` = picture.
    pub format: i32,
    /// Storage ID of the item to preview.
    pub sid: String,
    /// Bucket the item was found in. Defaults to empty.
    pub bucket: String,
    /// `0` = don't escape HTML, `1` = default (escape).
    pub escape: i32,
    /// Maximum number of lines to return. Defaults to `8`.
    pub lines: i32,
}

impl FilePreviewParams {
    /// Creates preview parameters with the same defaults as the Python SDK's `FILE_PREVIEW()`.
    pub fn new(ctype: i32, mediatype: i32, format: i32, sid: impl Into<String>) -> Self {
        Self {
            ctype,
            mediatype,
            format,
            sid: sid.into(),
            bucket: String::new(),
            escape: 0,
            lines: 8,
        }
    }

    /// Sets the bucket the item was found in.
    pub fn bucket(mut self, bucket: impl Into<String>) -> Self {
        self.bucket = bucket.into();
        self
    }

    /// Sets the maximum number of lines to return.
    pub fn lines(mut self, lines: i32) -> Self {
        self.lines = lines;
        self
    }
}

/// Picks the `/file/view` format code for a given content/media type, mirroring the Python
/// SDK's `FILE_VIEW()` if/elif branch.
///
/// Pulled out as a pure function so the format-selection logic is unit-testable without an
/// HTTP call.
pub(crate) fn resolve_view_format(ctype: i32, mediatype: i32) -> i32 {
    match mediatype {
        23 | 9 => 7,          // HTML
        15 => 6,              // PDF
        16 => 8,              // Word
        18 => 10,             // PowerPoint
        25 => 11,             // Ebook
        17 => 9,              // Excel
        _ if ctype == 1 => 0, // Text
        _ => 1,               // Hex view fallback
    }
}

impl IntelXClient {
    /// Shows a preview of a file's contents based on its storage ID. Previews are capped at
    /// 1000 characters server-side. Mirrors the Python SDK's `FILE_PREVIEW()`.
    pub async fn file_preview(&self, params: FilePreviewParams) -> Result<String> {
        self.rate_limit_sleep().await;
        let response = self
            .get_response(
                "/file/preview",
                &[
                    ("c", params.ctype.to_string()),
                    ("m", params.mediatype.to_string()),
                    ("f", params.format.to_string()),
                    ("sid", params.sid),
                    ("b", params.bucket),
                    ("e", params.escape.to_string()),
                    ("l", params.lines.to_string()),
                ],
            )
            .await?;
        Ok(response.text().await?)
    }

    /// Shows a file's contents based on its storage ID, auto-selecting the view format from
    /// the item's content/media type. Mirrors the Python SDK's `FILE_VIEW()`.
    pub async fn file_view(
        &self,
        ctype: i32,
        mediatype: i32,
        sid: &str,
        bucket: &str,
        escape: i32,
    ) -> Result<String> {
        self.rate_limit_sleep().await;
        let format = resolve_view_format(ctype, mediatype);
        let response = self
            .get_response(
                "/file/view",
                &[
                    ("f", format.to_string()),
                    ("storageid", sid.to_string()),
                    ("bucket", bucket.to_string()),
                    ("escape", escape.to_string()),
                ],
            )
            .await?;
        Ok(response.text().await?)
    }

    /// Reads a file's raw contents and streams it to `dest`. Use this for direct data download.
    /// Returns the number of bytes written. Mirrors the Python SDK's `FILE_READ()`.
    pub async fn file_read(
        &self,
        system_id: &str,
        kind: FileReadType,
        bucket: &str,
        dest: &Path,
    ) -> Result<u64> {
        self.rate_limit_sleep().await;
        let response = self
            .get_response(
                "/file/read",
                &[
                    ("type", (kind as i32).to_string()),
                    ("systemid", system_id.to_string()),
                    ("bucket", bucket.to_string()),
                ],
            )
            .await?;

        if !response.status().is_success() {
            return Err(api_error_from_status(response.status()));
        }

        let bytes = response.bytes().await?;
        tokio::fs::write(dest, &bytes).await?;
        Ok(bytes.len() as u64)
    }

    /// Shows a treeview of an item that has multiple files/folders. Returns `None` if the
    /// server reports it could not generate a tree (matching Python's `"Could not generate"`
    /// substring check). Mirrors the Python SDK's `FILE_TREE_VIEW()`.
    pub async fn file_tree_view(&self, sid: &str) -> Result<Option<String>> {
        self.rate_limit_sleep().await;
        let response = self
            .http_get_with_timeout(
                "/file/view",
                &[("f", "12".to_string()), ("storageid", sid.to_string())],
                5,
            )
            .await?;
        let text = response.text().await?;
        if text.contains("Could not generate") {
            Ok(None)
        } else {
            Ok(Some(text))
        }
    }

    /// Fetches the tree view for an item as JSON. Use the storage ID from a search result's
    /// `historyfile` (historical website copies) or `indexfile` (indexed sub-pages) field.
    /// Mirrors the Python SDK's `treeview()`.
    pub async fn treeview(&self, id: &str, bucket: &str) -> Result<serde_json::Value> {
        self.rate_limit_sleep().await;
        self.get(
            "/file/view",
            &[
                ("f", "13".to_string()),
                ("storageid", id.to_string()),
                ("bucket", bucket.to_string()),
            ],
        )
        .await
    }

    async fn http_get_with_timeout(
        &self,
        path: &str,
        query: &[(&str, String)],
        timeout_secs: u64,
    ) -> Result<reqwest::Response> {
        match tokio::time::timeout(
            std::time::Duration::from_secs(timeout_secs),
            self.get_response(path, query),
        )
        .await
        {
            Ok(result) => result,
            Err(elapsed) => Err(IntelXError::Timeout(elapsed)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolve_view_format_matches_python_branches() {
        assert_eq!(resolve_view_format(0, 23), 7); // HTML by media
        assert_eq!(resolve_view_format(0, 9), 7); // HTML copy of website
        assert_eq!(resolve_view_format(0, 15), 6); // PDF
        assert_eq!(resolve_view_format(0, 16), 8); // Word
        assert_eq!(resolve_view_format(0, 18), 10); // PowerPoint
        assert_eq!(resolve_view_format(0, 25), 11); // Ebook
        assert_eq!(resolve_view_format(0, 17), 9); // Excel
        assert_eq!(resolve_view_format(1, 0), 0); // ctype == 1 -> text
        assert_eq!(resolve_view_format(0, 0), 1); // fallback -> hex
    }
}
