use crate::grama::gramma_rules::{BinOp, Expr, Program, Stmt, TemplatePart, UnaryOp};
use crate::grama::value::Value;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

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

impl Default for Environment {
    fn default() -> Self {
        Self::new()
    }
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
    public_vars: HashMap<String, Value>, // Variables that appear in .env output (without 'private')
    env_output_lines: Vec<EnvOutputLine>, // Ordered list of variables and comments for .env output
    base_path: PathBuf,
    imported_files: HashMap<String, ()>, // Track imported files to prevent circular imports
}

/// Represents a line in the .env output (either a variable or a comment)
#[derive(Debug, Clone)]
enum EnvOutputLine {
    Variable(String), // Variable name (value looked up from public_vars)
    Comment(String),  // Comment text (including the # prefix)
}

impl Default for Evaluator {
    fn default() -> Self {
        Self::new()
    }
}

impl Evaluator {
    pub fn new() -> Self {
        Self::with_module(None)
    }

    pub fn with_module(module: Option<String>) -> Self {
        let mut env = Environment::new();
        // If module is provided, add it as a special variable
        if let Some(module_val) = module {
            env.define("module".to_string(), Value::String(module_val));
        }
        Self {
            env,
            output: Vec::new(),
            public_vars: HashMap::new(),
            env_output_lines: Vec::new(),
            base_path: std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")),
            imported_files: HashMap::new(),
        }
    }

