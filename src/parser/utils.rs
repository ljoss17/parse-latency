use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Deserialize, Serialize)]
pub struct FailedEntry {
    pub line: usize,
    pub entry: String,
    pub cause: String,
}

impl FailedEntry {
    pub fn new(line: usize, entry: &str, cause: String) -> Self {
        Self {
            line,
            entry: entry.to_owned(),
            cause,
        }
    }
}
