#![feature(rustc_private)]

extern crate aster;
extern crate syntax;

use syntax::ast;
use syntax::codemap::{DUMMY_SP, Spanned};
use syntax::ptr::P;

use aster::AstBuilder;

#[test]
fn test_lit() {
    let builder = AstBuilder::new();

    fn check(expr: P<ast::Expr>, lit: P<ast::Lit>) {
        assert_eq!(
            expr,
            P(ast::Expr {
                id: ast::DUMMY_NODE_ID,
                node: ast::ExprLit(lit),
                span: DUMMY_SP,
            })
        );
    }

    check(builder.expr().int(5), builder.lit().int(5));

    check(builder.expr().i8(5), builder.lit().i8(5));
    check(builder.expr().i16(5), builder.lit().i16(5));
    check(builder.expr().i32(5), builder.lit().i32(5));
    check(builder.expr().i64(5), builder.lit().i64(5));
    check(builder.expr().isize(5), builder.lit().isize(5));

    check(builder.expr().u8(5), builder.lit().u8(5));
    check(builder.expr().u16(5), builder.lit().u16(5));
    check(builder.expr().u32(5), builder.lit().u32(5));
    check(builder.expr().u64(5), builder.lit().u64(5));
    check(builder.expr().usize(5), builder.lit().usize(5));

    check(builder.expr().str("string"), builder.lit().str("string"));
}

#[test]
fn test_bin() {
    let builder = AstBuilder::new();

    assert_eq!(
        builder.expr().add().i8(1).i8(2),
        P(ast::Expr {
            id: ast::DUMMY_NODE_ID,
            node: ast::ExprBinary(
                Spanned {
                    span: DUMMY_SP,
                    node: ast::BiAdd,
                },
                builder.expr().i8(1),
                builder.expr().i8(2),
            ),
            span: DUMMY_SP,
        })
    );
}

#[test]
fn test_unit() {
    let builder = AstBuilder::new();

    assert_eq!(
        builder.expr().unit(),
        P(ast::Expr {
            id: ast::DUMMY_NODE_ID,
            node: ast::ExprTup(vec![]),
            span: DUMMY_SP,
        })
    );

    assert_eq!(
        builder.expr().tuple().build(),
        P(ast::Expr {
            id: ast::DUMMY_NODE_ID,
            node: ast::ExprTup(vec![]),
            span: DUMMY_SP,
        })
    );
}

#[test]
fn test_tuple() {
    let builder = AstBuilder::new();

    let expr = builder.expr().tuple()
        .expr().i8(1)
        .expr().tuple()
            .expr().unit()
            .expr().isize(2)
            .build()
        .build();

    assert_eq!(
        expr,
        P(ast::Expr {
            id: ast::DUMMY_NODE_ID,
            node: ast::ExprTup(vec![
                builder.expr().i8(1),
                P(ast::Expr {
                    id: ast::DUMMY_NODE_ID,
                    node: ast::ExprTup(vec![
                        builder.expr().unit(),
                        builder.expr().isize(2),
                    ]),
                    span: DUMMY_SP,
                })
            ]),
            span: DUMMY_SP,
        })
    );
}