    #[allow(dead_code)]
    pub fn with_base_path(base_path: PathBuf) -> Self {
        Self {
            env: Environment::new(),
            output: Vec::new(),
            public_vars: HashMap::new(),
            env_output_lines: Vec::new(),
            base_path,
            imported_files: HashMap::new(),
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

    /// Get public variables formatted as .env file content
    pub fn get_env_output(&self) -> Vec<String> {
        self.env_output_lines
            .iter()
            .map(|line| match line {
                EnvOutputLine::Variable(name) => {
                    // Look up the variable value from public_vars
                    if let Some(value) = self.public_vars.get(name) {
                        format!("{}={}", name, self.format_env_value(value))
                    } else {
                        // Variable was removed or doesn't exist, skip it
                        String::new()
                    }
                }
                EnvOutputLine::Comment(text) => text.clone(),
            })
            .filter(|line| !line.is_empty()) // Remove empty lines from removed variables
            .collect()
    }

    /// Get public variables for testing
    #[cfg(test)]
    pub fn get_public_vars(&self) -> &HashMap<String, Value> {
        &self.public_vars
    }

    /// Format a value for .env file (quoted if needed)
    fn format_env_value(&self, value: &Value) -> String {
        match value {
            Value::String(s) => {
                // Quote strings that contain spaces or special characters
                if s.contains(' ') || s.contains('=') || s.contains('"') {
                    format!("\"{}\"", s.replace('"', "\\\""))
                } else {
                    s.clone()
                }
            }
            Value::Number(n) => n.to_string(),
            Value::Bool(b) => b.to_string(),
            Value::Null => String::new(),
            Value::Array(_) => "[]".to_string(), // Basic array representation
            Value::Object(_) => "{}".to_string(), // Basic object representation
        }
    }

    fn eval_statement(&mut self, stmt: &Stmt) -> EvalResult<()> {
        match stmt {
            Stmt::VarDecl {
                name,
                value,
                private_,
            } => {
                let val = self.eval_expr(value)?;
                self.env.define(name.clone(), val.clone());
                // If not private, add to public_vars for .env output
                if !private_ {
                    self.public_vars.insert(name.clone(), val);
                    self.env_output_lines
                        .push(EnvOutputLine::Variable(name.clone()));
                }
                Ok(())
            }
            Stmt::Assignment { target, value } => {
                let val = self.eval_expr(value)?;
                self.env.assign(target, val)
            }
            Stmt::Import { is_aws_secret, .. } => {
                // Import must be assigned to a variable
                let fn_name = if *is_aws_secret {
                    "import_aws_secret"
                } else {
                    "import"
                };
                Err(RuntimeError::new(format!(
                    "{}() must be assigned to a variable. Use: private var = {}(\"path\")",
                    fn_name, fn_name
                )))
            }
            Stmt::Block(statements) => {
                // Execute each statement in the block
                // Blocks share the same scope (no new scope created)
                for stmt in statements {
                    self.eval_statement(stmt)?;
                }
                Ok(())
            }
            Stmt::Comment(text) => {
                // Comments are preserved in .env output
                self.env_output_lines
                    .push(EnvOutputLine::Comment(text.clone()));
                Ok(())
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

            Expr::Call { callee, args } => self.eval_call(callee, args),

            Expr::Index { target, index } => {
                let target_val = self.eval_expr(target)?;
                let index_val = self.eval_expr(index)?;
                self.eval_index(target_val, index_val)
            }

            Expr::Member { object, field } => {
                let obj_val = self.eval_expr(object)?;
                match obj_val {
                    Value::Object(map) => map.get(field).cloned().ok_or_else(|| {
                        RuntimeError::new(format!("Object has no field '{}'", field))
                    }),
                    _ => Err(RuntimeError::new(format!(
                        "Cannot access field '{}' on non-object type {}",
                        field,
                        obj_val.type_name()
                    ))),
                }
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
            Expr::ArrayComp(_) => Err(RuntimeError::new(
                "Array comprehensions not yet implemented",
            )),
            Expr::FindComp(_) => Err(RuntimeError::new("Find comprehensions not yet implemented")),
        }
    }

    fn eval_call(&mut self, callee: &Expr, args: &[Expr]) -> EvalResult<Value> {
        // For now, only support built-in functions (identifiers)
        if let Expr::Ident(name) = callee {
            self.eval_builtin_function(name, args)
        } else {
            Err(RuntimeError::new(
                "Only built-in functions are currently supported",
            ))
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
            "import" => self.builtin_import(args, false),
            "import_aws_secret" => self.builtin_import(args, true),
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
            Value::Object(o) => Ok(Value::Number(o.len() as f64)),
            _ => Err(RuntimeError::type_error(
                "string, array, or object",
                val.type_name(),
                "len()",
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
            Value::String(s) => s
                .parse::<f64>()
                .map(Value::Number)
                .map_err(|_| RuntimeError::new(format!("Cannot convert '{}' to number", s))),
            Value::Bool(b) => Ok(Value::Number(if b { 1.0 } else { 0.0 })),
            Value::Null => Ok(Value::Number(0.0)),
            Value::Array(_) | Value::Object(_) => Err(RuntimeError::type_error(
                "number, string, or boolean",
                val.type_name(),
                "num()",
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

    fn builtin_import(&mut self, args: &[Expr], is_aws_secret: bool) -> EvalResult<Value> {
        if args.len() != 1 {
            let fn_name = if is_aws_secret {
                "import_aws_secret"
            } else {
                "import"
            };
            return Err(RuntimeError::wrong_arg_count(fn_name, 1, args.len()));
        }

        // Evaluate the path argument
        let path_value = self.eval_expr(&args[0])?;
        let path_str = match path_value {
            Value::String(s) => s,
            _ => {
                let fn_name = if is_aws_secret {
                    "import_aws_secret"
                } else {
                    "import"
                };
                return Err(RuntimeError::new(format!(
                    "{}() expects a string path, got {}",
                    fn_name,
                    path_value.type_name()
                )));
            }
        };

        // Call eval_import which now returns Value::Object
        self.eval_import(&path_str, is_aws_secret)
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
                "indexing",
            )),
        }
    }

    fn eval_unary(&self, op: UnaryOp, val: Value) -> EvalResult<Value> {
        match op {
            UnaryOp::Plus => match val {
                Value::Number(n) => Ok(Value::Number(n)),
                _ => Err(RuntimeError::type_error(
                    "number",
                    val.type_name(),
                    "unary +",
                )),
            },
            UnaryOp::Minus => match val {
                Value::Number(n) => Ok(Value::Number(-n)),
                _ => Err(RuntimeError::type_error(
                    "number",
                    val.type_name(),
                    "unary -",
                )),
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
                    "addition",
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

            BinOp::Eq => Ok(Value::Bool(Self::values_equal(&left, &right))),
            BinOp::Ne => Ok(Value::Bool(!Self::values_equal(&left, &right))),

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
                &format!("operator '{}'", op_str),
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
                "comparison",
            )),
        }
    }

    fn values_equal(a: &Value, b: &Value) -> bool {
        match (a, b) {
            (Value::Number(x), Value::Number(y)) => x == y,
            (Value::String(x), Value::String(y)) => x == y,
            (Value::Bool(x), Value::Bool(y)) => x == y,
            (Value::Null, Value::Null) => true,
            (Value::Array(x), Value::Array(y)) => {
                x.len() == y.len()
                    && x.iter()
                        .zip(y.iter())
                        .all(|(a, b)| Self::values_equal(a, b))
            }
            _ => false,
        }
    }

    fn eval_import(&mut self, path: &str, is_aws_secret: bool) -> EvalResult<Value> {
        // Check if this is an AWS secret import
        if is_aws_secret {
            // For now, we'll just return an empty object with a note
            // In a real implementation, this would fetch from AWS Secrets Manager
            self.output.push(format!(
                "Note: import_aws_secret('{}') would fetch from AWS Secrets Manager",
                path
            ));
            return Ok(Value::Object(HashMap::new()));
        }

        // Resolve the file path relative to base_path
        let file_path = if Path::new(path).is_absolute() {
            PathBuf::from(path)
        } else {
            self.base_path.join(path)
        };

        // Canonicalize the path to detect circular imports reliably
        let canonical_path = file_path.canonicalize().map_err(|e| {
            RuntimeError::new(format!("Failed to resolve import file '{}': {}", path, e))
        })?;

        let canonical_str = canonical_path.to_string_lossy().to_string();

        // Check for circular imports
        if self.imported_files.contains_key(&canonical_str) {
            return Err(RuntimeError::new(format!(
                "Circular import detected: '{}'",
                path
            )));
        }

        // Mark as importing (to detect circular imports)
        self.imported_files.insert(canonical_str.clone(), ());

        // Read the file
        let source = fs::read_to_string(&canonical_path).map_err(|e| {
            RuntimeError::new(format!("Failed to read import file '{}': {}", path, e))
        })?;

        // Lex and parse the imported file
        let tokens = crate::lexing::analyze_code(&source);
        let program = crate::grama::build_statements(&tokens).map_err(|e| {
            RuntimeError::new(format!("Failed to parse import file '{}': {}", path, e))
        })?;

        // Save the current base path
        let original_base = self.base_path.clone();

        // Update base path to the imported file's directory
        if let Some(parent) = canonical_path.parent() {
            self.base_path = parent.to_path_buf();
        }

        // Create a new evaluator for the imported file to isolate its execution
        let mut import_evaluator = Evaluator {
            env: Environment::new(),
            output: Vec::new(),
            public_vars: HashMap::new(),
            env_output_lines: Vec::new(),
            base_path: self.base_path.clone(),
            imported_files: self.imported_files.clone(),
        };

        // Execute the imported program
        for stmt in &program.items {
            import_evaluator.eval_statement(stmt)?;
        }

        // Merge output from imported file
        self.output.extend(import_evaluator.output);

        // Restore original base path
        self.base_path = original_base;

        // Return an object containing all public variables from the imported file
        Ok(Value::Object(import_evaluator.public_vars))
    }
}
