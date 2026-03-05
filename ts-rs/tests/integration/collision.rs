use ts_rs::TS;

#[derive(TS)]
#[ts(rename = "Idempotent", export_to = "collision/Idempotent.ts")]
struct Idempotent {
    a: String,
}

#[derive(TS)]
#[ts(rename = "Colliding", export_to = "collision/Colliding.ts")]
struct CollidingA {
    x: i32,
}

#[derive(TS)]
#[ts(rename = "Colliding", export_to = "collision/Colliding.ts")]
struct CollidingB {
    y: bool,
    z: String,
}

#[test]
fn idempotent_export() {
    Idempotent::export_all().unwrap();
    // Second export of the exact same type should succeed
    Idempotent::export_all().unwrap();
}

#[test]
fn collision_is_detected() {
    CollidingA::export_all().unwrap();
    let err = CollidingB::export_all().unwrap_err();
    let msg = err.to_string();
    assert!(
        msg.contains("Colliding"),
        "error should mention the colliding type name, got: {msg}"
    );
}
