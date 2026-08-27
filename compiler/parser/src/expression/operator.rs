use std::iter::Peekable;

use interfaces::Spanned;
use lexer::Token;

pub(super) type OperatorToken = lexer::Operator;

pub(super) fn peek_operator_token(
    tokens: &mut Peekable<impl Iterator<Item = Spanned<Token>>>,
) -> Option<&OperatorToken> {
    let Spanned { content, .. } = tokens.peek()?;

    let Token::Operator(operator) = content else {
        return None;
    };

    Some(operator)
}
