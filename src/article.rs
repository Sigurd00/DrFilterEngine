use std::hash::{Hash, Hasher};

pub struct Article {
    uuid: String,
    link: String,
    content: Option<String>,
    identified_keywords: Option<Vec<String>>,
}

impl Article {
    pub fn new(
        uuid: String,
        link: String,
        content: Option<String>,
        identified_keywords: Option<Vec<String>>,
    ) -> Self {
        Self {
            uuid,
            link,
            content,
            identified_keywords,
        }
    }
}

impl Eq for Article {}

impl PartialEq for Article {
    fn eq(&self, other: &Self) -> bool {
        self.uuid == other.uuid
    }
}

impl Hash for Article {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.uuid.hash(state)
    }
}
