use symengine::{Expression, ExpressionMap};

#[test]
#[cfg(feature = "serde")]
fn serde_expr_json() {
    let expr = Expression::new("3 + a + b");

    let expr_json = serde_json::to_string(&expr).unwrap();
    let expr_clone = serde_json::from_str::<Expression>(&expr_json).unwrap();

    assert_eq!(expr, expr_clone);
}

#[test]
#[cfg(feature = "serde")]
fn serde_map_json() {
    let mut map = ExpressionMap::new();
    map.insert("a", 3.0);
    map.insert("b", -4.0);
    map.insert("c", Expression::new("a + b"));

    let map_json = serde_json::to_string(&map).unwrap();
    let map_clone = serde_json::from_str::<ExpressionMap<_>>(&map_json).unwrap();

    assert_eq!(map, map_clone);
}
