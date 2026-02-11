PR-03-WIT.md — 0.6.0 Component Contract Surface (WIT ↔ CBOR Lockstep, Self-Describing)
Summary

This PR upgrades the v0.6.0 component contract so that a component’s WASM-exported describe() payload is the single source of truth for:

tool/operation list

input/output/config schemas (inline as SchemaIR)

defaults, redactions, constraints (contract metadata)

This moves Greentic components to MCP-level introspection: downstream tools can call describe() once and obtain everything needed to validate and author flows without reading manifests.

This PR does not parse WIT. Instead, it introduces an explicit, static WIT ↔ CBOR schema mapping and contract fixtures to ensure drift is automatically detected.

Goals

Make greentic.component.describe@0.6.0 fully self-describing: operations + schemas inline

Standardize SchemaIR as the canonical schema representation (CBOR-first)

Ensure deterministic/canonical encoding and stable hashes

Add contract tests proving:

fixture decodes → re-encodes byte-identical

schema hashes are stable and computed from the typed SchemaIR value

mapping table pins which WIT exports return which schema IDs

Non-Goals

No WIT parsing or signature discovery

No legacy “ComponentDescribeLegacy” types

No manifest schema authority

1) WIT Contract Mapping (no parsing)

Add src/contracts/wit_map.rs defining explicit mapping from WIT exports to schema IDs.

Contract mapping

We treat the following as authoritative for greentic:component@0.6.0 world:

component-descriptor.get-component-info returns: greentic.component.info@0.6.0

component-descriptor.describe returns: greentic.component.describe@0.6.0

component-qa.qa-spec returns: greentic.component.qa@0.6.0

component-qa.apply-answers returns: greentic.component.config@0.6.0 (CBOR config payload)

component-schema.{input,output,config}-schema are optional in this phase:

they may return the same SchemaIR values already embedded in describe (or may be redundant)

downstream should prefer describe() as authoritative

If those endpoints exist, they MUST match the embedded SchemaIR byte-for-byte after canonicalization.

If PR-03 stays type-only, encode this as a documented requirement + test helper;
full cross-call enforcement can live in greentic-component doctor later.

Note: We do not parse WIT; this mapping is hand-maintained and enforced by fixtures/tests.

Data structure
pub struct WitReturnSchema {
  pub world: &'static str,
  pub interface: &'static str,
  pub func: &'static str,
  pub schema_id: &'static str,
  pub version: u32,
}

pub const WIT_RETURNS: &[WitReturnSchema] = &[ ... ];

2) Upgrade v0.6.0 ComponentDescribe to self-describing (no legacy)

Update src/schemas/component/v0_6_0/describe.rs:

SchemaIR (canonical typed schema AST)

Introduce/expand SchemaIr under src/schemas/common/schema_ir.rs as the canonical typed schema AST.

Requirements:

deterministic structures (BTreeMap, Vec)

CBOR-friendly values (ciborium::value::Value)

additive evolution (unknown fields ignored)

supports constraints necessary for strict validation (object required fields, bounds, enums, regex, oneOf, etc.)

Minimum SchemaIr variants for 0.6.0 (non-exhaustive; enough to make schemas enforceable):

Object { properties: BTreeMap<String, SchemaIr>, required: Vec<String>, additional: AdditionalProperties }

Array { items: Box<SchemaIr>, min_items, max_items }

String { min_len, max_len, regex, format }

Int/Float { min, max }

Bool, Null, Bytes

Enum { values: Vec<Value> }

OneOf { variants: Vec<SchemaIr> }

Optional later: Ref { id }

Path C1: ComponentRunInput/Output become schema carriers

Replace placeholder maps with SchemaIR wrappers:

pub struct ComponentRunInput {
  pub schema: SchemaIr,
}

pub struct ComponentRunOutput {
  pub schema: SchemaIr,
}

Operation list (MCP-like)

Extend ComponentDescribe:

pub struct ComponentDescribe {
  pub info: ComponentInfo,
  pub provided_capabilities: Vec<String>,
  pub required_capabilities: Vec<String>,
  pub metadata: BTreeMap<String, Value>,

  /// MCP-equivalent tool list.
  pub operations: Vec<ComponentOperation>,

