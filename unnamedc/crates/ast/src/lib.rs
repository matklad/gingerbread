pub mod validation;

use syntax::{SyntaxElement, SyntaxKind, SyntaxNode, SyntaxToken};
use text_size::TextRange;

pub trait AstNode: Sized {
    fn cast(node: SyntaxNode) -> Option<Self>;

    fn syntax(&self) -> &SyntaxNode;

    fn range(&self) -> TextRange {
        self.syntax().text_range()
    }
}

pub trait AstToken: Sized {
    fn cast(token: SyntaxToken) -> Option<Self>;

    fn syntax(&self) -> &SyntaxToken;

    fn text(&self) -> &str {
        self.syntax().text()
    }

    fn range(&self) -> TextRange {
        self.syntax().text_range()
    }
}

macro_rules! def_ast_node {
    ($kind:ident) => {
        #[derive(Debug, Clone, PartialEq, Eq, Hash)]
        pub struct $kind(SyntaxNode);

        impl AstNode for $kind {
            fn cast(node: SyntaxNode) -> Option<Self> {
                (node.kind() == SyntaxKind::$kind).then(|| Self(node))
            }

            fn syntax(&self) -> &SyntaxNode {
                &self.0
            }
        }
    };
}

macro_rules! def_ast_token {
    ($kind:ident) => {
        #[derive(Debug, Clone, PartialEq, Eq, Hash)]
        pub struct $kind(SyntaxToken);

        impl AstToken for $kind {
            fn cast(token: SyntaxToken) -> Option<Self> {
                (token.kind() == SyntaxKind::$kind).then(|| Self(token))
            }

            fn syntax(&self) -> &SyntaxToken {
                &self.0
            }
        }
    };
}

def_ast_node!(SourceFile);

impl SourceFile {
    pub fn stmts(&self) -> impl Iterator<Item = Stmt> {
        nodes(self)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Stmt {
    LocalDef(LocalDef),
    FncDef(FncDef),
    Expr(Expr),
}

impl AstNode for Stmt {
    fn cast(node: SyntaxNode) -> Option<Self> {
        match node.kind() {
            SyntaxKind::LocalDef => Some(Self::LocalDef(LocalDef(node))),
            SyntaxKind::FncDef => Some(Self::FncDef(FncDef(node))),
            SyntaxKind::BinExpr => Some(Self::Expr(Expr::Bin(BinExpr(node)))),
            SyntaxKind::Block => Some(Self::Expr(Expr::Block(Block(node)))),
            SyntaxKind::ParenExpr => Some(Self::Expr(Expr::Paren(ParenExpr(node)))),
            SyntaxKind::VarRef => Some(Self::Expr(Expr::VarRef(VarRef(node)))),
            SyntaxKind::IntLiteral => Some(Self::Expr(Expr::IntLiteral(IntLiteral(node)))),
            SyntaxKind::StringLiteral => Some(Self::Expr(Expr::StringLiteral(StringLiteral(node)))),
            _ => None,
        }
    }

    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::LocalDef(local_def) => local_def.syntax(),
            Self::FncDef(fnc_def) => fnc_def.syntax(),
            Self::Expr(expr) => expr.syntax(),
        }
    }
}

def_ast_node!(LocalDef);

impl LocalDef {
    pub fn name(&self) -> Option<Ident> {
        token(self)
    }

    pub fn value(&self) -> Option<Expr> {
        node(self)
    }
}

def_ast_node!(FncDef);

impl FncDef {
    pub fn name(&self) -> Option<Ident> {
        token(self)
    }

    pub fn param_list(&self) -> Option<ParamList> {
        node(self)
    }

    pub fn ret_ty(&self) -> Option<RetTy> {
        node(self)
    }

    pub fn body(&self) -> Option<Expr> {
        node(self)
    }
}

def_ast_node!(ParamList);

impl ParamList {
    pub fn params(&self) -> impl Iterator<Item = Param> {
        nodes(self)
    }
}

def_ast_node!(RetTy);

impl RetTy {
    pub fn ty(&self) -> Option<Ty> {
        node(self)
    }
}

def_ast_node!(Param);

impl Param {
    pub fn name(&self) -> Option<Ident> {
        token(self)
    }

    pub fn ty(&self) -> Option<Ty> {
        node(self)
    }
}

def_ast_node!(Ty);

impl Ty {
    pub fn name(&self) -> Option<Ident> {
        token(self)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Expr {
    Bin(BinExpr),
    Block(Block),
    Paren(ParenExpr),
    VarRef(VarRef),
    IntLiteral(IntLiteral),
    StringLiteral(StringLiteral),
}

impl AstNode for Expr {
    fn cast(node: SyntaxNode) -> Option<Self> {
        match node.kind() {
            SyntaxKind::BinExpr => Some(Self::Bin(BinExpr(node))),
            SyntaxKind::Block => Some(Self::Block(Block(node))),
            SyntaxKind::ParenExpr => Some(Self::Paren(ParenExpr(node))),
            SyntaxKind::VarRef => Some(Self::VarRef(VarRef(node))),
            SyntaxKind::IntLiteral => Some(Self::IntLiteral(IntLiteral(node))),
            SyntaxKind::StringLiteral => Some(Self::StringLiteral(StringLiteral(node))),
            _ => None,
        }
    }

    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::Bin(bin_expr) => bin_expr.syntax(),
            Self::Block(block) => block.syntax(),
            Self::Paren(paren_expr) => paren_expr.syntax(),
            Self::VarRef(var_ref) => var_ref.syntax(),
            Self::IntLiteral(int_literal) => int_literal.syntax(),
            Self::StringLiteral(string_literal) => string_literal.syntax(),
        }
    }
}

