use super::ast::*;
use super::infra::*;
use super::lexer::*;
use bimap::BiMap;
use std::iter::Peekable;

use either::Either;
use LoopCtrl::*;

pub trait IntoParser<'a> {
    fn into_parser(self) -> Parser<'a>;
}

impl<'a> IntoParser<'a> for LexerIterator<'a> {
    fn into_parser(self) -> Parser<'a> {
        Parser::new(self)
    }
}

pub trait TokenIterator<'a>: Iterator<Item = Token<'a>> + itertools::PeekingNext {
    fn expect(&mut self, token: TokenVariant<'a>) -> ParseResult<'a, Token<'a>> {
        // * separated variables because lifetime concerns.
        match self.next() {
            Some(t) => {
                if variant_eq(&t.var, &token) {
                    Ok(t)
                } else {
                    Err(parse_err(ParseErrVariant::ExpectToken(token), t.span))
                }
            }
            None => Err(parse_err(ParseErrVariant::ExpectToken(token), Span::zero())),
        }
    }

    fn expect_map_or<T>(
        &mut self,
        token: TokenVariant<'a>,
        map: impl FnOnce(Token<'a>) -> T,
        f: impl FnOnce(Token<'a>) -> Result<T, ParseError<'a>>,
    ) -> ParseResult<'a, T> {
        let next = self.next();
        match next {
            Some(v) => {
                if variant_eq(&v.var, &token) {
                    Ok(map(v))
                } else {
                    f(v)
                }
            }
            None => Err(parse_err(ParseErrVariant::ExpectToken(token), Span::zero())),
        }
    }

    fn try_consume(&mut self, token: TokenVariant<'a>) -> bool {
        match self.peeking_next(|v| variant_eq(&v.var, &token)) {
            Some(_) => true,
            None => false,
        }
    }

    fn try_consume_log_span(&mut self, token: TokenVariant<'a>) -> Option<Span> {
        match self.peeking_next(|v| variant_eq(&v.var, &token)) {
            Some(v) => Some(v.span),
            None => None,
        }
    }
}

type LexerWrapped<'a> = Peekable<LexerIterator<'a>>;

impl<'a> TokenIterator<'a> for LexerWrapped<'a> {}

pub struct TypeVar {
    types: Vec<TypeDef>,
    type_names: BiMap<usize, String>,
    vars: Vec<VarDef>,
    var_names: BiMap<usize, String>,
}

impl TypeVar {
    pub fn new() -> TypeVar {
        TypeVar {
            types: Vec::new(),
            type_names: BiMap::new(),
            vars: Vec::new(),
            var_names: BiMap::new(),
        }
    }

    pub fn insert_type(&mut self, type_name: &str, type_def: TypeDef) -> usize {
        unimplemented!()
    }
}

pub struct Parser<'a> {
    lexer: LexerWrapped<'a>,
    type_var: TypeVar,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: LexerIterator<'a>) -> Parser<'a> {
        Parser {
            lexer: lexer.peekable(),
            type_var: TypeVar::new(),
        }
    }

    fn p_stmt_or_expr(&mut self, scope: Ptr<Scope>) -> ParseResult<Either<Stmt, Expr>> {
        let next = self.lexer.peek().unwrap();

        match next.var {
            TokenVariant::While => self.p_while_stmt(scope).map(|inner| Either::Left(inner)),
            TokenVariant::If => self.p_if_expr(scope).map(|inner| Either::Right(inner)),
            TokenVariant::Identifier(i) => match scope.borrow().find_def(i) {
                None => Err(parse_err(
                    ParseErrVariant::CannotFindIdent(i.to_owned()),
                    next.span,
                )),
                Some(def) => match &*def.borrow() {
                    SymbolDef::Typ { .. } => self
                        .p_decl_stmt(scope.clone())
                        .map(|inner| Either::Left(inner)),
                    SymbolDef::Var { .. } => {
                        self.p_expr(scope.clone()).map(|inner| Either::Right(inner))
                    }
                },
            },
            _ => Err(parse_err(
                ParseErrVariant::UnexpectedToken(next.var.clone()),
                next.span,
            )),
        }
        // match scope.borrow().find_def();

        // unimplemented!()
    }

    fn p_block_expr(&mut self, scope: Ptr<Scope>) -> ParseResult<Expr> {
        unimplemented!()
    }

    fn p_block_expr_no_scope(&mut self, scope: Ptr<Scope>) -> ParseResult<Expr> {
        unimplemented!()
    }

    fn p_fn(&mut self, scope: Ptr<Scope>) -> ParseResult<Stmt> {
        unimplemented!()
    }

    fn p_while_stmt(&mut self, scope: Ptr<Scope>) -> ParseResult<Stmt> {
        unimplemented!()
    }

    fn p_if_expr(&mut self, scope: Ptr<Scope>) -> ParseResult<Expr> {
        unimplemented!()
    }

    fn p_decl_stmt(&mut self, scope: Ptr<Scope>) -> ParseResult<Stmt> {
        unimplemented!()
    }

    fn p_expr(&mut self, scope: Ptr<Scope>) -> ParseResult<Expr> {
        let stack = Vec::<Expr>::new();
        unimplemented!();
    }

    fn p_literal(&mut self) -> ParseResult<Expr> {
        let t = self.lexer.next().unwrap();
        match t.var {
            TokenVariant::IntegerLiteral(i) => Ok(Expr {
                var: ExprVariant::Literal(Literal::Integer { val: i }),
                span: t.span,
            }),
            TokenVariant::StringLiteral(s) => Ok(Expr {
                var: ExprVariant::Literal(Literal::String { val: s }),
                span: t.span,
            }),
            TokenVariant::BooleanLiteral(b) => Ok(Expr {
                var: ExprVariant::Literal(Literal::Boolean { val: b }),
                span: t.span,
            }),
            _ => Err(parse_err(ParseErrVariant::InternalErr, t.span)),
        }
    }
}
