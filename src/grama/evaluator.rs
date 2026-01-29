use crate::grama::gramma_rules::{Expr, Stmt, Program, BinOp, UnaryOp, TemplatePart};
use crate::grama::value::Value;
use std::collections::HashMap;

/// Runtime errors that can occur during evaluation
#[derive(Debug, Clone)]
pub struct RuntimeError {
    pub message: String,
}

impl std::fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for RuntimeError {}

impl RuntimeError {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }

    pub fn undefined_variable(name: &str) -> Self {
        Self::new(format!("Undefined variable: '{}'", name))
    }

    pub fn type_error(expected: &str, got: &str, operation: &str) -> Self {
        Self::new(format!(
            "Type error: expected {} for {}, got {}",
            expected, operation, got
        ))
    }

    pub fn unknown_function(name: &str) -> Self {
        Self::new(format!("Unknown function: '{}'", name))
    }

    pub fn wrong_arg_count(name: &str, expected: usize, got: usize) -> Self {
        Self::new(format!(
            "Function '{}' expects {} argument(s), got {}",
            name, expected, got
        ))
    }
}

pub type EvalResult<T> = Result<T, RuntimeError>;

/// Environment for storing variables
#[derive(Debug, Clone)]
pub struct Environment {
    variables: HashMap<String, Value>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
        }
    }

    pub fn define(&mut self, name: String, value: Value) {
        self.variables.insert(name, value);
    }

    pub fn assign(&mut self, name: &str, value: Value) -> EvalResult<()> {
        if self.variables.contains_key(name) {
            self.variables.insert(name.to_string(), value);
            Ok(())
        } else {
            Err(RuntimeError::undefined_variable(name))
        }
    }

    pub fn get(&self, name: &str) -> EvalResult<Value> {
        self.variables
            .get(name)
            .cloned()
            .ok_or_else(|| RuntimeError::undefined_variable(name))
    }
}

/// Evaluator for executing C.env programs
pub struct Evaluator {
    env: Environment,
    output: Vec<String>,
}

impl Evaluator {
    pub fn new() -> Self {
        Self {
            env: Environment::new(),
            output: Vec::new(),
        }
    }

    /// Execute a program and return the output
    pub fn eval_program(&mut self, program: &Program) -> EvalResult<Vec<String>> {
        self.output.clear();

        for stmt in &program.items {
            self.eval_statement(stmt)?;
        }

        Ok(self.output.clone())
    }

    fn eval_statement(&mut self, stmt: &Stmt) -> EvalResult<()> {
        match stmt {
            Stmt::VarDecl { name, value, .. } => {
                let val = self.eval_expr(value)?;
                self.env.define(name.clone(), val);
                Ok(())
            }
            Stmt::Assignment { target, value } => {
                let val = self.eval_expr(value)?;
                self.env.assign(target, val)
            }
            Stmt::ExprStmt(expr) => {
                self.eval_expr(expr)?;
                Ok(())
            }
        }
    }

    fn eval_expr(&mut self, expr: &Expr) -> EvalResult<Value> {
        match expr {
            Expr::Number(n) => Ok(Value::Number(*n)),
            Expr::Bool(b) => Ok(Value::Bool(*b)),
            Expr::Null => Ok(Value::Null),
            Expr::StringLiteral(s) => Ok(Value::String(s.clone())),

            Expr::Ident(name) => self.env.get(name),

            Expr::Template(parts) => {
                let mut result = String::new();
                for part in parts {
                    match part {
                        TemplatePart::Text(text) => result.push_str(text),
                        TemplatePart::Expr(expr) => {
                            let val = self.eval_expr(expr)?;
                            result.push_str(&val.to_string());
                        }
                    }
                }
                Ok(Value::String(result))
            }

            Expr::Call { callee, args } => {
                self.eval_call(callee, args)
            }

            Expr::Index { target, index } => {
                let target_val = self.eval_expr(target)?;
                let index_val = self.eval_expr(index)?;
                self.eval_index(target_val, index_val)
            }

            Expr::Unary { op, rhs } => {
                let val = self.eval_expr(rhs)?;
                self.eval_unary(*op, val)
            }

            Expr::Binary { lhs, op, rhs } => {
                let left = self.eval_expr(lhs)?;
                let right = self.eval_expr(rhs)?;
                self.eval_binary(left, *op, right)
            }

            Expr::IfExpr { cond, then_, else_ } => {
                let cond_val = self.eval_expr(cond)?;
                if cond_val.is_truthy() {
                    self.eval_expr(then_)
                } else {
                    self.eval_expr(else_)
                }
            }

            // Array comprehensions and find comprehensions not yet implemented
            Expr::ArrayComp(_) => {
                Err(RuntimeError::new("Array comprehensions not yet implemented"))
            }
            Expr::FindComp(_) => {
                Err(RuntimeError::new("Find comprehensions not yet implemented"))
            }
        }
    }

