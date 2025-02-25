use crate::parser::{CompletedMarker, Parser};
use crate::token_set::TokenSet;
use syntax::{NodeKind, TokenKind};

pub(super) fn parse_ty(p: &mut Parser<'_>, recovery_set: TokenSet) -> CompletedMarker {
    let m = p.start();
    p.expect_with_recovery_set(TokenKind::Ident, recovery_set);
    m.complete(p, NodeKind::Ty)
}
