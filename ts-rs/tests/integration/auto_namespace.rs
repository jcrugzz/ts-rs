use std::path::PathBuf;

use ts_rs::{TypeVisitor, TS};

// -- Manual TS impls for testing cross-crate behavior --

struct NsTypeA;
impl TS for NsTypeA {
    type WithoutGenerics = Self;
    type OptionInnerType = Self;
    fn name() -> String {
        "NsColliding".to_owned()
    }
    fn inline() -> String {
        "{ a: string }".to_owned()
    }
    fn inline_flattened() -> String {
        panic!("cannot be flattened")
    }
    fn optional_inline_flattened() -> String {
        panic!("cannot be flattened")
    }
    fn decl() -> String {
        "type NsColliding = { a: string };".to_owned()
    }
    fn decl_concrete() -> String {
        Self::decl()
    }
    fn output_path() -> Option<PathBuf> {
        Some(PathBuf::from("NsColliding.ts"))
    }
    fn crate_name() -> Option<&'static str> {
        Some("crate_a")
    }
}

struct NsTypeB;
impl TS for NsTypeB {
    type WithoutGenerics = Self;
    type OptionInnerType = Self;
    fn name() -> String {
        "NsColliding".to_owned()
    }
    fn inline() -> String {
        "{ b: number }".to_owned()
    }
    fn inline_flattened() -> String {
        panic!("cannot be flattened")
    }
    fn optional_inline_flattened() -> String {
        panic!("cannot be flattened")
    }
    fn decl() -> String {
        "type NsColliding = { b: number };".to_owned()
    }
    fn decl_concrete() -> String {
        Self::decl()
    }
    fn output_path() -> Option<PathBuf> {
        Some(PathBuf::from("NsColliding.ts"))
    }
    fn crate_name() -> Option<&'static str> {
        Some("crate_b")
    }
}

struct NsDep;
impl TS for NsDep {
    type WithoutGenerics = Self;
    type OptionInnerType = Self;
    fn name() -> String {
        "NsDep".to_owned()
    }
    fn inline() -> String {
        "{ val: string }".to_owned()
    }
    fn inline_flattened() -> String {
        panic!("cannot be flattened")
    }
    fn optional_inline_flattened() -> String {
        panic!("cannot be flattened")
    }
    fn decl() -> String {
        "type NsDep = { val: string };".to_owned()
    }
    fn decl_concrete() -> String {
        Self::decl()
    }
    fn output_path() -> Option<PathBuf> {
        Some(PathBuf::from("NsDep.ts"))
    }
    fn crate_name() -> Option<&'static str> {
        Some("dep_crate")
    }
}

struct NsParent;
impl TS for NsParent {
    type WithoutGenerics = Self;
    type OptionInnerType = Self;
    fn name() -> String {
        "NsParent".to_owned()
    }
    fn inline() -> String {
        format!("{{ dep: {} }}", NsDep::inline())
    }
    fn inline_flattened() -> String {
        panic!("cannot be flattened")
    }
    fn optional_inline_flattened() -> String {
        panic!("cannot be flattened")
    }
    fn decl() -> String {
        format!("type NsParent = {};", Self::inline())
    }
    fn decl_concrete() -> String {
        Self::decl()
    }
    fn output_path() -> Option<PathBuf> {
        Some(PathBuf::from("NsParent.ts"))
    }
    fn crate_name() -> Option<&'static str> {
        Some("parent_crate")
    }
    fn visit_dependencies(v: &mut impl TypeVisitor)
    where
        Self: 'static,
    {
        v.visit::<NsDep>();
    }
}

struct NsHyphenType;
impl TS for NsHyphenType {
    type WithoutGenerics = Self;
    type OptionInnerType = Self;
    fn name() -> String {
        "NsHyphenType".to_owned()
    }
    fn inline() -> String {
        "{ x: number }".to_owned()
    }
    fn inline_flattened() -> String {
        panic!("cannot be flattened")
    }
    fn optional_inline_flattened() -> String {
        panic!("cannot be flattened")
    }
    fn decl() -> String {
        "type NsHyphenType = { x: number };".to_owned()
    }
    fn decl_concrete() -> String {
        Self::decl()
    }
    fn output_path() -> Option<PathBuf> {
        Some(PathBuf::from("NsHyphenType.ts"))
    }
    fn crate_name() -> Option<&'static str> {
        Some("my-crate")
    }
}

fn unique_dir(name: &str) -> PathBuf {
    let dir = std::env::temp_dir()
        .join("ts_rs_auto_ns_tests")
        .join(name);
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

// All auto_namespace tests are in a single test function to avoid env var races
// (set_var/remove_var is process-global and tests run in parallel).
#[test]
fn auto_namespace() {
    std::env::set_var("TS_RS_AUTO_NAMESPACE", "true");

    // Test 1: separates crates
    {
        let dir = unique_dir("separates_crates");
        NsTypeA::export_all_to(&dir).unwrap();
        NsTypeB::export_all_to(&dir).unwrap();

        let path_a = dir.join("crate_a/NsColliding.ts");
        let path_b = dir.join("crate_b/NsColliding.ts");

        assert!(path_a.exists(), "crate_a/NsColliding.ts should exist");
        assert!(path_b.exists(), "crate_b/NsColliding.ts should exist");

        let content_a = std::fs::read_to_string(&path_a).unwrap();
        let content_b = std::fs::read_to_string(&path_b).unwrap();

        assert!(content_a.contains("a: string"), "crate_a should have NsTypeA's content");
        assert!(content_b.contains("b: number"), "crate_b should have NsTypeB's content");
    }

    // Test 2: import paths
    {
        let dir = unique_dir("import_paths");
        NsParent::export_all_to(&dir).unwrap();

        let parent_path = dir.join("parent_crate/NsParent.ts");
        assert!(parent_path.exists(), "parent_crate/NsParent.ts should exist");

        let content = std::fs::read_to_string(&parent_path).unwrap();
        assert!(
            content.contains("../dep_crate/NsDep"),
            "import path should reference ../dep_crate/NsDep, got:\n{content}"
        );
    }

    // Test 3: hyphen to underscore
    {
        let dir = unique_dir("hyphen_to_underscore");
        NsHyphenType::export_all_to(&dir).unwrap();

        let path = dir.join("my_crate/NsHyphenType.ts");
        assert!(
            path.exists(),
            "my_crate/NsHyphenType.ts should exist (hyphen converted to underscore)"
        );
    }

    std::env::remove_var("TS_RS_AUTO_NAMESPACE");

    // Test 4: off means no namespace
    {
        let dir = unique_dir("off_no_namespace");
        NsTypeA::export_all_to(&dir).unwrap();

        let path = dir.join("NsColliding.ts");
        assert!(path.exists(), "NsColliding.ts should exist at root level");
    }
}
