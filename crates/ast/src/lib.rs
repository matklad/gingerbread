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

def_ast_node!(Root);

impl Root {
    pub fn defs(&self) -> impl Iterator<Item = Def> {
        nodes(self)
    }

    pub fn stmts(&self) -> impl Iterator<Item = Stmt> {
        nodes(self)
    }

    pub fn tail_expr(&self) -> Option<Expr> {
        node(self)
    }
}

pub enum Def {
    FncDef(FncDef),
}

impl AstNode for Def {
    fn cast(node: SyntaxNode) -> Option<Self> {
        match node.kind() {
            SyntaxKind::FncDef => Some(Self::FncDef(FncDef(node))),

            _ => None,
        }
    }

    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::FncDef(fnc_def) => fnc_def.syntax(),
        }
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Stmt {
    LocalDef(LocalDef),
    ExprStmt(ExprStmt),
}

impl AstNode for Stmt {
    fn cast(node: SyntaxNode) -> Option<Self> {
        match node.kind() {
            SyntaxKind::LocalDef => Some(Self::LocalDef(LocalDef(node))),
            SyntaxKind::ExprStmt => Some(Self::ExprStmt(ExprStmt(node))),
            _ => None,
        }
    }

    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::LocalDef(local_def) => local_def.syntax(),
            Self::ExprStmt(expr) => expr.syntax(),
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

def_ast_node!(ExprStmt);

impl ExprStmt {
    pub fn expr(&self) -> Option<Expr> {
        node(self)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Expr {
    Bin(BinExpr),
    Block(Block),
    FncCall(FncCall),
    IntLiteral(IntLiteral),
    StringLiteral(StringLiteral),
}

impl AstNode for Expr {
    fn cast(node: SyntaxNode) -> Option<Self> {
        match node.kind() {
            SyntaxKind::BinExpr => Some(Self::Bin(BinExpr(node))),
            SyntaxKind::Block => Some(Self::Block(Block(node))),
            SyntaxKind::FncCall => Some(Self::FncCall(FncCall(node))),
            SyntaxKind::IntLiteral => Some(Self::IntLiteral(IntLiteral(node))),
            SyntaxKind::StringLiteral => Some(Self::StringLiteral(StringLiteral(node))),
            _ => None,
        }
    }

    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::Bin(bin_expr) => bin_expr.syntax(),
            Self::Block(block) => block.syntax(),
            Self::FncCall(fnc_call) => fnc_call.syntax(),
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

    pub fn tail_expr(&self) -> Option<Expr> {
        node(self)
    }
}

def_ast_node!(FncCall);

impl FncCall {
    pub fn name(&self) -> Option<Ident> {
        token(self)
    }

    pub fn arg_list(&self) -> Option<ArgList> {
        node(self)
    }
}

def_ast_node!(ArgList);

impl ArgList {
    pub fn args(&self) -> impl Iterator<Item = Arg> {
        nodes(self)
    }
}

def_ast_node!(Arg);

impl Arg {
    pub fn value(&self) -> Option<Expr> {
        node(self)
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

    fn parse(input: &str) -> Root {
        let syntax = parser::parse_repl_line(&lexer::lex(input)).syntax_node();
        Root::cast(syntax).unwrap()
    }

    #[test]
    fn cast_root() {
        parse("");
    }

    #[test]
    fn get_stmts() {
        let root = parse("let a = b; a;");
        assert_eq!(root.stmts().count(), 2);
    }

    #[test]
    fn inspect_stmt_kind() {
        let root = parse("let foo = bar; baz * quuz;");
        let mut stmts = root.stmts();

        assert!(matches!(stmts.next(), Some(Stmt::LocalDef(_))));
        assert!(matches!(stmts.next(), Some(Stmt::ExprStmt(_))));
        assert!(stmts.next().is_none());
    }

    #[test]
    fn get_name_of_local_def() {
        let root = parse("let a = 10;");
        let stmt = root.stmts().next().unwrap();

        let local_def = match stmt {
            Stmt::LocalDef(local_def) => local_def,
            _ => unreachable!(),
        };

        assert_eq!(local_def.name().unwrap().text(), "a");
    }

    #[test]
    fn get_value_of_local_def() {
        let root = parse("let foo = 5;");
        let stmt = root.stmts().next().unwrap();

        let local_def = match stmt {
            Stmt::LocalDef(local_def) => local_def,
            _ => unreachable!(),
        };

        assert!(matches!(local_def.value(), Some(Expr::IntLiteral(_))));
    }

    #[test]
    fn get_lhs_and_rhs_of_bin_expr() {
        let root = parse("foo * 2");
        assert!(root.stmts().next().is_none());

        let bin_expr = match root.tail_expr() {
            Some(Expr::Bin(bin_expr)) => bin_expr,
            _ => unreachable!(),
        };

        assert!(matches!(bin_expr.lhs(), Some(Expr::FncCall(_))));
        assert!(matches!(bin_expr.rhs(), Some(Expr::IntLiteral(_))));
    }

    #[test]
    fn get_operator_of_bin_expr() {
        let root = parse("a + b");

        let bin_expr = match root.tail_expr() {
            Some(Expr::Bin(bin_expr)) => bin_expr,
            _ => unreachable!(),
        };

        assert!(matches!(bin_expr.op(), Some(Op::Add(_))));
    }

    #[test]
    fn get_name_of_fnc_call() {
        let root = parse("idx");

        let fnc_call = match root.tail_expr() {
            Some(Expr::FncCall(fnc_call)) => fnc_call,
            _ => unreachable!(),
        };

        assert_eq!(fnc_call.name().unwrap().text(), "idx");
    }

    #[test]
    fn get_args_of_fnc_call() {
        let root = parse("mul 10, 20");

        let fnc_call = match root.tail_expr() {
            Some(Expr::FncCall(fnc_call)) => fnc_call,
            _ => unreachable!(),
        };

        let mut args = fnc_call.arg_list().unwrap().args();

        assert_eq!(args.next().unwrap().value().unwrap().syntax().to_string(), "10");
        assert_eq!(args.next().unwrap().value().unwrap().syntax().to_string(), "20");
        assert!(args.next().is_none());
    }

    #[test]
    fn get_value_of_int_literal() {
        let root = parse("92");

        let int_literal = match root.tail_expr() {
            Some(Expr::IntLiteral(int_literal)) => int_literal,
            _ => unreachable!(),
        };

        assert_eq!(int_literal.value().unwrap().text(), "92");
    }

    #[test]
    fn get_value_of_string_literal() {
        let root = parse("\"👀\"");

        let string_literal = match root.tail_expr() {
            Some(Expr::StringLiteral(string_literal)) => string_literal,
            _ => unreachable!(),
        };

        assert_eq!(string_literal.value().unwrap().text(), "\"👀\"");
    }

    #[test]
    fn get_block_stmts_and_tail_expr() {
        let root = parse("{ let a = 10; let b = a * {a - 1}; b + 5 }");

        let block = match root.tail_expr() {
            Some(Expr::Block(block)) => block,
            _ => unreachable!(),
        };

        let mut stmts = block.stmts();

        assert!(matches!(stmts.next(), Some(Stmt::LocalDef(_))));
        assert!(matches!(stmts.next(), Some(Stmt::LocalDef(_))));
        assert!(stmts.next().is_none());

        assert!(matches!(block.tail_expr(), Some(Expr::Bin(_))));
    }

    #[test]
    fn get_fnc_def_name() {
        let root = parse("fnc a() -> {};");
        let def = root.defs().next().unwrap();

        let Def::FncDef(fnc_def) = def;

        assert_eq!(fnc_def.name().unwrap().text(), "a");
    }

    #[test]
    fn get_fnc_def_params() {
        let root = parse("fnc add(x: s32, y: s32) -> {};");
        let def = root.defs().next().unwrap();

        let Def::FncDef(fnc_def) = def;

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
        let root = parse("fnc four(): s32 -> 4;");
        let def = root.defs().next().unwrap();

        let Def::FncDef(fnc_def) = def;

        assert_eq!(fnc_def.ret_ty().unwrap().ty().unwrap().name().unwrap().text(), "s32");
    }

    #[test]
    fn get_fnc_def_body() {
        let root = parse("fnc nothing() -> {};");
        let def = root.defs().next().unwrap();

        let Def::FncDef(fnc_def) = def;

        let block = match fnc_def.body().unwrap() {
            Expr::Block(block) => block,
            _ => unreachable!(),
        };

        assert!(block.stmts().next().is_none());
    }
}