//! `Malarky` library support for the application crate.
#![doc(test(no_crate_inject))]

/// Exposes the executable source-mapping domain contract used by Malarky.
#[path = "../docs/design/malarky-domain.rs"]
pub mod domain_contract;

// TODO: Replace this stub when application logic moves behind the executable.
/// Returns the generated application greeting.
///
/// # Examples
///
/// ```
/// use malarky::greet;
///
/// assert_eq!(greet(), "Hello from Malarky!");
/// ```
#[must_use]
pub const fn greet() -> &'static str { "Hello from Malarky!" }
