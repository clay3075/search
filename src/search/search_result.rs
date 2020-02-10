pub struct SearchResult {
    pub path: String,
    pub query: String,
    pub line_number: u64,
    pub cursor_pos: u64
}

impl SearchResult  {
    pub fn new(path: String, query: String, line_number: u64, cursor_pos: u64) -> Self {
        Self {
            path,
            query,
            line_number,
            cursor_pos
        }
    }
}

