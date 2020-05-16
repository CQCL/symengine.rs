use symengine::{Expression, ExpressionMap, ExpressionMapKey};

#[test]
fn simple_subs() {
    let expr = Expression::new("a * b + 10");

    let mut map = ExpressionMap::new();
    map.insert("a", 3i64);
    map.insert("b", -4i64);

    assert_eq!(map.eval_once(&expr), -2i64);
}

#[test]
fn clone_map() {
    let expr = Expression::new("a * b + 10");

    let mut map = ExpressionMap::new();
    map.insert("a", 3i64);
    map.insert("b", -4i64);

    let map = map.clone();
    assert_eq!(map.eval_once(&expr), -2i64);
}

#[test]
fn custom_key() {
    #[derive(Clone, PartialEq, Eq, Hash)]
    enum Key<'a> {
        Variable(&'a str),
        Placeholder(&'a str),
    }

    impl<'a> ExpressionMapKey for Key<'a> {
        fn to_string(&self) -> String {
            match self {
                Self::Variable(var) => format!("var_{}", var),
                Self::Placeholder(ph) => format!("ph_{}", ph),
            }
        }
    }

    let mut map = ExpressionMap::new();
    map.insert(Key::Variable("a"), 3i64);
    map.insert(Key::Placeholder("a"), -4i64);

    assert_eq!(map.eval_once(&Expression::new("var_a * ph_a + 10")), -2i64);
}
