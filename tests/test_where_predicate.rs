use syntax::ast;
use syntax::codemap::DUMMY_SP;
use syntax::owned_slice::OwnedSlice;

use aster::AstBuilder;
use aster::lifetime::{IntoLifetime, IntoLifetimeDef};
use aster::path::IntoPath;

#[test]
fn test_bound() {
    let builder = AstBuilder::new();
    let predicate = builder.where_predicate().bound().id("T")
        .for_lifetime("'a").build()
        .trait_("Trait").build()
        .build();

    assert_eq!(
        predicate,
        ast::WherePredicate::BoundPredicate(ast::WhereBoundPredicate {
            span: DUMMY_SP,
            bound_lifetimes: vec![
                "'a".into_lifetime_def(),
            ],
            bounded_ty: builder.ty().id("T"),
            bounds: OwnedSlice::from_vec(vec![
                builder.ty_param_bound()
                    .trait_("Trait")
                    .build(),
            ]),
        })
    );
}

#[test]
fn test_lifetime() {
    let builder = AstBuilder::new();
    let predicate = builder.where_predicate().lifetime("'a")
        .bound("'b")
        .bound("'c")
        .build();

    assert_eq!(
        predicate,
        ast::WherePredicate::RegionPredicate(ast::WhereRegionPredicate {
            span: DUMMY_SP,
            lifetime: "'a".into_lifetime(),
            bounds: vec![
                "'b".into_lifetime(),
                "'c".into_lifetime(),
            ],
        })
    );
}

#[test]
fn test_eq() {
    let builder = AstBuilder::new();
    let predicate = builder.where_predicate().eq("T").ty().usize();

    assert_eq!(
        predicate,
        ast::WherePredicate::EqPredicate(ast::WhereEqPredicate {
            id: ast::DUMMY_NODE_ID,
            span: DUMMY_SP,
            path: "T".into_path(),
            ty: builder.ty().usize(),
        })
    );
}
