//! Corpus oracle: parse every real course fixture in `tests/test-data/`.
//!
//! This guards the binary parser. It does three things:
//!  1. Prove every known good course still parses, catching accidental
//!     breakage when an enum or a struct layout is edited.
//!  2. Report the distribution of `CourseSaveDataVersion` across the corpus.
//!  3. Surface any `Unknown(_)` tags, a value from a newer app release that
//!     parsed but has no name.
//!
//! A parse failure means the on-disk layout of that save version is not
//! modelled by the parser.
//!
//! Run with `cargo test -p murmelbahn-lib --test parse_all -- --nocapture` to
//! see the summary.

use std::collections::BTreeMap;
use std::fs;
use std::path::PathBuf;

use murmelbahn_lib::app::course::{CourseSaveDataVersion, SaveDataHeader, SavedCourse};

fn test_data_dir() -> PathBuf {
    let mut dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    dir.push("tests/test-data");
    dir
}

/// Count occurrences of every `Unknown(n)` tag in a course's `Debug` output.
///
/// Each tag enum we open uses an `Unknown(u32)` catch-all, which `Debug`
/// renders as `Unknown(1337)`. Scanning the debug string finds them generically
/// across all course versions without hand-traversing each one's nested
/// structs, and sidesteps serde_json's lack of `u128` support (the header guid).
fn count_unknowns(debug: &str, out: &mut BTreeMap<u64, usize>) {
    const NEEDLE: &str = "Unknown(";
    let mut cursor = 0;
    while let Some(rel) = debug[cursor..].find(NEEDLE) {
        let start = cursor + rel + NEEDLE.len();
        let end = debug[start..].find(')').map(|e| start + e).unwrap_or(start);
        if let Ok(n) = debug[start..end].parse::<u64>() {
            *out.entry(n).or_default() += 1;
        }
        cursor = end.max(start + 1);
    }
}

#[test]
fn all_fixtures_parse() {
    let mut versions: BTreeMap<String, usize> = BTreeMap::new();
    let mut unknown_tags: BTreeMap<u64, usize> = BTreeMap::new();
    let mut failures: Vec<(String, Option<u32>, String)> = Vec::new();
    let mut total = 0usize;

    let dir = test_data_dir();
    if !dir.exists() {
        eprintln!("tests/test-data not present, skipping corpus oracle (local-only fixtures)");
        return;
    }

    for entry in fs::read_dir(&dir).expect("read tests/test-data").flatten() {
        let path = entry.path();
        if !path.is_file() {
            continue;
        }
        total += 1;
        let name = path.file_name().unwrap().to_string_lossy().into_owned();
        let bytes = fs::read(&path).expect("read fixture");

        match SavedCourse::from_bytes(&bytes) {
            Ok(course) => {
                *versions
                    .entry(format!("{:?}", course.header.version))
                    .or_default() += 1;
                count_unknowns(&format!("{course:?}"), &mut unknown_tags);
            }
            Err(e) => failures.push((
                name,
                SaveDataHeader::peek_version_raw(&bytes),
                e.to_string(),
            )),
        }
    }

    eprintln!("\n=== corpus oracle: {total} fixtures ===");
    eprintln!("versions:");
    for (version, count) in &versions {
        eprintln!("  {version}: {count}");
    }
    if unknown_tags.is_empty() {
        eprintln!("unknown tags: none");
    } else {
        eprintln!("unknown tags (raw discriminant -> count), from a newer app release:");
        for (id, count) in &unknown_tags {
            eprintln!("  {id}: {count}");
        }
    }
    if !failures.is_empty() {
        eprintln!("FAILURES ({}):", failures.len());
        for (name, raw_version, err) in &failures {
            let hint = match raw_version {
                Some(v) => match CourseSaveDataVersion::try_from(v.to_le_bytes().as_slice()) {
                    // Header version is one we recognise but couldn't decode the
                    // body: a regression, or a known-but-unsupported old version.
                    Ok(known) => format!(
                        "version {v} ({known:?}, supported={})",
                        known.is_supported()
                    ),
                    // Header version this parser does not define, so a newer
                    // app release changed the save format.
                    Err(_) => format!("version {v} UNKNOWN, an unrecognised save format"),
                },
                None => "version unreadable (truncated?)".to_string(),
            };
            eprintln!("  {name}: {hint} :: {err}");
        }
    }

    if total == 0 {
        eprintln!("tests/test-data is empty, skipping corpus oracle (local-only fixtures)");
        return;
    }
    assert!(
        failures.is_empty(),
        "{} fixture(s) failed to parse, an unmodelled save format; see summary above",
        failures.len()
    );
}
