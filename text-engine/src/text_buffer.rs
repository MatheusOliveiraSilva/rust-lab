pub struct TextBuffer {
    text: String,
}

impl TextBuffer {
    pub fn new() -> Self {
        TextBuffer {
            text: String::from(""),
        }
    }

    pub fn from_str(s: &str) -> Self {
        TextBuffer {
            text: String::from(s),
        }
    }

    pub fn as_str(&self) -> &str {
        &self.text
    }

    pub fn len(&self) -> usize {
        self.text.len()
    }

    pub fn is_empty(&self) -> bool {
        self.text.is_empty()
    }

    pub fn insert(&mut self, at: usize, text: &str) -> bool {
        if at > self.text.len() || !self.text.is_char_boundary(at) {
            return false;
        }
        self.text.insert_str(at, text);
        true
    }

    pub fn delete(&mut self, start: usize, end: usize) -> bool {
        let (from, to) = if start <= end { (start, end) } else { (end, start) };

        if to > self.text.len() {
            return false;
        }
        if !self.text.is_char_boundary(from) || !self.text.is_char_boundary(to) {
            return false;
        }

        if from == to {
            return true;
        }

        self.text.replace_range(from..to, "");
        true
    }

    pub fn replace_range(&mut self, start: usize, end: usize, rep: &str) -> bool {
        let (from, to) = if start <= end { (start, end) } else { (end, start) };

        if to > self.text.len() {
            return false;
        }
        if !self.text.is_char_boundary(from) || !self.text.is_char_boundary(to) {
            return false;
        }

        self.text.replace_range(from..to, rep);
        true
    }

    pub fn get_range(&self, start: usize, end: usize) -> Option<&str> {
        let (from, to) = if start <= end { (start, end) } else { (end, start) };

        if to > self.text.len() {
            return None;
        }
        if !self.text.is_char_boundary(from) || !self.text.is_char_boundary(to) {
            return None;
        }

        if from == to {
            return Some("");
        }

        Some(&self.text[from..to])
    }
}
