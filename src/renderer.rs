use crate::Ruby;

#[derive(Debug)]
pub struct HtmlRenderer {
    open_paren: String,
    close_paren: String,
}

impl HtmlRenderer {
    pub fn new() -> Self {
        Self {
            open_paren: "（".to_owned(),
            close_paren: "）".to_owned(),
        }
    }

    fn push_open_paren(&self, buffer: &mut String) {
        buffer.push_str("<rp>");
        buffer.push_str(&self.open_paren);
        buffer.push_str("</rp>");
    }

    fn push_close_paren(&self, buffer: &mut String) {
        buffer.push_str("<rp>");
        buffer.push_str(&self.close_paren);
        buffer.push_str("</rp>");
    }

    pub fn render(&self, ruby: &Ruby, buffer: &mut String) {
        buffer.push_str("<ruby>");

        for (rb, rt) in ruby.base().iter().zip(ruby.ruby()) {
            buffer.push_str(rb);
            self.push_open_paren(buffer);
            buffer.push_str("<rt>");
            buffer.push_str(rt);
            buffer.push_str("</rt>");
            self.push_close_paren(buffer);
        }

        buffer.push_str("</ruby>");
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn render_ruby(input: &Ruby) -> String {
        let mut output = String::new();
        HtmlRenderer::new().render(&input, &mut output);

        output
    }

    #[test]
    fn simple_ruby_to_html() {
        let input = Ruby::from_str_vecs(vec!["漢", "字"], vec!["かん", "じ"]);
        let expected = "<ruby>漢<rp>（</rp><rt>かん</rt><rp>）</rp>字<rp>（</rp><rt>じ</rt><rp>）</rp></ruby>";
        assert_eq!(render_ruby(&input).as_str(), expected);
    }

    #[test]
    fn one_group_ruby_to_html() {
        let input = Ruby::from_str_vecs(vec!["境界"], vec!["フロンティア"]);
        let expected = "<ruby>境界<rp>（</rp><rt>フロンティア</rt><rp>）</rp></ruby>";
        assert_eq!(render_ruby(&input).as_str(), expected);
    }
}
