use crate::grammar::stmt::{parse_stmt, STMT_FIRST};
use crate::parser::{CompletedMarker, Parser};
use crate::token_set::TokenSet;
use syntax::SyntaxKind;
use token::TokenKind;

pub(super) const EXPR_FIRST: TokenSet = TokenSet::new([
    TokenKind::Ident,
    TokenKind::LBrace,
    TokenKind::Int,
    TokenKind::String,
    TokenKind::LParen,
]);

pub(super) fn parse_expr(p: &mut Parser<'_, '_>) -> Option<CompletedMarker> {
    parse_expr_with_recovery_set(p, TokenSet::default())
}

fn parse_expr_with_recovery_set(
    p: &mut Parser<'_, '_>,
    recovery_set: TokenSet,
) -> Option<CompletedMarker> {
    parse_expr_bp(p, 0, recovery_set)
}

fn parse_expr_bp(
    p: &mut Parser<'_, '_>,
    min_bp: u8,
    recovery_set: TokenSet,
) -> Option<CompletedMarker> {
    let mut lhs = parse_lhs(p, recovery_set)?;

    loop {
        let _guard = p.disable_expected_tracking();
        let (left_bp, right_bp) = if p.at(TokenKind::Plus) || p.at(TokenKind::Hyphen) {
            (1, 2)
        } else if p.at(TokenKind::Asterisk) || p.at(TokenKind::Slash) {
            (3, 4)
        } else {
            break;
        };

        if left_bp < min_bp {
            break;
        }

        p.bump();

        let m = lhs.precede(p);
        parse_expr_bp(p, right_bp, recovery_set);
        lhs = m.complete(p, SyntaxKind::BinExpr);
    }

    Some(lhs)
}

fn parse_lhs(p: &mut Parser<'_, '_>, recovery_set: TokenSet) -> Option<CompletedMarker> {
    let _guard = p.expected_syntax_name("expression");
    let completed_marker = if p.at(TokenKind::Ident) {
        parse_var_ref(p)
    } else if p.at(TokenKind::LBrace) {
        parse_block(p)
    } else if p.at(TokenKind::Int) {
        parse_int_literal(p)
    } else if p.at(TokenKind::String) {
        parse_string_literal(p)
    } else if p.at(TokenKind::LParen) {
        parse_paren_expr(p)
    } else {
        return p.error_with_recovery_set(recovery_set);
    };

    Some(completed_marker)
}

fn parse_var_ref(p: &mut Parser<'_, '_>) -> CompletedMarker {
    assert!(p.at(TokenKind::Ident));
    let m = p.start();
    p.bump();
    m.complete(p, SyntaxKind::VarRef)
}

fn parse_block(p: &mut Parser<'_, '_>) -> CompletedMarker {
    assert!(p.at(TokenKind::LBrace));
    let m = p.start();
    p.bump();

    while !p.at(TokenKind::RBrace) && p.at_set(STMT_FIRST) {
        parse_stmt(p);
    }

    p.expect(TokenKind::RBrace);

    m.complete(p, SyntaxKind::Block)
}

fn parse_int_literal(p: &mut Parser<'_, '_>) -> CompletedMarker {
    assert!(p.at(TokenKind::Int));
    let m = p.start();
    p.bump();
    m.complete(p, SyntaxKind::IntLiteral)
}

fn parse_string_literal(p: &mut Parser<'_, '_>) -> CompletedMarker {
    assert!(p.at(TokenKind::String));
    let m = p.start();
    p.bump();
    m.complete(p, SyntaxKind::StringLiteral)
}

fn parse_paren_expr(p: &mut Parser<'_, '_>) -> CompletedMarker {
    assert!(p.at(TokenKind::LParen));
    let m = p.start();
    p.bump();

    parse_expr_with_recovery_set(p, TokenSet::new([TokenKind::RParen]));
    p.expect(TokenKind::RParen);

    m.complete(p, SyntaxKind::ParenExpr)
}