  /// Component-level config schema (authoritative).
  pub config_schema: SchemaIr,
}


ComponentOperation includes inline schemas and contract metadata:

pub struct ComponentOperation {
  pub id: String,
  pub display_name: Option<I18nText>,
  pub input: ComponentRunInput,
  pub output: ComponentRunOutput,

  pub defaults: BTreeMap<String, Value>,
  pub redactions: Vec<RedactionRule>,
  pub constraints: BTreeMap<String, Value>,

  /// Stable hash computed from typed SchemaIR values.
  pub schema_hash: String,
}

Redaction representation

Define a minimal RedactionRule:

path-based rules (stable and independent of SchemaIr internals)

pub struct RedactionRule {
  pub json_pointer: String,     // e.g. "/api_key"
  pub kind: RedactionKind,      // "secret" | "mask" | "drop"
}

RedactionKind is fixed for 0.6.0:

Secret (never display; treat as secret material)

Mask (display masked)

Drop (remove field from output/logs)

Represent in CBOR as snake_case strings: "secret" | "mask" | "drop"

3) Hashing + canonicalization enforcement

Add/confirm:

canonical CBOR encoder (map order, canonical integers/bytes)

schema_hash(schema: &SchemaIr) -> String hashing the typed SchemaIr value, not raw bytes

operation_schema_hash(op: &ComponentOperation, config_schema: &SchemaIr) -> String:

must include (input.schema, output.schema, config_schema) in hash material

Hash algorithm:

sha256 over a canonical CBOR encoding of a map containing the typed values:

{ input: SchemaIr, output: SchemaIr, config: SchemaIr }

4) Schema Registry updates

Update src/schema_registry.rs to include (or confirm) these schema IDs:

greentic.component.info@0.6.0

greentic.component.describe@0.6.0 ✅ now self-describing + inline SchemaIR

greentic.component.qa@0.6.0

greentic.component.config@0.6.0 (config payload produced by apply-answers)

If any already exist, update their associated Rust type paths accordingly.

5) Fixtures and contract tests (drift detection)

Add/replace fixtures to match the new describe payload shape:

fixtures/component/describe_v0_6_0.cbor

must include:

operations=[{id:"run", input.schema=..., output.schema=..., schema_hash=...}]

config_schema=...

fixtures/component/qa_default_v0_6_0.cbor (already planned in PR-01)

optional fixtures/component/info_v0_6_0.cbor

Required tests

Byte-identical fixture stability

decode fixture → struct → encode → bytes identical

Hash correctness

compute schema_hash from decoded SchemaIR values

equals embedded schema_hash

WIT mapping table coverage

assert WIT_RETURNS includes entries for:

describe → greentic.component.describe@0.6.0

qa-spec → greentic.component.qa@0.6.0

get-component-info → greentic.component.info@0.6.0

Additive evolution

test decoding with an unknown field present does not fail

Global policy: ignore unknown fields everywhere in schema structs

Do not use deny_unknown_fields; use #[serde(default)] for newly added fields and container fields,
keep optional fields as Option<_> and collections as #[serde(default)] Vec/Map.

6) Documentation updates

Add doc comments or README section:

WIT ABI is stable and versioned separately

CBOR schemas evolve via schema IDs and versions

Tools must treat WASM describe() as authoritative

Registry + fixtures guarantee lockstep without parsing WIT

Acceptance criteria

cargo test passes

Fixtures roundtrip byte-for-byte

ComponentDescribe includes operations + inline SchemaIR + config_schema

ComponentRunInput/Output use Path C1 (schema: SchemaIr)

schema_hash is stable and computed from typed SchemaIR value

Static WIT→schema mapping exists and is covered by tests

No legacy describe type introduced

Decisions (locked)

SchemaIr is the canonical typed schema AST in greentic-types; PR includes/expands it.

schema_hash covers (input, output, config) SchemaIR values.

Inline schemas in describe() are authoritative; schema endpoints must match when present.

RedactionKind is fixed: secret|mask|drop.

info_v0_6_0.cbor is optional (describe fixture is sufficient).

Unknown fields are ignored globally via serde defaults (no deny_unknown_fields).
