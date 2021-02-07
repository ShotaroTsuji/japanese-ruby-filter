#[derive(Debug,Clone)]
pub struct LatexLike<'a> {
    slice: &'a str,
}

fn match_macro_name(s: &str) -> (Option<&str>, &str) {
    if !s.starts_with('\\') {
        return (None, s);
    }

    for (index, c) in s.char_indices().skip(1) {
        if !c.is_ascii_alphanumeric() {
            return (Some(&s[1..index]), &s[index..]);
        }
    }

    (Some(&s[1..]), "")
}

fn take_exact_n_args(s: &str, n: usize) -> Option<(Vec<&str>, &str)> {
    let mut iter = Arguments::new(s);
    let mut v = Vec::with_capacity(n);

    for _ in 0..n {
        v.push(iter.next()?);
    }

    match iter.next() {
        Some(_) => None,
        None => Some((v, iter.into_inner())),
    }
}

#[derive(Debug,Clone)]
pub struct Arguments<'a> {
    s: &'a str,
}

impl<'a> Arguments<'a> {
    pub fn new(s: &'a str) -> Self {
        Self {
            s: s,
        }
    }

    pub fn into_inner(self) -> &'a str {
        self.s
    }
}

impl<'a> Iterator for Arguments<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.s.starts_with('{') {
            return None;
        }

        let end = find_close_brace(self.s)?;

        let arg = &self.s[1..end];
        let remain = &self.s[end+1..];

        self.s = remain;

        Some(arg)
    }
}

fn find_close_brace(s: &str) -> Option<usize> {
    let mut is_escaped = false;

    for (index, c) in s.char_indices() {
        match (c, is_escaped) {
            ('\\', false) => {
                is_escaped = true;
            },
            ('}', false) => {
                return Some(index);
            },
            _ => {
                is_escaped = false;
            }
        }
    }

    None
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn match_ruby_macro() {
        let s = "\\ruby{漢字}{かんじ}";
        assert_eq!(match_macro_name(s), (Some("ruby"), "{漢字}{かんじ}"));
    }

    #[test]
    fn find_close_brace_normal() {
        let s = "{漢字}";
        assert_eq!(find_close_brace(s), Some(7));
    }

    #[test]
    fn parse_two_arguments() {
        let mut args = Arguments::new("{漢字}{かんじ}です");
        assert_eq!(args.next(), Some("漢字"));
        assert_eq!(args.next(), Some("かんじ"));
        assert_eq!(args.next(), None);
        assert_eq!(args.next(), None);
        assert_eq!(args.into_inner(), "です");
    }

    #[test]
    fn take_exact_two_arguments() {
        let s = "{漢字}{かんじ}です";
        assert_eq!(take_exact_n_args(s, 2), Some((vec!["漢字", "かんじ"], "です")));
    }

    #[test]
    fn take_exact_three_arguments() {
        let s = "{a}{b}{c}def";
        assert_eq!(take_exact_n_args(s, 3), Some((vec!["a", "b", "c"], "def")));
    }
}
