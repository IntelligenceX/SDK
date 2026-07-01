use serde::{Deserialize, Serialize};

use crate::util::null_as_default;

/// A meta-data tag helping classify an item's content (language, topic, etc).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Tag {
    /// Tag class.
    pub class: i32,
    /// Tag value.
    pub value: String,
}

/// A relation between two items.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Relationship {
    /// Target item's system ID.
    pub target: uuid::Uuid,
    /// Relation type (server-defined).
    pub relation: i32,
}

/// Generic item meta-data, as used for search results.
///
/// Every field except `systemid` is optional server-side and defaults to its zero value when
/// absent *or explicitly `null`* (the API sends `null` rather than omitting some fields),
/// mirroring the OpenAPI spec's `required: [systemid]`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Item {
    /// System identifier uniquely identifying the item.
    pub systemid: uuid::Uuid,
    /// Storage identifier, empty if not stored/available.
    #[serde(default, deserialize_with = "null_as_default")]
    pub storageid: String,
    /// Whether the data of the item is in store and `storageid` is valid.
    #[serde(default, deserialize_with = "null_as_default")]
    pub instore: bool,
    /// Size in bytes of the item data.
    #[serde(default, deserialize_with = "null_as_default")]
    pub size: i64,
    /// Native access level of the item.
    #[serde(default, deserialize_with = "null_as_default")]
    pub accesslevel: i32,
    /// Low-level content type (0 = Binary, 1 = Plain text, ...).
    #[serde(rename = "type", default, deserialize_with = "null_as_default")]
    pub item_type: i32,
    /// High-level media type (1 = Paste document, ... 24 = Text file).
    #[serde(default, deserialize_with = "null_as_default")]
    pub media: i32,
    /// When the item was added to the system, as `YYYY-mm-dd HH:ii:ss` (not RFC 3339).
    #[serde(default, deserialize_with = "null_as_default")]
    pub added: String,
    /// When the item was discovered or created, as `YYYY-mm-dd HH:ii:ss` (not RFC 3339).
    #[serde(default, deserialize_with = "null_as_default")]
    pub date: String,
    /// Name or title.
    #[serde(default, deserialize_with = "null_as_default")]
    pub name: String,
    /// Full description, text only.
    #[serde(default, deserialize_with = "null_as_default")]
    pub description: String,
    /// X-Score, ranking relevancy, 0-100.
    #[serde(default, deserialize_with = "null_as_default")]
    pub xscore: i32,
    /// Simhash of the item data, used to compare similarity via Hamming distance.
    ///
    /// The OpenAPI spec documents this as `int64`, but the server actually returns the raw
    /// 64-bit hash bit pattern, which can exceed `i64::MAX`. Stored as `u64` to match (the Go
    /// SDK's `Item.Simhash` is `uint64` for the same reason).
    #[serde(default, deserialize_with = "null_as_default")]
    pub simhash: u64,
    /// Bucket identifier the item was found in.
    #[serde(default, deserialize_with = "null_as_default")]
    pub bucket: String,
    /// Meta-data tags helping in classification of the item data.
    #[serde(default, deserialize_with = "null_as_default")]
    pub tags: Vec<Tag>,
    /// Related items.
    #[serde(default, deserialize_with = "null_as_default")]
    pub relations: Vec<Relationship>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn item_deserializes_with_only_systemid_present() {
        let json = r#"{"systemid":"61202067-543e-4e6a-8c23-11f9b8f008cf"}"#;
        let item: Item = serde_json::from_str(json).unwrap();
        assert_eq!(
            item.systemid.to_string(),
            "61202067-543e-4e6a-8c23-11f9b8f008cf"
        );
        assert_eq!(item.bucket, "");
        assert_eq!(item.xscore, 0);
        assert!(item.tags.is_empty());
    }

    #[test]
    fn item_type_field_renames_to_type_keyword() {
        let json = r#"{"systemid":"61202067-543e-4e6a-8c23-11f9b8f008cf","type":5}"#;
        let item: Item = serde_json::from_str(json).unwrap();
        assert_eq!(item.item_type, 5);
    }

    #[test]
    fn item_tolerates_explicit_nulls_for_optional_fields() {
        let json = r#"{
            "systemid": "61202067-543e-4e6a-8c23-11f9b8f008cf",
            "storageid": null,
            "size": null,
            "name": null,
            "tags": null,
            "relations": null,
            "simhash": null
        }"#;
        let item: Item = serde_json::from_str(json).unwrap();
        assert_eq!(item.storageid, "");
        assert_eq!(item.size, 0);
        assert_eq!(item.name, "");
        assert!(item.tags.is_empty());
        assert!(item.relations.is_empty());
        assert_eq!(item.simhash, 0);
    }

    #[test]
    fn item_accepts_simhash_values_above_i64_max() {
        let json = r#"{
            "systemid": "61202067-543e-4e6a-8c23-11f9b8f008cf",
            "simhash": 12638153115695167422
        }"#;
        let item: Item = serde_json::from_str(json).unwrap();
        assert_eq!(item.simhash, 12638153115695167422);
    }
}
