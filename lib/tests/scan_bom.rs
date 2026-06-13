//! Bill-of-materials robustness: computing the BOM for every fixture must not
//! panic. The parse oracle (parse_all) only deserializes courses; this exercises
//! the heavier BOM path (rail-length and retainer-height resolution), which is
//! where a malformed or unhandled course would crash.

use murmelbahn_lib::app::BillOfMaterials;
use murmelbahn_lib::app::course::SavedCourse;
use std::panic::{AssertUnwindSafe, catch_unwind};
use std::{fs, path::PathBuf};

#[test]
fn every_course_computes_a_bill_of_materials_without_panicking() {
    let mut dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    dir.push("tests/test-data");

    let previous_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(|_| {})); // keep per-panic noise out of the report

    let mut failures = Vec::new();
    for entry in fs::read_dir(&dir).expect("read tests/test-data").flatten() {
        let path = entry.path();
        if !path.is_file() {
            continue;
        }
        let bytes = fs::read(&path).expect("read fixture");
        let Ok(course) = SavedCourse::from_bytes(&bytes) else {
            continue; // parse failures are the parse oracle's job, not this test's
        };
        let result = catch_unwind(AssertUnwindSafe(|| {
            let _ = BillOfMaterials::from(course.course);
        }));
        if result.is_err() {
            failures.push(path.file_name().unwrap().to_string_lossy().into_owned());
        }
    }

    std::panic::set_hook(previous_hook);
    assert!(
        failures.is_empty(),
        "bill of materials panicked for: {failures:?}"
    );
}
