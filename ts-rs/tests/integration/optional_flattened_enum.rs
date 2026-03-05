#![allow(dead_code)]

#[cfg(feature = "serde-compat")]
use serde::Serialize;
use ts_rs::{Config, TS};

#[test]
fn two_variant_enum() {
    #[derive(TS)]
    #[cfg_attr(feature = "serde-compat", derive(Serialize))]
    #[cfg_attr(feature = "serde-compat", serde(rename_all = "camelCase"))]
    #[cfg_attr(not(feature = "serde-compat"), ts(rename_all = "camelCase"))]
    enum Enum {
        FirstOption(String),
        SecondOption(bool),
    }

    #[derive(TS)]
    #[cfg_attr(feature = "serde-compat", derive(Serialize))]
    struct T {
        a: String,
        #[cfg_attr(feature = "serde-compat", serde(flatten))]
        #[cfg_attr(not(feature = "serde-compat"), ts(flatten))]
        flattened: Option<Enum>,
    }

    let cfg = Config::default();

    assert_eq!(
        T::optional_inline_flattened(&cfg),
        r#"{ a: string, } & ({ "firstOption": string; "secondOption"?: never } | { "secondOption": boolean; "firstOption"?: never } | { "firstOption"?: never; "secondOption"?: never })"#
    );
}

#[test]
fn three_variant_enum() {
    #[derive(TS)]
    #[cfg_attr(feature = "serde-compat", derive(Serialize))]
    #[cfg_attr(feature = "serde-compat", serde(rename_all = "camelCase"))]
    #[cfg_attr(not(feature = "serde-compat"), ts(rename_all = "camelCase"))]
    enum Enum {
        FirstOption(String),
        SecondOption(bool),
        ThirdOption(usize),
    }

    #[derive(TS)]
    #[cfg_attr(feature = "serde-compat", derive(Serialize))]
    struct T {
        a: String,
        #[cfg_attr(feature = "serde-compat", serde(flatten))]
        #[cfg_attr(not(feature = "serde-compat"), ts(flatten))]
        flattened: Option<Enum>,
    }

    let cfg = Config::default();

    assert_eq!(
        T::optional_inline_flattened(&cfg),
        r#"{ a: string, } & ({ "firstOption": string; "secondOption"?: never; "thirdOption"?: never } | { "secondOption": boolean; "firstOption"?: never; "thirdOption"?: never } | { "thirdOption": number; "firstOption"?: never; "secondOption"?: never } | { "firstOption"?: never; "secondOption"?: never; "thirdOption"?: never })"#
    );
}

#[test]
#[should_panic(expected = "Enum cannot be flattened")]
fn unit_variants() {
    #[derive(TS)]
    #[cfg_attr(feature = "serde-compat", derive(Serialize))]
    #[cfg_attr(feature = "serde-compat", serde(rename_all = "camelCase"))]
    #[cfg_attr(not(feature = "serde-compat"), ts(rename_all = "camelCase"))]
    enum Enum {
        First,
        Second,
        Third,
    }

    #[derive(TS)]
    #[cfg_attr(feature = "serde-compat", derive(Serialize))]
    struct T {
        a: String,
        #[cfg_attr(feature = "serde-compat", serde(flatten))]
        #[cfg_attr(not(feature = "serde-compat"), ts(flatten))]
        status: Option<Enum>,
    }

    let cfg = Config::default();

    // Unit variants can't be properly typed as optional flattened
    assert_eq!(
        T::optional_inline_flattened(&cfg),
        r#"{ a: string, } & ("first" | "second" | "third" | { "first"?: never; "second"?: never; "third"?: never })"#
    );
}

#[test]
#[should_panic(expected = "Enum cannot be flattened")]
fn mixed_variant_types() {
    #[derive(TS)]
    #[cfg_attr(feature = "serde-compat", derive(Serialize))]
    #[cfg_attr(feature = "serde-compat", serde(rename_all = "camelCase"))]
    #[cfg_attr(not(feature = "serde-compat"), ts(rename_all = "camelCase"))]
    enum Enum {
        Unit,
        Tuple(i32, String),
        Struct { x: i32, y: String },
    }

    #[derive(TS)]
    #[cfg_attr(feature = "serde-compat", derive(Serialize))]
    struct T {
        a: String,
        #[cfg_attr(feature = "serde-compat", serde(flatten))]
        #[cfg_attr(not(feature = "serde-compat"), ts(flatten))]
        data: Option<Enum>,
    }

    let cfg = Config::default();

    // Mixed variants can't be properly typed
    assert_eq!(
        T::optional_inline_flattened(&cfg),
        r#"{ a: string, } & ("unit" | { "tuple": [number, string]; "unit"?: never; "struct"?: never } | { "struct": { x: number, y: string, }; "unit"?: never; "tuple"?: never } | { "unit"?: never; "tuple"?: never; "struct"?: never })"#
    );
}

