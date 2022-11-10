use std::cmp;
use unicode_segmentation::UnicodeSegmentation;

#[derive(Default)]
pub struct Row {
    _string: String,
    len: usize,
}

impl From<&str> for Row {
    fn from(slice: &str) -> Self {
        let mut row = Self {
            _string: String::from(slice),
            len: 0,
        };

        row.update_len();
        row
    }
}

impl Row {
    /// Inputs are `start` and `end`  
    ///
    /// it return the content inside a row
    ///
    /// and also take care of any [`Graphemes`] Error
    pub fn render(&self, start: usize, end: usize) -> String {
        let end = cmp::min(end, self._string.len());
        let start = cmp::min(start, end);
        let mut result = String::new();

        for grapheme in self._string[..]
            .graphemes(true)
            .skip(start)
            .take(end - start)
        {
            if grapheme == "\t" {
                result.push_str(" ");
            } else {
                result.push_str(grapheme);
            }
        }

        result
    }

    pub fn len(&self) -> usize {
        self._string[..].graphemes(true).count()
    }

    pub fn is_empty(&self) -> bool {
        self._string.is_empty()
    }

    pub fn insert(&mut self, at: usize, c: char) {
        if at >= self.len() {
            self._string.push(c);
        } else {
            let mut result: String = self._string[..].graphemes(true).take(at).collect();
            let remainder: String = self._string[..].graphemes(true).skip(at).collect();
            result.push(c);
            result.push_str(&remainder);
            self._string = result;
        }
        self.update_len();
    }

    pub fn delete(&mut self, at: usize) {
        if at >= self.len() {
            return;
        } else {
            let mut result: String = self._string[..].graphemes(true).take(at).collect();
            let remainder: String = self._string[..].graphemes(true).skip(at + 1).collect();
            result.push_str(&remainder);
            self._string = result;
        }
    }

    pub fn append(&mut self, new: &Self) {
        self._string = format!("{}{}", self._string, new._string);
        self.update_len();
    }

    fn update_len(&mut self) {
        self.len = self._string[..].graphemes(true).count();
    }
}
