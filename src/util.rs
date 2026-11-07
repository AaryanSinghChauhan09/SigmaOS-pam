//! Pam model — authentication framework.

#[derive(Debug, Clone)]
pub struct Module {
    pub name: String,
    pub type_: String,
    pub control: String,
    pub args: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Result {
    Success,
    Ignore,
    Fail,
    Bad,
    Die,
    OK,
    Done,
}

#[derive(Debug, Clone)]
pub struct Pam {
    pub service: String,
    pub user: String,
}

impl Pam {
    pub fn new(service: &str, user: &str) -> Self {
        Self { service: service.to_string(), user: user.to_string() }
    }

    pub fn authenticate(&self, _modules: &[Module]) -> Result {
        Result::Success
    }

    pub fn account(&self, _modules: &[Module]) -> Result {
        Result::Success
    }

    pub fn session(&self, _modules: &[Module]) -> Result {
        Result::Success
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn auth() {
        let p = Pam::new("sshd", "user");
        assert_eq!(p.authenticate(&[]), Result::Success);
    }
}
