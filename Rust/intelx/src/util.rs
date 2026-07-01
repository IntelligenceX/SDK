//! Internal helpers shared across modules.

use serde::Deserialize;

/// Deserializes a field that may be missing, `null`, or present, falling back to
/// `T::default()` in the first two cases.
///
/// `#[serde(default)]` alone only covers a *missing* key; if the API sends the key with an
/// explicit JSON `null` (which the Intelligence X API does for some array/string fields),
/// deserialization still fails because `null` doesn't satisfy most types' `Deserialize` impl.
/// Pair this with `#[serde(default)]` to handle both cases:
/// `#[serde(default, deserialize_with = "crate::util::null_as_default")]`.
pub(crate) fn null_as_default<'de, D, T>(deserializer: D) -> std::result::Result<T, D::Error>
where
    D: serde::Deserializer<'de>,
    T: Default + Deserialize<'de>,
{
    Ok(Option::<T>::deserialize(deserializer)?.unwrap_or_default())
}

/// Extracts a filename from a `Content-Disposition` header value, mirroring the Python SDK's
/// `re.search(r'filename="?([^"]+)"?', cd)` used by `INTEL_EXPORT`.
///
/// Handles the common `filename="x.zip"` and `filename=x.zip` forms. RFC 5987's
/// `filename*=UTF-8''x.zip` form is also recognized as a fallback.
pub(crate) fn parse_content_disposition_filename(header: &str) -> Option<String> {
    for part in header.split(';') {
        let part = part.trim();
        if let Some(value) = part.strip_prefix("filename=") {
            let value = value.trim().trim_matches('"');
            if !value.is_empty() {
                return Some(value.to_string());
            }
        }
    }
    for part in header.split(';') {
        let part = part.trim();
        if let Some(value) = part.strip_prefix("filename*=") {
            // Strip the `UTF-8''` (or similar charset/lang) prefix per RFC 5987.
            if let Some(idx) = value.find("''") {
                let value = &value[idx + 2..];
                if !value.is_empty() {
                    return Some(value.to_string());
                }
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, PartialEq, serde::Deserialize)]
    struct NullableVec {
        #[serde(default, deserialize_with = "null_as_default")]
        items: Vec<i32>,
    }

    #[test]
    fn null_as_default_treats_explicit_null_as_default() {
        let parsed: NullableVec = serde_json::from_str(r#"{"items": null}"#).unwrap();
        assert_eq!(parsed.items, Vec::<i32>::new());
    }

    #[test]
    fn null_as_default_treats_missing_key_as_default() {
        let parsed: NullableVec = serde_json::from_str(r#"{}"#).unwrap();
        assert_eq!(parsed.items, Vec::<i32>::new());
    }

    #[test]
    fn null_as_default_passes_through_present_value() {
        let parsed: NullableVec = serde_json::from_str(r#"{"items": [1, 2, 3]}"#).unwrap();
        assert_eq!(parsed.items, vec![1, 2, 3]);
    }

    #[test]
    fn parses_quoted_filename() {
        assert_eq!(
            parse_content_disposition_filename(r#"attachment; filename="Search 2024.csv""#),
            Some("Search 2024.csv".to_string())
        );
    }

    #[test]
    fn parses_unquoted_filename() {
        assert_eq!(
            parse_content_disposition_filename("attachment; filename=export.zip"),
            Some("export.zip".to_string())
        );
    }

    #[test]
    fn parses_rfc5987_filename_star() {
        assert_eq!(
            parse_content_disposition_filename("attachment; filename*=UTF-8''export%20file.zip"),
            Some("export%20file.zip".to_string())
        );
    }

    #[test]
    fn returns_none_when_no_filename_present() {
        assert_eq!(parse_content_disposition_filename("attachment"), None);
        assert_eq!(parse_content_disposition_filename(""), None);
    }
}
