use symengine::Expression;

#[test]
fn simple_canonical() {
    let expr = Expression::new("a + b + 3");
    assert_eq!(expr.as_str(), "3 + a + b");
}

#[test]
fn equal() {
    let expr1 = Expression::new("(a + 3) * (a - 3)");
    let expr2 = Expression::new("a ** 2 - 9".to_string());
    assert_eq!(expr1, expr2);
}