    fn eval_call(&mut self, callee: &Expr, args: &[Expr]) -> EvalResult<Value> {
        // For now, only support built-in functions (identifiers)
        if let Expr::Ident(name) = callee {
            self.eval_builtin_function(name, args)
        } else {
            Err(RuntimeError::new("Only built-in functions are currently supported"))
        }
    }

    fn eval_builtin_function(&mut self, name: &str, args: &[Expr]) -> EvalResult<Value> {
        match name {
            "print" => self.builtin_print(args),
            "len" => self.builtin_len(args),
            "type" => self.builtin_type(args),
            "str" => self.builtin_str(args),
            "num" => self.builtin_num(args),
            "bool" => self.builtin_bool(args),
            _ => Err(RuntimeError::unknown_function(name)),
        }
    }

    fn builtin_print(&mut self, args: &[Expr]) -> EvalResult<Value> {
        let mut parts = Vec::new();

        for arg in args {
            let val = self.eval_expr(arg)?;
            parts.push(val.to_string());
        }

        let output = if parts.is_empty() {
            String::new()
        } else {
            parts.join(" ")
        };

        self.output.push(output);
        Ok(Value::Null)
    }

    fn builtin_len(&mut self, args: &[Expr]) -> EvalResult<Value> {
        if args.len() != 1 {
            return Err(RuntimeError::wrong_arg_count("len", 1, args.len()));
        }

        let val = self.eval_expr(&args[0])?;
        match val {
            Value::String(s) => Ok(Value::Number(s.len() as f64)),
            Value::Array(a) => Ok(Value::Number(a.len() as f64)),
            _ => Err(RuntimeError::type_error(
                "string or array",
                val.type_name(),
                "len()"
            )),
        }
    }

    fn builtin_type(&mut self, args: &[Expr]) -> EvalResult<Value> {
        if args.len() != 1 {
            return Err(RuntimeError::wrong_arg_count("type", 1, args.len()));
        }

        let val = self.eval_expr(&args[0])?;
        Ok(Value::String(val.type_name().to_string()))
    }

    fn builtin_str(&mut self, args: &[Expr]) -> EvalResult<Value> {
        if args.len() != 1 {
            return Err(RuntimeError::wrong_arg_count("str", 1, args.len()));
        }

        let val = self.eval_expr(&args[0])?;
        Ok(Value::String(val.to_string()))
    }

    fn builtin_num(&mut self, args: &[Expr]) -> EvalResult<Value> {
        if args.len() != 1 {
            return Err(RuntimeError::wrong_arg_count("num", 1, args.len()));
        }

        let val = self.eval_expr(&args[0])?;
        match val {
            Value::Number(n) => Ok(Value::Number(n)),
            Value::String(s) => {
                s.parse::<f64>()
                    .map(Value::Number)
                    .map_err(|_| RuntimeError::new(format!("Cannot convert '{}' to number", s)))
            }
            Value::Bool(b) => Ok(Value::Number(if b { 1.0 } else { 0.0 })),
            Value::Null => Ok(Value::Number(0.0)),
            Value::Array(_) => Err(RuntimeError::type_error(
                "number, string, or boolean",
                "array",
                "num()"
            )),
        }
    }

    fn builtin_bool(&mut self, args: &[Expr]) -> EvalResult<Value> {
        if args.len() != 1 {
            return Err(RuntimeError::wrong_arg_count("bool", 1, args.len()));
        }

        let val = self.eval_expr(&args[0])?;
        Ok(Value::Bool(val.is_truthy()))
    }

