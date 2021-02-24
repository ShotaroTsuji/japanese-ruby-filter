use std::collections::VecDeque;
use pulldown_cmark::Event;
use crate::Filtered;
use crate::latex_like::LatexLikeFilter;
use crate::renderer::HtmlRenderer;

#[derive(Debug)]
pub struct Filter<'a, I> {
    iter: I,
    renderer: HtmlRenderer,
    queue: VecDeque<Event<'a>>,
}

impl<'a, I> Filter<'a, I>
where
    I: Iterator<Item=Event<'a>>,
{
    pub fn new(iter: I) -> Self {
        Self {
            iter: iter,
            renderer: HtmlRenderer::new(),
            queue: VecDeque::new(),
        }
    }
}

impl<'a, I> Iterator for Filter<'a, I>
where
    I: Iterator<Item=Event<'a>>,
{
    type Item = Event<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(event) = self.queue.pop_front() {
            return Some(event);
        }

        match self.iter.next() {
            Some(Event::Text(s)) => {
                for filtered in LatexLikeFilter::new(&s) {
                    eprintln!("{:?}", filtered);
                    match filtered {
                        Filtered::Plain(s) => {
                            let s = s.to_owned();
                            self.queue.push_back(Event::Text(s.into()));
                        },
                        Filtered::Ruby(r) => {
                            let mut buf = String::new();
                            self.renderer.render(&r, &mut buf);
                            self.queue.push_back(Event::Html(buf.into()));
                        },
                    }
                }
            },
            Some(event) => {
                self.queue.push_back(event);
            },
            None => {},
        }

        self.queue.pop_front()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use pulldown_cmark::Tag;

    #[test]
    fn apply_filter() {
        let s =
r#"## 章題

これは\ruby{漢字}{かん|じ}にルビをつける機能です。"#;

        let parser = pulldown_cmark::Parser::new(s);
        let mut filter = Filter::new(parser);
        assert_eq!(filter.next(), Some(Event::Start(Tag::Heading(2))));
        assert_eq!(filter.next(), Some(Event::Text("章題".into())));
        assert_eq!(filter.next(), Some(Event::End(Tag::Heading(2))));
        assert_eq!(filter.next(), Some(Event::Start(Tag::Paragraph)));
        assert_eq!(filter.next(), Some(Event::Text("これは".into())));
        assert_eq!(filter.next(), Some(Event::Html(r#"<ruby>漢<rp>（</rp><rt>かん</rt><rp>）</rp>字<rp>（</rp><rt>じ</rt><rp>）</rp></ruby>"#.into())));
        assert_eq!(filter.next(), Some(Event::Text("にルビをつける機能です。".into())));
        assert_eq!(filter.next(), Some(Event::End(Tag::Paragraph)));
        assert_eq!(filter.next(), None);
        assert_eq!(filter.next(), None);
        assert_eq!(filter.next(), None);
    }
}
