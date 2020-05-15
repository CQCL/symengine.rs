use symengine::{Expression, ExpressionMap};

#[test]
fn simple_subs() {
    let expr = Expression::new("a * b + 10");

    let mut map = ExpressionMap::new();
    map.insert("a", 3i64);
    map.insert("b", -4i64);

    dbg!(map.eval(&expr));
    assert_eq!(map.eval(&expr), -2i64);
}