#[test]
fn nested_structs() {
    #[derive(TS)]
    #[cfg_attr(feature = "serde-compat", derive(Serialize))]
    struct Inner {
        value: i32,
    }

    #[derive(TS)]
    #[cfg_attr(feature = "serde-compat", derive(Serialize))]
    #[cfg_attr(feature = "serde-compat", serde(rename_all = "camelCase"))]
    #[cfg_attr(not(feature = "serde-compat"), ts(rename_all = "camelCase"))]
    enum Enum {
        First(Inner),
        Second(Inner),
    }

    #[derive(TS)]
    #[cfg_attr(feature = "serde-compat", derive(Serialize))]
    struct T {
        a: String,
        #[cfg_attr(feature = "serde-compat", serde(flatten))]
        #[cfg_attr(not(feature = "serde-compat"), ts(flatten))]
        nested: Option<Enum>,
    }

    let cfg = Config::default();

    assert_eq!(
        T::optional_inline_flattened(&cfg),
        r#"{ a: string, } & ({ "first": Inner; "second"?: never } | { "second": Inner; "first"?: never } | { "first"?: never; "second"?: never })"#
    );
}

#[test]
fn kebab_case_renaming() {
    #[derive(TS)]
    #[cfg_attr(feature = "serde-compat", derive(Serialize))]
    #[cfg_attr(feature = "serde-compat", serde(rename_all = "kebab-case"))]
    #[cfg_attr(not(feature = "serde-compat"), ts(rename_all = "kebab-case"))]
    enum Enum {
        FirstOption(String),
        SecondOption(bool),
    }

    #[derive(TS)]
    #[cfg_attr(feature = "serde-compat", derive(Serialize))]
    struct T {
        a: String,
        #[cfg_attr(feature = "serde-compat", serde(flatten))]
        #[cfg_attr(not(feature = "serde-compat"), ts(flatten))]
        flattened: Option<Enum>,
    }

    let cfg = Config::default();
    let result = T::optional_inline_flattened(&cfg);

    assert!(result.contains(r#""first-option": string"#));
    assert!(result.contains(r#""second-option": boolean"#));
    assert!(result.contains(r#""first-option"?: never"#));
    assert!(result.contains(r#""second-option"?: never"#));
}

#[test]
fn single_variant_enum() {
    #[derive(TS)]
    #[cfg_attr(feature = "serde-compat", derive(Serialize))]
    #[cfg_attr(feature = "serde-compat", serde(rename_all = "camelCase"))]
    #[cfg_attr(not(feature = "serde-compat"), ts(rename_all = "camelCase"))]
    enum Enum {
        Only(String),
    }

    #[derive(TS)]
    #[cfg_attr(feature = "serde-compat", derive(Serialize))]
    struct T {
        a: String,
        #[cfg_attr(feature = "serde-compat", serde(flatten))]
        #[cfg_attr(not(feature = "serde-compat"), ts(flatten))]
        single: Option<Enum>,
    }

    let cfg = Config::default();

    assert_eq!(
        T::optional_inline_flattened(&cfg),
        r#"{ a: string, } & ({ "only": string; } | { "only"?: never })"#
    );
}

#[test]
fn original_non_optional_enum() {
    #[derive(TS)]
    #[cfg_attr(feature = "serde-compat", derive(Serialize))]
    #[cfg_attr(feature = "serde-compat", serde(rename_all = "camelCase"))]
    #[cfg_attr(not(feature = "serde-compat"), ts(rename_all = "camelCase"))]
    enum Enum {
        FirstOption(String),
        SecondOption(bool),
    }

    #[derive(TS)]
    #[cfg_attr(feature = "serde-compat", derive(Serialize))]
    struct T {
        a: String,
        #[cfg_attr(feature = "serde-compat", serde(flatten))]
        #[cfg_attr(not(feature = "serde-compat"), ts(flatten))]
        flattened: Enum,
    }

    let cfg = Config::default();

    assert_eq!(
        T::optional_inline_flattened(&cfg),
        r#"{ a: string, } & ({ "firstOption": string } | { "secondOption": boolean })"#
    );
}
