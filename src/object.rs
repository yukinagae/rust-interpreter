#![allow(unused_imports)]
#![allow(dead_code)]

use std::fmt;
use self::Object::*;

#[derive(Debug)]
pub enum Object {
    Int(i64),
    Str(String),
    Bool(bool),
    Null,
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Int(v) => write!(f, "{}", v.to_string()),
            Str(ref v) => write!(f, "{}", v.to_string()),
            Bool(v) => write!(f, "{}", v.to_string()),
            Null => write!(f, "null"),
        }

    }
}


#[test]
#[ignore]
fn object_to_string_test() {
    assert_eq!("12".to_string(), Int(12).to_string());
    assert_eq!("foo".to_string(), Str("foo".to_string()).to_string());
    assert_eq!("true".to_string(), Bool(true).to_string());
    assert_eq!("false".to_string(), Bool(false).to_string());
    assert_eq!("null".to_string(), Null.to_string());
}
