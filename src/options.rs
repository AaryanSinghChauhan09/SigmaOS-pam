//! Options model.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    Auth,
    Account,
    Session,
    Password,
}

#[derive(Debug, Clone)]
pub struct PamOptions {
    pub mode: Mode,
    pub service: String,
    pub user: String,
}

impl Default for PamOptions {
    fn default() -> Self { Self { mode: Mode::Auth, service: String::new(), user: String::new() } }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn opts() { assert_eq!(PamOptions::default().mode, Mode::Auth); }
}
