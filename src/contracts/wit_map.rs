//! Static WIT return schema mapping for greentic component contracts.

/// Mapping entry describing which WIT export returns which schema.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct WitReturnSchema {
    /// WIT world name (versioned).
    pub world: &'static str,
    /// WIT interface name.
    pub interface: &'static str,
    /// WIT function name.
    pub func: &'static str,
    /// Schema identifier.
    pub schema_id: &'static str,
    /// Schema version.
    pub version: u32,
}

/// Mapping table for greentic:component@0.6.0.
pub const WIT_RETURNS: &[WitReturnSchema] = &[
    WitReturnSchema {
        world: "greentic:component@0.6.0",
        interface: "component-descriptor",
        func: "get-component-info",
        schema_id: "greentic.component.info@0.6.0",
        version: 6,
    },
    WitReturnSchema {
        world: "greentic:component@0.6.0",
        interface: "component-descriptor",
        func: "describe",
        schema_id: "greentic.component.describe@0.6.0",
        version: 6,
    },
    WitReturnSchema {
        world: "greentic:component@0.6.0",
        interface: "component-qa",
        func: "qa-spec",
        schema_id: "greentic.component.qa@0.6.0",
        version: 6,
    },
    WitReturnSchema {
        world: "greentic:component@0.6.0",
        interface: "component-qa",
        func: "apply-answers",
        schema_id: "greentic.component.config@0.6.0",
        version: 6,
    },
    WitReturnSchema {
        world: "greentic:component@0.6.0",
        interface: "component-schema",
        func: "input-schema",
        schema_id: "greentic.component.schema@0.6.0",
        version: 6,
    },
    WitReturnSchema {
        world: "greentic:component@0.6.0",
        interface: "component-schema",
        func: "output-schema",
        schema_id: "greentic.component.schema@0.6.0",
        version: 6,
    },
    WitReturnSchema {
        world: "greentic:component@0.6.0",
        interface: "component-schema",
        func: "config-schema",
        schema_id: "greentic.component.schema@0.6.0",
        version: 6,
    },
];
