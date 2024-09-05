use std::{rc::Rc, time::{SystemTime, UNIX_EPOCH}};

use crate::{callable::NativeFunction, environ::Environment, error::EvalError, expr::LiteralExpr};

pub fn clock(_args: Vec<LiteralExpr>) -> Result<LiteralExpr, EvalError> {
    let start = SystemTime::now();
    let since_the_epoch = start.duration_since(UNIX_EPOCH).expect("Time went backwards");
    let seconds = since_the_epoch.as_secs_f64();
    Ok(LiteralExpr::Number(seconds))
}

pub fn define_native_functions(environment: &mut Environment) {
    let clock_function = NativeFunction::new("clock", 0, clock);
    environment.define("clock".to_string(), LiteralExpr::Callable(Rc::new(clock_function)));
}
