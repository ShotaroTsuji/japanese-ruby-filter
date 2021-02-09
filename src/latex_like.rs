#[derive(Debug,Clone)]
pub struct LatexLike<'a> {
    slice: &'a str,
}

pub trait ArityTable {
    fn get_arity<S: AsRef<str>>(&self, name: S) -> Option<usize>;
}

impl ArityTable for [(&str, usize)] {
    fn get_arity<S: AsRef<str>>(&self, name: S) -> Option<usize> {
        let name = name.as_ref();
        self.iter()
            .find_map(|(cmd, arity)| {
                if cmd == &name {
                    Some(*arity)
                } else {
                    None
                }
            })
    }
}

#[derive(Debug,Clone,PartialEq)]
pub struct Command<'a> {
    name: &'a str,
    args: Vec<&'a str>,
}

fn take_next_command<'a, T: ArityTable + ?Sized>(s: &'a str, arity_table: &T) -> Option<(&'a str, Command<'a>, &'a str)> {
    take_next_command_inner(s, arity_table, 0)
}

fn take_next_command_inner<'a, T: ArityTable + ?Sized>(s: &'a str, arity_table: &T, index: usize) -> Option<(&'a str, Command<'a>, &'a str)> {
    let parsed = s.get(index..)?;

    match parse_command(parsed, arity_table) {
        Ok((pos, cmd)) => {
            let before = &s[..index];
            let after = &s[index+pos..];
            return Some((before, cmd, after));
        },
        Err(retry_pos) => {
            take_next_command_inner(s, arity_table, index + retry_pos)
        },
    }
}

fn parse_command<'a, T: ArityTable + ?Sized>(s: &'a str, arity_table: &T) -> Result<(usize, Command<'a>), usize> {
    let cmd_name_end = match parse_command_name(s) {
        Some(pos) => pos,
        None => {
            return match s.chars().next() {
                Some(c) => Err(c.len_utf8()),
                None => Err(1),
            };
        },
    };
    let name = &s[1..cmd_name_end];
    eprintln!("# Command name: {}", name);

    let arity = match arity_table.get_arity(name) {
        Some(arity) => arity,
        None => return Err(1),
    };

    let mut args = Arguments::with_position(s, cmd_name_end);
    let mut args_vec = Vec::with_capacity(arity);

    for _ in 0..arity {
        match args.next() {
            Some(arg) => {
                args_vec.push(arg);
            },
            None => {
                return Err(cmd_name_end+1);
            },
        }
    }

    let cmd = Command {
        name: name,
        args: args_vec,
    };

    Ok((args.current_head(), cmd))
}

fn parse_command_name(s: &str) -> Option<usize> {
    let mut iter = s.char_indices();

    match iter.next() {
        Some((0, '\\')) => {},
        _ => return None,
    }

    match iter.next() {
        Some((_, c)) if c.is_ascii_alphabetic() => {},
        _ => return None,
    }

    for (index, c) in iter {
        if !c.is_ascii_alphabetic() {
            return Some(index);
        }
    }

    Some(s.len())
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

#[derive(Debug,Clone)]
pub struct Arguments<'a> {
    s: &'a str,
    head: usize,
}

impl<'a> Arguments<'a> {
    pub fn new(s: &'a str) -> Self {
        Self {
            s: s,
            head: 0,
        }
    }

    pub fn with_position(s: &'a str, pos: usize) -> Self {
        Self {
            s: s,
            head: pos,
        }
    }

    pub fn into_inner(self) -> &'a str {
        self.s
    }

    pub fn current_head(&self) -> usize {
        self.head
    }
}

impl<'a> Iterator for Arguments<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        let s = &self.s[self.head..];
        eprintln!("s = {}", s);
        if !s.starts_with('{') {
            return None;
        }

        let end = find_close_brace(s)?;
        eprintln!("end = {}", end);

        let arg = &s[1..end];
        eprintln!("arg = {}", arg);

        self.head = self.head + end + 1;

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
        let s = "{漢字}{かんじ}です";
        let mut args = Arguments::new(s);
        assert_eq!(args.next(), Some("漢字"));
        assert_eq!(args.current_head(), 8);
        assert_eq!(args.next(), Some("かんじ"));
        assert_eq!(args.next(), None);
        assert_eq!(args.next(), None);
        assert_eq!(args.current_head(), 19);
        assert_eq!(&s[args.current_head()..], "です");
    }

    #[test]
    fn parse_unclosed_argument() {
        let mut args = Arguments::new("{漢字}{かんじです");
        assert_eq!(args.next(), Some("漢字"));
        assert_eq!(args.next(), None);
        assert_eq!(args.next(), None);
        assert_eq!(args.current_head(), 8);
    }

    #[test]
    fn parse_one_command() {
        let s = "\\cmd{aaa} world!";
        let table = [("cmd", 1usize)];
        assert_eq!(parse_command(s, &table[..]), Ok((9,
                    Command { name: "cmd", args: vec!["aaa"], })));
    }

    #[test]
    fn parse_command_name_followed_by_brace() {
        assert_eq!(parse_command_name("\\cmd{abc}"), Some(4));
        assert_eq!(parse_command_name("\\ruby{abc}"), Some(5));
    }

    #[test]
    fn parse_command_name_followed_by_space() {
        assert_eq!(parse_command_name("\\nop abc"), Some(4));
    }

    #[test]
    fn take_next_one_command() {
        let s = "Hello, \\cmd{world}!";
        let table = [("cmd", 1usize)];
        assert_eq!(take_next_command(s, &table[..]), Some(("Hello, ",
                    Command { name: "cmd", args: vec!["world"] },
                    "!"
        )));
        let s = "これは\\ruby{漢字}{かんじ}のルビです。";
        let table = [("ruby", 2usize)];
        assert_eq!(take_next_command(s, &table[..]), Some(("これは",
                    Command { name: "ruby", args: vec!["漢字", "かんじ"] },
                    "のルビです。"
        )));
    }

    #[test]
    fn try_to_take_command_from_no_command() {
        let s = "Hello, world!";
        let table = [("cmd", 1usize)];
        assert_eq!(take_next_command(s, &table[..]), None);
    }
}
