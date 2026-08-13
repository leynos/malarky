//! Contract test for the repository `Makefile`.
//!
//! The dev-fast profile (Cranelift plus mold, configured in
//! `tools/dev-fast/config.toml`) is the standard development path: the
//! `build`, `test`, `lint`, and `typecheck` targets must pass
//! `--config tools/dev-fast/config.toml` to every cargo invocation they
//! make, and `coverage` must never do so, because it needs the supported
//! LLVM backend and platform linker for `cargo llvm-cov`. See the "dev-fast
//! profile is the standard development path" section of `AGENTS.md`.
//!
//! This test parses `Makefile` textually rather than semantically, so that
//! an agent who edits a standard target's recipe without preserving the
//! `--config` wiring fails locally, before the estate-wide audit does.

use std::{fs, io, path::PathBuf};

use rstest::rstest;

/// Returns the raw text of the repository's root `Makefile`.
///
/// This is a fixture, not a test body: it propagates the read error
/// rather than panicking, so each test decides how to report a failure.
fn makefile_text() -> io::Result<String> {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("Makefile");
    fs::read_to_string(&path)
}

/// Returns `true` if `makefile` contains a header line starting with
/// `header`, that is, a line beginning at column zero with `header`
/// (typically `"<target>:"`).
fn has_header(makefile: &str, header: &str) -> bool {
    makefile.lines().any(|line| line.starts_with(header))
}

/// Extracts the tab-indented recipe lines that immediately follow the
/// first line starting with `header`, stopping at the next line that is
/// not tab-indented. Returns an empty string if `header` is not found.
fn recipe_for(makefile: &str, header: &str) -> String {
    let mut lines = makefile.lines();
    let found_header = lines.by_ref().any(|line| line.starts_with(header));
    if found_header {
        collect_recipe_lines(lines)
    } else {
        String::new()
    }
}

/// Collects leading tab-indented lines from `lines` into a single string,
/// one line per entry, stopping at the first non-indented line.
fn collect_recipe_lines<'a>(lines: impl Iterator<Item = &'a str>) -> String {
    let mut recipe = String::new();
    for line in lines {
        if !line.starts_with('\t') {
            break;
        }
        recipe.push_str(line);
        recipe.push('\n');
    }
    recipe
}

/// Returns `true` if `text` case-insensitively matches `dev[-_]fast`.
fn mentions_dev_fast(text: &str) -> bool {
    let lower = text.to_lowercase();
    lower.contains("dev-fast") || lower.contains("dev_fast")
}

/// Returns `true` if `line` invokes cargo, via the `$(CARGO)` Make
/// variable or a bare `cargo` word.
fn invokes_cargo(line: &str) -> bool { line.contains("$(CARGO)") || line.contains("cargo") }

/// Returns the cargo-invoking lines of `recipe` that do not pass
/// `--config`, so a caller can report exactly which invocation was missed
/// rather than only that the recipe as a whole lacks the flag somewhere.
fn cargo_lines_missing_config(recipe: &str) -> Vec<&str> {
    recipe
        .lines()
        .filter(|line| invokes_cargo(line))
        .filter(|line| !line.contains("--config"))
        .collect()
}

/// The standard development targets, and the Makefile header whose recipe
/// text carries their cargo invocation. `build` is a phony target with an
/// empty recipe of its own; its cargo invocation lives in the
/// `target/%/$(TARGET):` pattern rule it depends on, so that header is
/// checked in its place.
// Note: the case name deliberately avoids the literal `test`, which
// collides with rstest's own use of `extern crate test` and silently
// drops the `#[test]` registration for every case in the group.
#[rstest]
#[case::build("build:", "target/%/$(TARGET):")]
#[case::unit_test("test:", "test:")]
#[case::lint("lint:", "lint:")]
#[case::typecheck("typecheck:", "typecheck:")]
fn standard_targets_apply_dev_fast_config(
    #[case] target_header: &str,
    #[case] recipe_header: &str,
) {
    let makefile = makefile_text().expect("failed to read Makefile");
    if !has_header(&makefile, target_header) {
        // The target does not exist in this Makefile; nothing to assert.
        return;
    }
    let recipe = recipe_for(&makefile, recipe_header);
    assert!(
        !recipe.is_empty(),
        "expected a non-empty recipe for {recipe_header} (implementing {target_header}) in \
         Makefile; see the \"dev-fast profile is the standard development path\" section of \
         AGENTS.md"
    );
    let missing = cargo_lines_missing_config(&recipe);
    assert!(
        missing.is_empty(),
        "{target_header} (recipe {recipe_header}) has cargo invocation(s) without --config, so \
         they would skip the dev-fast profile: {missing:?}; see AGENTS.md"
    );
    assert!(
        mentions_dev_fast(&recipe),
        "{target_header} (recipe {recipe_header}) must reference the dev-fast configuration \
         fragment (matching /dev[-_]fast/i); see AGENTS.md"
    );
}

/// `coverage` needs the supported LLVM backend and platform linker for
/// `cargo llvm-cov`, so it must never apply the dev-fast profile.
#[test]
fn coverage_target_excludes_dev_fast() {
    let makefile = makefile_text().expect("failed to read Makefile");
    if !has_header(&makefile, "coverage:") {
        return;
    }
    let recipe = recipe_for(&makefile, "coverage:");
    assert!(
        !recipe.is_empty(),
        "expected a non-empty recipe for the coverage target in Makefile"
    );
    assert!(
        !mentions_dev_fast(&recipe),
        "coverage must never apply the dev-fast profile (Cranelift/mold); it needs the supported \
         LLVM backend and platform linker for cargo llvm-cov, per AGENTS.md"
    );
}

/// The dev-fast fragment the standard targets reference must actually
/// exist, or the `--config` flag they pass points nowhere.
#[test]
fn dev_fast_fragment_exists() {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tools/dev-fast/config.toml");
    assert!(
        path.is_file(),
        "expected {} to exist; it is the dev-fast configuration fragment referenced by the \
         standard Makefile targets and AGENTS.md",
        path.display()
    );
}