def_ast_node!(BinExpr);

impl BinExpr {
    pub fn lhs(&self) -> Option<Expr> {
        node(self)
    }

    pub fn rhs(&self) -> Option<Expr> {
        nodes(self).nth(1)
    }

    pub fn op(&self) -> Option<Op> {
        token(self)
    }
}

def_ast_node!(Block);

impl Block {
    pub fn stmts(&self) -> impl Iterator<Item = Stmt> {
        nodes(self)
    }
}

def_ast_node!(ParenExpr);

impl ParenExpr {
    pub fn inner(&self) -> Option<Expr> {
        node(self)
    }
}

def_ast_node!(VarRef);

impl VarRef {
    pub fn name(&self) -> Option<Ident> {
        token(self)
    }
}

def_ast_node!(IntLiteral);

impl IntLiteral {
    pub fn value(&self) -> Option<Int> {
        token(self)
    }
}

def_ast_node!(StringLiteral);

impl StringLiteral {
    pub fn value(&self) -> Option<String> {
        token(self)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Op {
    Add(Plus),
    Sub(Hyphen),
    Mul(Asterisk),
    Div(Slash),
}

impl AstToken for Op {
    fn cast(token: SyntaxToken) -> Option<Self> {
        match token.kind() {
            SyntaxKind::Plus => Some(Self::Add(Plus(token))),
            SyntaxKind::Hyphen => Some(Self::Sub(Hyphen(token))),
            SyntaxKind::Asterisk => Some(Self::Mul(Asterisk(token))),
            SyntaxKind::Slash => Some(Self::Div(Slash(token))),
            _ => None,
        }
    }

    fn syntax(&self) -> &SyntaxToken {
        match self {
            Self::Add(plus) => plus.syntax(),
            Self::Sub(hyphen) => hyphen.syntax(),
            Self::Mul(asterisk) => asterisk.syntax(),
            Self::Div(slash) => slash.syntax(),
        }
    }
}

def_ast_token!(Plus);
def_ast_token!(Hyphen);
def_ast_token!(Asterisk);
def_ast_token!(Slash);
def_ast_token!(Ident);
def_ast_token!(Int);
def_ast_token!(String);

fn nodes<Parent: AstNode, Child: AstNode>(node: &Parent) -> impl Iterator<Item = Child> {
    node.syntax().children().filter_map(Child::cast)
}

fn node<Parent: AstNode, Child: AstNode>(node: &Parent) -> Option<Child> {
    node.syntax().children().find_map(Child::cast)
}

fn token<Node: AstNode, Token: AstToken>(node: &Node) -> Option<Token> {
    node.syntax().children_with_tokens().filter_map(SyntaxElement::into_token).find_map(Token::cast)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse(input: &str) -> SourceFile {
        let syntax = parser::parse(&lexer::lex(input)).syntax_node();
        SourceFile::cast(syntax).unwrap()
    }

    #[test]
    fn cast_source_file() {
        parse("");
    }

    #[test]
    fn get_stmts() {
        let source_file = parse("let a = b\na");
        assert_eq!(source_file.stmts().count(), 2);
    }

    #[test]
    fn inspect_stmt_and_expr_kind() {
        let source_file = parse("let foo = bar\nbaz * quuz");
        let mut stmts = source_file.stmts();
        let local_def = stmts.next().unwrap();
        let expr = stmts.next().unwrap();
        assert!(stmts.next().is_none());

        match local_def {
            Stmt::LocalDef(_) => {}
            _ => unreachable!(),
        }

        match expr {
            Stmt::Expr(Expr::Bin(_)) => {}
            _ => unreachable!(),
        }
    }

    #[test]
    fn get_name_of_local_def() {
        let source_file = parse("let a = 10");
        let stmt = source_file.stmts().next().unwrap();

        let local_def = match stmt {
            Stmt::LocalDef(local_def) => local_def,
            _ => unreachable!(),
        };

        assert_eq!(local_def.name().unwrap().text(), "a");
    }

    #[test]
    fn get_value_of_local_def() {
        let source_file = parse("let foo = 5");
        let stmt = source_file.stmts().next().unwrap();

        let local_def = match stmt {
            Stmt::LocalDef(local_def) => local_def,
            _ => unreachable!(),
        };

        match local_def.value() {
            Some(Expr::IntLiteral(_)) => {}
            _ => unreachable!(),
        }
    }

    #[test]
    fn get_inner_expr_of_paren_expr() {
        let source_file = parse("(1)");
        let stmt = source_file.stmts().next().unwrap();

        let paren_expr = match stmt {
            Stmt::Expr(Expr::Paren(paren_expr)) => paren_expr,
            _ => unreachable!(),
        };

        match paren_expr.inner() {
            Some(Expr::IntLiteral(_)) => {}
            _ => unreachable!(),
        }
    }

    #[test]
    fn get_lhs_and_rhs_of_bin_expr() {
        let source_file = parse("foo * 2");
        let stmt = source_file.stmts().next().unwrap();

        let bin_expr = match stmt {
            Stmt::Expr(Expr::Bin(bin_expr)) => bin_expr,
            _ => unreachable!(),
        };

        match bin_expr.lhs() {
            Some(Expr::VarRef(_)) => {}
            _ => unreachable!(),
        }

        match bin_expr.rhs() {
            Some(Expr::IntLiteral(_)) => {}
            _ => unreachable!(),
        }
    }

    #[test]
    fn get_operator_of_bin_expr() {
        let source_file = parse("a + b");
        let stmt = source_file.stmts().next().unwrap();

        let bin_expr = match stmt {
            Stmt::Expr(Expr::Bin(bin_expr)) => bin_expr,
            _ => unreachable!(),
        };

        match bin_expr.op() {
            Some(Op::Add(_)) => {}
            _ => unreachable!(),
        }
    }

    #[test]
    fn get_name_of_var_ref() {
        let source_file = parse("idx");
        let stmt = source_file.stmts().next().unwrap();

        let var_ref = match stmt {
            Stmt::Expr(Expr::VarRef(var_ref)) => var_ref,
            _ => unreachable!(),
        };

        assert_eq!(var_ref.name().unwrap().text(), "idx");
    }

    #[test]
    fn get_value_of_int_literal() {
        let source_file = parse("92");
        let stmt = source_file.stmts().next().unwrap();

        let int_literal = match stmt {
            Stmt::Expr(Expr::IntLiteral(int_literal)) => int_literal,
            _ => unreachable!(),
        };

        assert_eq!(int_literal.value().unwrap().text(), "92");
    }

    #[test]
    fn get_value_of_string_literal() {
        let source_file = parse("\"👀\"");
        let stmt = source_file.stmts().next().unwrap();

        let string_literal = match stmt {
            Stmt::Expr(Expr::StringLiteral(string_literal)) => string_literal,
            _ => unreachable!(),
        };

        assert_eq!(string_literal.value().unwrap().text(), "\"👀\"");
    }

    #[test]
    fn get_block_stmts() {
        let source_file = parse("{ let a = 10\nlet b = a * (a - 1)\nb + 5 }");
        let stmt = source_file.stmts().next().unwrap();

        let block = match stmt {
            Stmt::Expr(Expr::Block(block)) => block,
            _ => unreachable!(),
        };

        let mut stmts = block.stmts();

        assert!(matches!(stmts.next(), Some(Stmt::LocalDef(_))));
        assert!(matches!(stmts.next(), Some(Stmt::LocalDef(_))));
        assert!(matches!(stmts.next(), Some(Stmt::Expr(Expr::Bin(_)))));
        assert!(stmts.next().is_none());
    }

    #[test]
    fn get_fnc_def_name() {
        let source_file = parse("fnc a() -> {}");
        let stmt = source_file.stmts().next().unwrap();

        let fnc_def = match stmt {
            Stmt::FncDef(fnc_def) => fnc_def,
            _ => unreachable!(),
        };

        assert_eq!(fnc_def.name().unwrap().text(), "a");
    }

    #[test]
    fn get_fnc_def_params() {
        let source_file = parse("fnc add(x: s32, y: s32) -> {}");
        let stmt = source_file.stmts().next().unwrap();

        let fnc_def = match stmt {
            Stmt::FncDef(fnc_def) => fnc_def,
            _ => unreachable!(),
        };

        let mut params = fnc_def.param_list().unwrap().params();

        let param = params.next().unwrap();
        assert_eq!(param.name().unwrap().text(), "x");
        assert_eq!(param.ty().unwrap().name().unwrap().text(), "s32");

        let param = params.next().unwrap();
        assert_eq!(param.name().unwrap().text(), "y");
        assert_eq!(param.ty().unwrap().name().unwrap().text(), "s32");

        assert!(params.next().is_none());
    }

    #[test]
    fn get_fnc_def_ret_ty() {
        let source_file = parse("fnc four(): s32 -> 4");
        let stmt = source_file.stmts().next().unwrap();

        let fnc_def = match stmt {
            Stmt::FncDef(fnc_def) => fnc_def,
            _ => unreachable!(),
        };

        assert_eq!(fnc_def.ret_ty().unwrap().ty().unwrap().name().unwrap().text(), "s32");
    }

    #[test]
    fn get_fnc_def_body() {
        let source_file = parse("fnc nothing() -> {}");
        let stmt = source_file.stmts().next().unwrap();

        let fnc_def = match stmt {
            Stmt::FncDef(fnc_def) => fnc_def,
            _ => unreachable!(),
        };

        let block = match fnc_def.body().unwrap() {
            Expr::Block(block) => block,
            _ => unreachable!(),
        };

        assert!(block.stmts().next().is_none());
    }
}
