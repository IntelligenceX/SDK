use std::ops::{Deref, DerefMut};

use serde::{Deserialize, Serialize};

use super::item::Item;
use crate::util::null_as_default;

/// A tag in human-readable form, as attached to a [`SearchResult`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PanelSearchResultTag {
    /// Tag class.
    pub class: i32,
    /// Human-friendly tag class.
    #[serde(default, deserialize_with = "null_as_default")]
    pub classh: String,
    /// Tag value.
    pub value: String,
    /// Human-friendly tag value.
    #[serde(default, deserialize_with = "null_as_default")]
    pub valueh: String,
}

/// A single search result record. Extends [`Item`] with human-readable fields.
///
/// `SearchResult` derefs to [`Item`], so `result.bucket` works directly instead of requiring
/// `result.item.bucket`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    /// The underlying item meta-data.
    #[serde(flatten)]
    pub item: Item,
    /// Human friendly access level info.
    #[serde(default, deserialize_with = "null_as_default")]
    pub accesslevelh: String,
    /// Human friendly media type info.
    #[serde(default, deserialize_with = "null_as_default")]
    pub mediah: String,
    /// Human friendly simhash.
    #[serde(default, deserialize_with = "null_as_default")]
    pub simhashh: String,
    /// Human friendly content type info.
    #[serde(default, deserialize_with = "null_as_default")]
    pub typeh: String,
    /// Human friendly tags.
    #[serde(default, deserialize_with = "null_as_default")]
    pub tagsh: Vec<PanelSearchResultTag>,
    /// Random ID assigned to this result row.
    #[serde(default)]
    pub randomid: Option<uuid::Uuid>,
    /// Human friendly bucket name.
    #[serde(default, deserialize_with = "null_as_default")]
    pub bucketh: String,
    /// File group.
    #[serde(default, deserialize_with = "null_as_default")]
    pub group: String,
    /// Index file ID (storage ID of an indexed sub-page tree view, if any).
    #[serde(default, deserialize_with = "null_as_default")]
    pub indexfile: String,
}

impl Deref for SearchResult {
    type Target = Item;

    fn deref(&self) -> &Item {
        &self.item
    }
}

impl DerefMut for SearchResult {
    fn deref_mut(&mut self) -> &mut Item {
        &mut self.item
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn search_result_flattens_item_fields_and_derefs() {
        let json = r#"{
            "systemid": "61202067-543e-4e6a-8c23-11f9b8f008cf",
            "bucket": "pastes",
            "mediah": "Paste document"
        }"#;
        let result: SearchResult = serde_json::from_str(json).unwrap();
        assert_eq!(result.bucket, "pastes");
        assert_eq!(result.mediah, "Paste document");
    }
}
