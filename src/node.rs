
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum NodeType {
    Archive,
    Full,
    Light,
}

impl From<&str> for NodeType {
    fn from(journaldb_type: &str) -> Self {
        match journaldb_type {
            "archive" => NodeType::Archive,
            "full" => NodeType::Full,
            "light" => NodeType::Light,
            str => panic!("input archive or full, your input: {}", str),
        }
    }
}

