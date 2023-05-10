use crate::prelude::*;
use surrealdb::sql::{Array, Object, Value};

impl TryFrom<W<Value>> for Object {
    type Error = Error;
    fn try_from(val: W<Value>) -> Result<Object, Error> {
        match val.0 {
            Value::Object(obj) => Ok(obj),
            _ => Err(Error::XValueNotOfType("Object")),
        }
    }
}

impl TryFrom<W<Value>> for Array {
    type Error = Error;
    fn try_from(val: W<Value>) -> Result<Array, Error> {
        match val.0 {
            Value::Array(obj) => Ok(obj),
            _ => Err(Error::XValueNotOfType("Array")),
        }
    }
}
impl TryFrom<W<Value>> for usize {
    type Error = Error;
    fn try_from(val: W<Value>) -> Result<usize, Error> {
        match val.0 {
            Value::Number(obj) => Ok(obj.as_usize()),
            _ => Err(Error::XValueNotOfType("usize")),
        }
    }
}
impl TryFrom<W<Value>> for f64 {
    type Error = Error;
    fn try_from(val: W<Value>) -> Result<f64, Error> {
        match val.0 {
            Value::Number(obj) => Ok(obj.as_float()),
            _ => Err(Error::XValueNotOfType("f64")),
        }
    }
}

impl TryFrom<W<Value>> for i64 {
    type Error = Error;
    fn try_from(val: W<Value>) -> Result<i64, Error> {
        match val.0 {
            Value::Number(obj) => Ok(obj.as_int()),
            _ => Err(Error::XValueNotOfType("i64")),
        }
    }
}

impl TryFrom<W<Value>> for bool {
    type Error = Error;
    fn try_from(val: W<Value>) -> Result<bool, Error> {
        match val.0 {
            Value::False => Ok(false),
            Value::True => Ok(true),
            _ => Err(Error::XValueNotOfType("bool")),
        }
    }
}

impl TryFrom<W<Value>> for String {
    type Error = Error;
    fn try_from(val: W<Value>) -> Result<String, Error> {
        match val.0 {
            Value::Strand(strand) => Ok(strand.as_string()),
            Value::Thing(thing) => Ok(thing.to_string()),
            _ => Err(Error::XValueNotOfType("String")),
        }
    }
}
