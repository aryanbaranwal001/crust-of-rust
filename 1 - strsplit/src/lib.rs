// #![warn(missing_docs, missing_debug_implementations)]

pub struct StrSplit<'haystack, D> {
    remainder: Option<&'haystack str>,
    delimiter: D,
}

impl<'haystack, D> StrSplit<'haystack, D> {
    pub fn new(haystack: &'haystack str, delimiter: D) -> Self {
        Self {
            remainder: Some(haystack),
            delimiter,
        }
    }
}

pub trait Delimiter {
    fn find_next(&self, s: &str) -> Option<(usize, usize)>;
}

impl<'haystack, D> Iterator for StrSplit<'haystack, D>
where
    D: Delimiter,
{
    type Item = &'haystack str;
    fn next(&mut self) -> Option<Self::Item> {
        let remainder = self.remainder.as_mut()?;

        if let Some((delim_start, delim_end)) = self.delimiter.find_next(*remainder) {
            let until_delimiter = &remainder[..delim_start];

            *remainder = &remainder[delim_end..];
            Some(until_delimiter)
        } else {
            self.remainder.take()
        }
    }
}

impl Delimiter for &str {
    fn find_next(&self, s: &str) -> Option<(usize, usize)> {
        s.find(self).map(|start| (start, start + self.len()))
    }
}

impl Delimiter for char {
    fn find_next(&self, s: &str) -> Option<(usize, usize)> {
        s.char_indices()
            .find(|(_, b)| b == self)
            .map(|(start, _)| (start, start + self.len_utf8()))
    }
}

fn _until_char(s: &str, c: char) -> &str {
    let x = StrSplit::new(s, c).next().unwrap();
    x
}

#[test]
fn until_char_test() {
    assert_eq!(_until_char("hellow world", 'o'), "hell");
}
