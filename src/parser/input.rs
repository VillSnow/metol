use std::str::{CharIndices, Chars};

use super::utils::is_line_break_char;

#[derive(Debug, Clone, Copy)]
pub struct Input<'a> {
    pub s: &'a str,
    pub is_line_head: bool,
}

impl<'a> nom::Input for Input<'a> {
    type Item = char;
    type Iter = Chars<'a>;
    type IterIndices = CharIndices<'a>;

    fn input_len(&self) -> usize {
        self.s.len()
    }

    fn take(&self, index: usize) -> Self {
        Self {
            s: &self.s[..index],
            is_line_head: self.is_line_head,
        }
    }

    fn take_from(&self, index: usize) -> Self {
        let is_line_head = self.s[..index]
            .chars()
            .next_back()
            .map(is_line_break_char)
            .unwrap_or(self.is_line_head);

        Self {
            s: &self.s[index..],
            is_line_head,
        }
    }

    fn take_split(&self, index: usize) -> (Self, Self) {
        let (prefix, suffix) = self.s.split_at(index);

        (
            Input {
                s: suffix,
                is_line_head: prefix
                    .chars()
                    .next_back()
                    .map(is_line_break_char)
                    .unwrap_or(self.is_line_head),
            },
            Input {
                s: prefix,
                is_line_head: self.is_line_head,
            },
        )
    }

    fn position<P>(&self, predicate: P) -> Option<usize>
    where
        P: Fn(Self::Item) -> bool,
    {
        self.s.find(predicate)
    }

    fn iter_elements(&self) -> Self::Iter {
        self.s.chars()
    }

    fn iter_indices(&self) -> Self::IterIndices {
        self.s.char_indices()
    }

    fn slice_index(&self, count: usize) -> Result<usize, nom::Needed> {
        nom::Input::slice_index(&self.s, count)
    }
}

impl nom::Offset for Input<'_> {
    fn offset(&self, second: &Self) -> usize {
        second.s.as_ptr() as usize - self.s.as_ptr() as usize
    }
}

impl<'a> nom::Compare<&'a str> for Input<'a> {
    fn compare(&self, t: &'a str) -> nom::CompareResult {
        nom::Compare::compare(&self.s, t)
    }

    fn compare_no_case(&self, t: &'a str) -> nom::CompareResult {
        nom::Compare::compare_no_case(&self.s, t)
    }
}
