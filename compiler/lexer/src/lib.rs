mod cursor;
mod token;

use std::iter;

use interfaces::Span;
pub use token::*;

use crate::cursor::Cursor;

#[derive(Debug)]
pub struct Lexer<'source> {
    cursor: Cursor<'source>,
}

impl<'source> Lexer<'source> {
    pub fn new(source: &'source str) -> Self {
        Self {
            cursor: Cursor::new(source),
        }
    }

    pub fn tokens(mut self) -> impl Iterator<Item = TokenSpan> {
        let mut tokenizer = tokenizer();
        iter::from_fn(move || self.next_token(&mut tokenizer))
    }

    fn next_token(&mut self, tokenizer: &mut impl Tokenizer) -> Option<TokenSpan> {
        let span_start = self.cursor.index();

        match tokenizer.next_token(&mut self.cursor) {
            NextToken::Token(token) => token_span(token, span_start, self.cursor.index()),
            NextToken::Ignore => self.next_token(tokenizer),
            NextToken::Done => None,
            NextToken::Unrecognized(_) => {
                token_span(Token::Invalid, span_start, self.cursor.index())
            }
        }
    }
}

fn tokenizer() -> impl Tokenizer {
    let dash_template = dash_builder_template();
    let tilde_template = tilde_builder_template();
    let plus_template = plus_builder_template();
    let star_template = star_builder_template();
    let slash_template = slash_builder_template();
    let percent_template = percent_builder_template();

    IdentifierBuilder::new
        .tokenizer()
        .with(NewLineTokenBuilder::new.tokenizer())
        .with(NumericLiteralBuilder::new.tokenizer())
        .with(StringLiteralBuilder::new.tokenizer())
        .with((move |start| dash_template.create(start)).tokenizer())
        .with((move |start| tilde_template.create(start)).tokenizer())
        .with((move |start| plus_template.create(start)).tokenizer())
        .with((move |start| star_template.create(start)).tokenizer())
        .with((move |start| slash_template.create(start)).tokenizer())
        .with((move |start| percent_template.create(start)).tokenizer())
        .space_separate()
}

const fn token_span(token: Token, span_start: usize, span_end: usize) -> Option<TokenSpan> {
    let span = Span {
        start: span_start,
        length: span_end - span_start,
    };

    Some(TokenSpan { token, span })
}

#[cfg(test)]
mod tests {
    use crate::Lexer;

    #[test]
    fn test() {
        let lexer = Lexer::new(
            r#"Hello pizza 1 rust35 5.5 2"5rust 10."0
            25.0rust "Hi pizzzzzaa!!
                     \\
            2.5 8" + 35 - *** / 42.5-~
                \\
            +*/%"#,
        );
        for token in lexer.tokens() {
            println!("{token:?}")
        }
    }
}
