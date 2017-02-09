
use ast::Expression;
use ast::Expression::*;
use object::Object;
use object::Object::*;

fn eval(node: Expression) -> Object {
    match node {
        IntegerExpression{ value } => Int(value as i64),
        IdentifierExpression { value } => Str(value),
        BooleanExpression { value } => Bool(value),
        _ => Null,
    }
}


#[test]
fn eval_test() {

    assert_eq!(Int(32), eval(IntegerExpression{ value: 32 }));
    assert_eq!(Str("foo".to_string()), eval(IdentifierExpression{ value: "foo".to_string() }));
    assert_eq!(Bool(true), eval(BooleanExpression{ value: true }));
}