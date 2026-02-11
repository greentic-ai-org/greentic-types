//! Canonical typed schema IR (CBOR-first).
use alloc::{boxed::Box, collections::BTreeMap, string::String, vec::Vec};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use ciborium::value::Value;

/// Additional properties policy for objects.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(
    feature = "serde",
    serde(rename_all = "snake_case", tag = "type", content = "schema")
)]
pub enum AdditionalProperties {
    /// Allow additional properties.
    #[default]
    Allow,
    /// Forbid additional properties.
    Forbid,
    /// Schema for additional properties.
    Schema(Box<SchemaIr>),
}

/// Canonical schema representation.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case", tag = "type"))]
pub enum SchemaIr {
    /// Object schema.
    Object {
        /// Named properties.
        #[cfg_attr(feature = "serde", serde(default))]
        properties: BTreeMap<String, SchemaIr>,
        /// Required property names.
        #[cfg_attr(feature = "serde", serde(default))]
        required: Vec<String>,
        /// Additional properties policy.
        #[cfg_attr(feature = "serde", serde(default))]
        additional: AdditionalProperties,
    },
    /// Array schema.
    Array {
        /// Item schema.
        items: Box<SchemaIr>,
        /// Minimum length.
        #[cfg_attr(feature = "serde", serde(default))]
        min_items: Option<u64>,
        /// Maximum length.
        #[cfg_attr(feature = "serde", serde(default))]
        max_items: Option<u64>,
    },
    /// String schema.
    String {
        /// Minimum length.
        #[cfg_attr(feature = "serde", serde(default))]
        min_len: Option<u64>,
        /// Maximum length.
        #[cfg_attr(feature = "serde", serde(default))]
        max_len: Option<u64>,
        /// Regex pattern.
        #[cfg_attr(feature = "serde", serde(default))]
        regex: Option<String>,
        /// Optional format label.
        #[cfg_attr(feature = "serde", serde(default))]
        format: Option<String>,
    },
    /// Integer schema.
    Int {
        /// Minimum value.
        #[cfg_attr(feature = "serde", serde(default))]
        min: Option<i64>,
        /// Maximum value.
        #[cfg_attr(feature = "serde", serde(default))]
        max: Option<i64>,
    },
    /// Floating-point schema.
    Float {
        /// Minimum value.
        #[cfg_attr(feature = "serde", serde(default))]
        min: Option<f64>,
        /// Maximum value.
        #[cfg_attr(feature = "serde", serde(default))]
        max: Option<f64>,
    },
    /// Boolean schema.
    Bool,
    /// Null schema.
    Null,
    /// Bytes schema.
    Bytes,
    /// Enumerated values.
    Enum {
        /// Allowed values.
        #[cfg_attr(feature = "serde", serde(default))]
        values: Vec<Value>,
    },
    /// Union schema.
    OneOf {
        /// Variant schemas.
        #[cfg_attr(feature = "serde", serde(default))]
        variants: Vec<SchemaIr>,
    },
    /// Reference schema (reserved for later).
    Ref {
        /// Reference identifier.
        id: String,
    },
}
