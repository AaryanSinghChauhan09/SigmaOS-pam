//! SigmaOS-pam — Pluggable Authentication Modules model.

pub mod util;
pub mod options;

pub use util::{Pam, Module, Result};
pub use options::{PamOptions, Mode};
