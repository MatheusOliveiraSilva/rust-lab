const CHUNK_SIZE: usize = 32;

pub struct Rope {
    chunks: Vec<String>,
    total_len: usize,
}

impl Rope {
    pub fn new() -> Self {
        Rope {
            chunks: Vec::new(),
            total_len: 0,
        }
    }

    pub fn from_str(s: &str) -> Self {
        let mut chunks = Vec::new();
        let mut i = 0;
        let len = s.len();

        while i < len {
            let mut end = usize::min(i + CHUNK_SIZE, len);

            while !s.is_char_boundary(end) {
                end -= 1;
            }

            let chunk = &s[i..end];
            chunks.push(chunk.to_string());

            i = end;
        }

        Rope {
            chunks,
            total_len: len,
        }
    }

    pub fn len(&self) -> usize {
        self.total_len
    }

    pub fn to_string(&self) -> String {
        self.chunks.join("")
    }

    pub fn find_chunk(&self, index: usize) -> Option<(usize, usize)> {
        if index > self.total_len {
            return None;
        }

        let mut accumulated = 0usize;

        for (i, chunk) in self.chunks.iter().enumerate() {
            if accumulated + chunk.len() > index {
                let local_offset = index - accumulated;
                return Some((i, local_offset));
            }
            accumulated += chunk.len();
        }

        if let Some(last_chunk) = self.chunks.last() {
            let last_idx = self.chunks.len() - 1;
            return Some((last_idx, last_chunk.len()));
        } else {
            return Some((0, 0));
        }
    }

    pub fn insert(&mut self, index: usize, text: &str) -> bool {
        if index > self.total_len {
            return false;
        }

        if self.chunks.is_empty() {
            self.chunks.push(text.to_string());
            self.total_len = text.len();
            return true;
        }

        let (chunk_index, local_offset) = match self.find_chunk(index) {
            Some(pair) => pair,
            None => return false,
        };

        let old_chunk = self.chunks.remove(chunk_index);
        let old_len = old_chunk.len();

        if local_offset == 0 {
            self.chunks.insert(chunk_index, text.to_string());
            self.chunks.insert(chunk_index + 1, old_chunk);
        } else if local_offset == old_len {
            self.chunks.insert(chunk_index, old_chunk);
            self.chunks.insert(chunk_index + 1, text.to_string());
        } else {
            let left = old_chunk[..local_offset].to_string();
            let right = old_chunk[local_offset..].to_string();

            let mut position = chunk_index;
            self.chunks.insert(position, left);
            position += 1;
            self.chunks.insert(position, text.to_string());
            position += 1;
            self.chunks.insert(position, right);
        }

        self.total_len += text.len();
        true
    }

    pub fn delete(&mut self, start: usize, end: usize) -> bool {
        let (from, to) = if start <= end { (start, end) } else { (end, start) };
        if from == to {
            return true;
        }
        if to > self.total_len {
            return false;
        }

        if self.chunks.is_empty() {
            return false;
        }

        let (start_chunk_idx, start_off) = match self.find_chunk(from) {
            Some(p) => p,
            None => return false,
        };
        let (end_chunk_idx, end_off) = match self.find_chunk(to) {
            Some(p) => p,
            None => return false,
        };

        if start_chunk_idx == end_chunk_idx {
            let chunk = self.chunks.remove(start_chunk_idx);
            let new_chunk = {
                let left = &chunk[..start_off];
                let right = &chunk[end_off..];
                let mut s = String::with_capacity(left.len() + right.len());
                s.push_str(left);
                s.push_str(right);
                s
            };
            if !new_chunk.is_empty() {
                self.chunks.insert(start_chunk_idx, new_chunk);
            }
            self.total_len -= to - from;
            return true;
        }

        let prefix = {
            let chunk = &self.chunks[start_chunk_idx];
            chunk[..start_off].to_string()
        };
        let suffix = {
            let chunk = &self.chunks[end_chunk_idx];
            chunk[end_off..].to_string()
        };

        self.chunks[start_chunk_idx] = prefix;

        if start_chunk_idx + 1 <= end_chunk_idx {
            self.chunks.drain(start_chunk_idx + 1..=end_chunk_idx);
        }

        if !suffix.is_empty() {
            if self.chunks[start_chunk_idx].is_empty() {
                self.chunks[start_chunk_idx] = suffix;
            } else {
                self.chunks.insert(start_chunk_idx + 1, suffix);
            }
        } else {
            if self.chunks[start_chunk_idx].is_empty() {
                self.chunks.remove(start_chunk_idx);
            }
        }

        self.total_len -= to - from;
        true
    }

    pub fn slice(&self, start: usize, end: usize) -> Option<String> {
        let (from, to) = if start <= end { (start, end) } else { (end, start) };

        if to > self.total_len {
            return None
        }

        if from == to {
            return Some(String::from(""))
        }

        let (start_chunk_idx, start_off) = self.find_chunk(start).unwrap();
        let (end_chunk_idx, end_off) = self.find_chunk(end).unwrap();

        if start_chunk_idx == end_chunk_idx {
            let chunk = &self.chunks[start_chunk_idx];
            let slice = chunk[start_off..end_off].to_string();
            return Some(slice)
        }

        let mut acumulator = String::with_capacity(to - from);
        let start_chunk = &self.chunks[start_chunk_idx];
        let left = &start_chunk[start_off..];

        let end_chunk = &self.chunks[end_chunk_idx];
        let right = &end_chunk[..end_off];

        acumulator.push_str(left);
        for chunk in &self.chunks[start_chunk_idx + 1..end_chunk_idx] {
            acumulator.push_str(chunk.as_str());
        }
        acumulator.push_str(right);

        Some(acumulator)
    }
}
