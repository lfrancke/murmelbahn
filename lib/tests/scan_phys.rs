//! Physical bill-of-materials robustness: converting every fixture's app BOM
//! into the physical (buildable) BOM must not panic. This exercises the
//! LayerKind/RailKind/TileKind -> Element conversions, which should return an
//! error for anything unmapped rather than crashing.

use murmelbahn_lib::app::BillOfMaterials as AppBom;
use murmelbahn_lib::app::course::SavedCourse;
use murmelbahn_lib::physical::BillOfMaterials as PhysBom;
use std::panic::{AssertUnwindSafe, catch_unwind};
use std::{fs, path::PathBuf};

#[test]
fn physical_bom_conversion_never_panics() {
    let mut dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    dir.push("tests/test-data");
    if !dir.exists() {
        eprintln!("tests/test-data not present, skipping (local-only fixtures)");
        return;
    }
    let prev = std::panic::take_hook();
    std::panic::set_hook(Box::new(|_| {}));

    let mut panicked = Vec::new();
    for entry in fs::read_dir(&dir).expect("read tests/test-data").flatten() {
        let path = entry.path();
        if !path.is_file() {
            continue;
        }
        let Ok(course) = SavedCourse::from_bytes(&fs::read(&path).expect("read fixture")) else {
            continue;
        };
        let app = AppBom::from(course.course);
        // try_from may legitimately return Err (unmapped element); only a panic is a bug.
        if catch_unwind(AssertUnwindSafe(|| {
            let _ = PhysBom::try_from(app);
        }))
        .is_err()
        {
            panicked.push(path.file_name().unwrap().to_string_lossy().into_owned());
        }
    }

    std::panic::set_hook(prev);
    assert!(
        panicked.is_empty(),
        "physical BOM conversion panicked for: {panicked:?}"
    );
}
