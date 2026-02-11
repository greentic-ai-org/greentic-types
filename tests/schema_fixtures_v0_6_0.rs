use std::collections::BTreeSet;

use ciborium::{de::from_reader, ser::into_writer, value::Value};
use greentic_types::cbor::canonical;
use greentic_types::contracts::wit_map::WIT_RETURNS;
use greentic_types::schemas::component::v0_6_0::{
    ComponentDescribe, ComponentQaSpec, QaMode as ComponentQaMode, schema_hash,
};
use greentic_types::schemas::pack::v0_6_0::{PackDescribe, PackQaSpec, QaMode as PackQaMode};
use greentic_types::{CborBytes, Envelope, I18nText};

#[test]
fn pack_describe_fixture_roundtrip() {
    let bytes = include_bytes!("../fixtures/pack/describe_v0_6_0.cbor");
    let cbor = CborBytes::new(bytes.to_vec());
    let spec = cbor.decode::<PackDescribe>().expect("decode pack describe");
    let encoded = canonical::to_canonical_cbor(&spec).expect("encode pack describe");
    assert_eq!(bytes.as_slice(), encoded.as_slice());
}

#[test]
fn pack_qa_fixture_roundtrip() {
    let bytes = include_bytes!("../fixtures/pack/qa_setup_v0_6_0.cbor");
    let cbor = CborBytes::new(bytes.to_vec());
    let spec = cbor.decode::<PackQaSpec>().expect("decode pack qa");
    let encoded = canonical::to_canonical_cbor(&spec).expect("encode pack qa");
    assert_eq!(bytes.as_slice(), encoded.as_slice());
    assert_eq!(spec.mode, PackQaMode::Setup);
}

#[test]
fn component_describe_fixture_roundtrip() {
    let bytes = include_bytes!("../fixtures/component/describe_v0_6_0.cbor");
    let cbor = CborBytes::new(bytes.to_vec());
    let spec = cbor
        .decode::<ComponentDescribe>()
        .expect("decode component describe");
    let encoded =
        canonical::to_canonical_cbor_allow_floats(&spec).expect("encode component describe");
    assert_eq!(bytes.as_slice(), encoded.as_slice());
}

#[test]
fn component_describe_schema_hash_matches() {
    let bytes = include_bytes!("../fixtures/component/describe_v0_6_0.cbor");
    let cbor = CborBytes::new(bytes.to_vec());
    let spec = cbor
        .decode::<ComponentDescribe>()
        .expect("decode component describe");
    let op = spec.operations.first().expect("operation");
    let computed =
        schema_hash(&op.input.schema, &op.output.schema, &spec.config_schema).expect("schema hash");
    assert_eq!(computed, op.schema_hash);
}

#[test]
fn component_qa_fixture_roundtrip() {
    let bytes = include_bytes!("../fixtures/component/qa_default_v0_6_0.cbor");
    let cbor = CborBytes::new(bytes.to_vec());
    let spec = cbor
        .decode::<ComponentQaSpec>()
        .expect("decode component qa");
    let encoded = canonical::to_canonical_cbor(&spec).expect("encode component qa");
    assert_eq!(bytes.as_slice(), encoded.as_slice());
    assert_eq!(spec.mode, ComponentQaMode::Default);
}

#[test]
fn component_describe_unknown_fields_ignored() {
    let bytes = include_bytes!("../fixtures/component/describe_v0_6_0.cbor");
    let value: Value = from_reader(bytes.as_slice()).expect("decode describe cbor");
    let mut entries = match value {
        Value::Map(entries) => entries,
        _ => panic!("expected map"),
    };
    entries.push((
        Value::Text("unknown_field".to_string()),
        Value::Text("extra".to_string()),
    ));
    let mut buf = Vec::new();
    into_writer(&Value::Map(entries), &mut buf).expect("encode with unknown field");
    let cbor = CborBytes::new(buf);
    let _spec = cbor
        .decode::<ComponentDescribe>()
        .expect("decode component describe with unknown field");
}

#[test]
fn wit_mapping_table_covers_component_exports() {
    let has_describe = WIT_RETURNS.iter().any(|entry| {
        entry.interface == "component-descriptor"
            && entry.func == "describe"
            && entry.schema_id == "greentic.component.describe@0.6.0"
    });
    let has_info = WIT_RETURNS.iter().any(|entry| {
        entry.interface == "component-descriptor"
            && entry.func == "get-component-info"
            && entry.schema_id == "greentic.component.info@0.6.0"
    });
    let has_qa = WIT_RETURNS.iter().any(|entry| {
        entry.interface == "component-qa"
            && entry.func == "qa-spec"
            && entry.schema_id == "greentic.component.qa@0.6.0"
    });
    let has_input_schema = WIT_RETURNS.iter().any(|entry| {
        entry.interface == "component-schema"
            && entry.func == "input-schema"
            && entry.schema_id == "greentic.component.schema@0.6.0"
    });
    let has_output_schema = WIT_RETURNS.iter().any(|entry| {
        entry.interface == "component-schema"
            && entry.func == "output-schema"
            && entry.schema_id == "greentic.component.schema@0.6.0"
    });
    let has_config_schema = WIT_RETURNS.iter().any(|entry| {
        entry.interface == "component-schema"
            && entry.func == "config-schema"
            && entry.schema_id == "greentic.component.schema@0.6.0"
    });
    assert!(has_describe, "missing describe mapping");
    assert!(has_info, "missing component info mapping");
    assert!(has_qa, "missing QA mapping");
    assert!(has_input_schema, "missing input-schema mapping");
    assert!(has_output_schema, "missing output-schema mapping");
    assert!(has_config_schema, "missing config-schema mapping");
}

#[test]
fn envelope_roundtrip() {
    let spec = ComponentQaSpec {
        mode: ComponentQaMode::Default,
        title: I18nText::new("component.qa.default.title", Some("Default".to_string())),
        description: None,
        questions: Vec::new(),
        defaults: Default::default(),
    };

    let envelope =
        Envelope::new("component", "greentic.component.qa@0.6.0", 6, &spec).expect("envelope");
    let decoded: ComponentQaSpec = envelope.decode_body().expect("decode envelope");
    assert_eq!(decoded.mode, ComponentQaMode::Default);
    envelope.ensure_canonical().expect("canonical body");
}

#[test]
fn qa_i18n_keys_collects_expected_set() {
    let spec = ComponentQaSpec {
        mode: ComponentQaMode::Default,
        title: I18nText::new("component.qa.default.title", Some("Default".to_string())),
        description: Some(I18nText::new(
            "component.qa.default.description",
            Some("Defaults".to_string()),
        )),
        questions: vec![],
        defaults: Default::default(),
    };

    let keys = spec.i18n_keys();
    let expected: BTreeSet<String> = [
        "component.qa.default.title",
        "component.qa.default.description",
    ]
    .into_iter()
    .map(|value| value.to_string())
    .collect();

    assert_eq!(keys, expected);
}
