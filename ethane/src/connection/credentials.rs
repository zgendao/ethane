/// Credentials used for authentication.
///
/// Supports Basic and Bearer authentication.
#[derive(Debug, Clone)]
pub enum Credentials {
    Bearer(String),
    Basic(String),
}

impl Credentials {
    pub fn to_auth_string(&self) -> String {
        match self {
            Self::Bearer(token) => String::from("Bearer ") + &token,
            Self::Basic(token) => String::from("Basic ") + &token,
        }
    }
}
