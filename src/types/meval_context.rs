use std::f64;

use chrono::{Datelike, NaiveDate, Utc};
use meval::{ContextProvider, FuncEvalError};


pub struct MevalContext {}

impl MevalContext {
    pub fn new() -> Self {
        Self{}
    }

    pub fn eval(&self, expression: impl AsRef<str>) -> Result<f64, meval::Error> {
        meval::eval_str_with_context(expression, self)
    }
}

impl ContextProvider for MevalContext {
    fn get_var(&self, name: &str) -> Option<f64> {
        let sophie_age = calc_age(NaiveDate::from_ymd_opt(2024, 2, 1));

        match name {
            "pi" => Some(f64::consts::PI),
            "e" => Some(f64::consts::E),
            "nan" => Some(f64::NAN),
            "inf" => Some(f64::INFINITY),

            "sophie_age" => Some(sophie_age),
            
            _ => None
        }
    }

    fn eval_func(&self, name: &str, args: &[f64]) -> Result<f64, FuncEvalError> {
        match name {
            "avg" => avarage(args),
            "avgw" => avarage_with_weight(args),

            "if" => if_func(args),
            "bool" => call_fn(bool_func, args),
            "and" => call_fn2(and_func, args),
            "or" => call_fn2(or_func, args),
            "not" => call_fn(not_func, args),
            "xor" => call_fn2(xor_func, args),

            "eq" => call_fn2(eq_func, args),
            "neq" => call_fn2(neq_func, args),
            "gt" => call_fn2(gt_func, args),
            "gte" => call_fn2(gte_func, args),
            "lt" => call_fn2(lt_func, args),
            "lte" => call_fn2(lte_func, args),

            "sqrt" => call_fn(f64::sqrt, args),
            "exp" => call_fn(f64::exp, args),
            "ln" => call_fn(f64::ln, args),
            "abs" => call_fn(f64::abs, args),
            "sin" => call_fn(f64::sin, args),
            "cos" => call_fn(f64::cos, args),
            "tan" => call_fn(f64::tan, args),
            "asin" => call_fn(f64::asin, args),
            "acos" => call_fn(f64::acos, args),
            "atan" => call_fn(f64::atan, args),
            "sinh" => call_fn(f64::sinh, args),
            "cosh" => call_fn(f64::cosh, args),
            "tanh" => call_fn(f64::tanh, args),
            "asinh" => call_fn(f64::asinh, args),
            "acosh" => call_fn(f64::acosh, args),
            "atanh" => call_fn(f64::atanh, args),
            "floor" => call_fn(f64::floor, args),
            "ceil" => call_fn(f64::ceil, args),
            "round" => call_fn(f64::round, args),
            "signum" => call_fn(f64::signum, args),
            "atan2" => atan2(args),
            "max" => max(args),
            "min" => min(args),

            _ => Err(FuncEvalError::UnknownFunction)
        }
    }
}

fn call_fn(func: fn(f64) -> f64, args: &[f64]) -> Result<f64, FuncEvalError> {
    if args.len() != 1 {
        return Err(FuncEvalError::NumberArgs(1usize));
    }

    Ok(func(args[0]))
}

fn call_fn2(func: fn(f64, f64) -> f64, args: &[f64]) -> Result<f64, FuncEvalError> {
    if args.len() != 2 {
        return Err(FuncEvalError::NumberArgs(2usize));
    }

    Ok(func(args[0], args[1]))
}

fn calc_age(birth: Option<NaiveDate>) -> f64 {
    if birth.is_none() {
        return 0.0;
    }

    let birth = birth.unwrap();
    let now = Utc::now().naive_utc().date();
    let age = (now.year() - birth.year()) as f64;

    if now.month() < birth.month() || (now.month() == birth.month() && now.day() < birth.day()) {
        age - 1.0
    } else {
        age
    }
}

fn avarage(args: &[f64]) -> Result<f64, FuncEvalError> {
    Ok(args.iter().sum::<f64>() / args.len() as f64)
}

fn avarage_with_weight(args: &[f64]) -> Result<f64, FuncEvalError> {
    if (args.len() % 2) != 0 {
        return Err(FuncEvalError::TooFewArguments);
    }

    let mut sum = 0.0;
    let mut weight = 0.0;

    for i in (0..args.len()).step_by(2) {
        sum += args[i] * args[i + 1];
        weight += args[i + 1];
    }

    Ok(sum / weight)
}

fn atan2(args: &[f64]) -> Result<f64, FuncEvalError> {
    if args.len() != 2 {
        return Err(FuncEvalError::NumberArgs(2usize));
    }

    Ok(f64::atan2(args[0], args[1]))
}

fn max(args: &[f64]) -> Result<f64, FuncEvalError> {
    Ok(args.iter().cloned().fold(f64::NEG_INFINITY, f64::max))
}

fn min(args: &[f64]) -> Result<f64, FuncEvalError> {
    Ok(args.iter().cloned().fold(f64::INFINITY, f64::min))
}

fn f64_to_bool(num: f64) -> bool {
    num != 0.0 && !num.is_nan()
}

fn bool_to_f64(b: bool) -> f64 {
    if b { 1.0 } else { 0.0 }
}

fn if_func(args: &[f64]) -> Result<f64, FuncEvalError> {
    if args.len() < 2 {
        return Err(FuncEvalError::TooFewArguments);
    } else if args.len() > 3 {
        return Err(FuncEvalError::TooManyArguments);
    }

    Ok(if args[0] != 0.0 { args[1] } else { *args.get(2).unwrap_or(&0.0) })
}

fn bool_func(num: f64) -> f64 {
    bool_to_f64(f64_to_bool(num))
}

fn and_func(num1: f64, num2: f64) -> f64 {
    bool_to_f64(f64_to_bool(num1) && f64_to_bool(num2))
}

fn or_func(num1: f64, num2: f64) -> f64 {
    bool_to_f64(f64_to_bool(num1) || f64_to_bool(num2))
}

fn not_func(num: f64) -> f64 {
    bool_to_f64(!f64_to_bool(num))
}

fn xor_func(num1: f64, num2: f64) -> f64 {
    bool_to_f64(f64_to_bool(num1) ^ f64_to_bool(num2))
}

fn eq_func(num1: f64, num2: f64) -> f64 {
    bool_to_f64(num1 == num2)
}

fn neq_func(num1: f64, num2: f64) -> f64 {
    bool_to_f64(num1 != num2)
}

fn gt_func(num1: f64, num2: f64) -> f64 {
    bool_to_f64(num1 > num2)
}

fn gte_func(num1: f64, num2: f64) -> f64 {
    bool_to_f64(num1 >= num2)
}

fn lt_func(num1: f64, num2: f64) -> f64 {
    bool_to_f64(num1 < num2)
}

fn lte_func(num1: f64, num2: f64) -> f64 {
    bool_to_f64(num1 <= num2)
}
