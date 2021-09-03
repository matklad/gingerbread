use crate::parser::{CompletedMarker, Parser};
use syntax::SyntaxKind;
use token::TokenKind;

pub(crate) fn root(p: &mut Parser<'_, '_>) {
    let m = p.start();

    if !p.at_eof() {
        parse_expr(p);
    }

    m.complete(p, SyntaxKind::Root);
}

fn parse_expr(p: &mut Parser<'_, '_>) {
    parse_expr_bp(p, 0)
}

fn parse_expr_bp(p: &mut Parser<'_, '_>, min_bp: u8) {
    let mut lhs = parse_lhs(p);

    loop {
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
        parse_expr_bp(p, right_bp);
        lhs = m.complete(p, SyntaxKind::BinExpr);
    }
}

fn parse_lhs(p: &mut Parser<'_, '_>) -> CompletedMarker {
    if p.at(TokenKind::Ident) {
        parse_var_ref(p)
    } else if p.at(TokenKind::Int) {
        parse_int_literal(p)
    } else {
        panic!();
    }
}

fn parse_var_ref(p: &mut Parser<'_, '_>) -> CompletedMarker {
    assert!(p.at(TokenKind::Ident));
    let m = p.start();
    p.bump();
    m.complete(p, SyntaxKind::VarRef)
}

fn parse_int_literal(p: &mut Parser<'_, '_>) -> CompletedMarker {
    assert!(p.at(TokenKind::Int));
    let m = p.start();
    p.bump();
    m.complete(p, SyntaxKind::IntLiteral)
}

#[cfg(test)]
mod tests {
    use expect_test::{expect, Expect};

    fn check(input: &str, expect: Expect) {
        let tokens = lexer::lex(input);
        let parse = crate::parse(tokens);
        expect.assert_eq(&parse.debug_syntax_tree());
    }

    #[test]
    fn parse_nothing() {
        check("", expect![["Root@0..0"]]);
    }

    #[test]
    fn parse_var_ref() {
        check(
            "foo",
            expect![[r#"
Root@0..3
  VarRef@0..3
    Ident@0..3 "foo""#]],
        );
    }

    #[test]
    fn parse_var_ref_with_whitespace() {
        check(
            " foo   ",
            expect![[r#"
Root@0..7
  Whitespace@0..1 " "
  VarRef@1..7
    Ident@1..4 "foo"
    Whitespace@4..7 "   ""#]],
        );
    }

    #[test]
    fn parse_int_literal() {
        check(
            "123",
            expect![[r#"
Root@0..3
  IntLiteral@0..3
    Int@0..3 "123""#]],
        );
    }

    #[test]
    fn parse_int_literal_addition() {
        check(
            "2+4",
            expect![[r#"
Root@0..3
  BinExpr@0..3
    IntLiteral@0..1
      Int@0..1 "2"
    Plus@1..2 "+"
    IntLiteral@2..3
      Int@2..3 "4""#]],
        );
    }

    #[test]
    fn parse_var_ref_and_int_literal_subtraction() {
        check(
            "len-1",
            expect![[r#"
Root@0..5
  BinExpr@0..5
    VarRef@0..3
      Ident@0..3 "len"
    Hyphen@3..4 "-"
    IntLiteral@4..5
      Int@4..5 "1""#]],
        );
    }

    #[test]
    fn parse_var_ref_multiplication() {
        check(
            "foo * bar",
            expect![[r#"
Root@0..9
  BinExpr@0..9
    VarRef@0..4
      Ident@0..3 "foo"
      Whitespace@3..4 " "
    Asterisk@4..5 "*"
    Whitespace@5..6 " "
    VarRef@6..9
      Ident@6..9 "bar""#]],
        );
    }

    #[test]
    fn parse_int_literal_division() {
        check(
            "22/ 7",
            expect![[r#"
Root@0..5
  BinExpr@0..5
    IntLiteral@0..2
      Int@0..2 "22"
    Slash@2..3 "/"
    Whitespace@3..4 " "
    IntLiteral@4..5
      Int@4..5 "7""#]],
        );
    }

    #[test]
    fn parse_two_additions() {
        check(
            "1+2+3",
            expect![[r#"
Root@0..5
  BinExpr@0..5
    BinExpr@0..3
      IntLiteral@0..1
        Int@0..1 "1"
      Plus@1..2 "+"
      IntLiteral@2..3
        Int@2..3 "2"
    Plus@3..4 "+"
    IntLiteral@4..5
      Int@4..5 "3""#]],
        );
    }

    #[test]
    fn parse_four_multiplications() {
        check(
            "x1*x2*x3*x4",
            expect![[r#"
Root@0..11
  BinExpr@0..11
    BinExpr@0..8
      BinExpr@0..5
        VarRef@0..2
          Ident@0..2 "x1"
        Asterisk@2..3 "*"
        VarRef@3..5
          Ident@3..5 "x2"
      Asterisk@5..6 "*"
      VarRef@6..8
        Ident@6..8 "x3"
    Asterisk@8..9 "*"
    VarRef@9..11
      Ident@9..11 "x4""#]],
        );
    }

    #[test]
    fn parse_addition_and_multiplication() {
        check(
            "1+2*3",
            expect![[r#"
Root@0..5
  BinExpr@0..5
    IntLiteral@0..1
      Int@0..1 "1"
    Plus@1..2 "+"
    BinExpr@2..5
      IntLiteral@2..3
        Int@2..3 "2"
      Asterisk@3..4 "*"
      IntLiteral@4..5
        Int@4..5 "3""#]],
        );
    }

    #[test]
    fn parse_division_and_subtraction() {
        check(
            "10/9-8/7",
            expect![[r#"
Root@0..8
  BinExpr@0..8
    BinExpr@0..4
      IntLiteral@0..2
        Int@0..2 "10"
      Slash@2..3 "/"
      IntLiteral@3..4
        Int@3..4 "9"
    Hyphen@4..5 "-"
    BinExpr@5..8
      IntLiteral@5..6
        Int@5..6 "8"
      Slash@6..7 "/"
      IntLiteral@7..8
        Int@7..8 "7""#]],
        );
    }
}
