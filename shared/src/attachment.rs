use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Attachment {
    pub id: Option<String>,
    pub content_type: String,
}

impl Attachment {
    pub fn extension(&self) -> &str {
        let mut iter = self.content_type.split("/");
        let toplevel = iter.next().unwrap_or("bin");
        let sublevel = iter.next().unwrap_or("bin");
        mime_guess::get_extensions(toplevel, sublevel)
            .map(|ext| ext[0])
            .unwrap_or("bin")
    }
}

impl fmt::Display for Attachment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}.{}",
            self.id.clone().unwrap_or("None".into()),
            self.extension()
        )
    }
}