    fn eval_index(&self, target: Value, index: Value) -> EvalResult<Value> {
        match (target, index) {
            (Value::Array(arr), Value::Number(n)) => {
                let idx = n as usize;
                arr.get(idx)
                    .cloned()
                    .ok_or_else(|| RuntimeError::new(format!("Index {} out of bounds", idx)))
            }
            (Value::String(s), Value::Number(n)) => {
                let idx = n as usize;
                s.chars()
                    .nth(idx)
                    .map(|c| Value::String(c.to_string()))
                    .ok_or_else(|| RuntimeError::new(format!("Index {} out of bounds", idx)))
            }
            (target, index) => Err(RuntimeError::type_error(
                "array or string with numeric index",
                &format!("{}[{}]", target.type_name(), index.type_name()),
                "indexing"
            )),
        }
    }

    fn eval_unary(&self, op: UnaryOp, val: Value) -> EvalResult<Value> {
        match op {
            UnaryOp::Plus => match val {
                Value::Number(n) => Ok(Value::Number(n)),
                _ => Err(RuntimeError::type_error("number", val.type_name(), "unary +")),
            },
            UnaryOp::Minus => match val {
                Value::Number(n) => Ok(Value::Number(-n)),
                _ => Err(RuntimeError::type_error("number", val.type_name(), "unary -")),
            },
        }
    }

    fn eval_binary(&self, left: Value, op: BinOp, right: Value) -> EvalResult<Value> {
        match op {
            BinOp::Add => match (left, right) {
                (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a + b)),
                (Value::String(a), Value::String(b)) => Ok(Value::String(format!("{}{}", a, b))),
                (a, b) => Err(RuntimeError::type_error(
                    "number + number or string + string",
                    &format!("{} + {}", a.type_name(), b.type_name()),
                    "addition"
                )),
            },
            BinOp::Sub => self.numeric_binop(left, right, |a, b| a - b, "-"),
            BinOp::Mul => self.numeric_binop(left, right, |a, b| a * b, "*"),
            BinOp::Div => self.numeric_binop(left, right, |a, b| a / b, "/"),
            BinOp::Mod => self.numeric_binop(left, right, |a, b| a % b, "%"),

            BinOp::Lt => self.comparison_binop(left, right, |a, b| a < b),
            BinOp::Le => self.comparison_binop(left, right, |a, b| a <= b),
            BinOp::Gt => self.comparison_binop(left, right, |a, b| a > b),
            BinOp::Ge => self.comparison_binop(left, right, |a, b| a >= b),

            BinOp::Eq => Ok(Value::Bool(self.values_equal(&left, &right))),
            BinOp::Ne => Ok(Value::Bool(!self.values_equal(&left, &right))),

            BinOp::And => Ok(Value::Bool(left.is_truthy() && right.is_truthy())),
            BinOp::Or => Ok(Value::Bool(left.is_truthy() || right.is_truthy())),
        }
    }

    fn numeric_binop<F>(&self, left: Value, right: Value, f: F, op_str: &str) -> EvalResult<Value>
    where
        F: FnOnce(f64, f64) -> f64,
    {
        match (left, right) {
            (Value::Number(a), Value::Number(b)) => Ok(Value::Number(f(a, b))),
            (a, b) => Err(RuntimeError::type_error(
                "number",
                &format!("{} {} {}", a.type_name(), op_str, b.type_name()),
                &format!("operator '{}'", op_str)
            )),
        }
    }

    fn comparison_binop<F>(&self, left: Value, right: Value, f: F) -> EvalResult<Value>
    where
        F: FnOnce(f64, f64) -> bool,
    {
        match (left, right) {
            (Value::Number(a), Value::Number(b)) => Ok(Value::Bool(f(a, b))),
            (a, b) => Err(RuntimeError::type_error(
                "number",
                &format!("{} vs {}", a.type_name(), b.type_name()),
                "comparison"
            )),
        }
    }

    fn values_equal(&self, a: &Value, b: &Value) -> bool {
        match (a, b) {
            (Value::Number(x), Value::Number(y)) => x == y,
            (Value::String(x), Value::String(y)) => x == y,
            (Value::Bool(x), Value::Bool(y)) => x == y,
            (Value::Null, Value::Null) => true,
            (Value::Array(x), Value::Array(y)) => {
                x.len() == y.len() && x.iter().zip(y.iter()).all(|(a, b)| self.values_equal(a, b))
            }
            _ => false,
        }
    }
}
