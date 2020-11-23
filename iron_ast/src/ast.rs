use std::collections::HashMap;

#[derive(Debug)]
pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
    Pow,
}

/// Represents a node in an Iron abstract syntax tree (AST).
#[derive(Debug)]
pub enum IronASTNode {
    Identifier(String),

    /// Represents an 8-bit signed integer.
    LiteralI8(i8),
    /// Represents an 16-bit signed integer.
    LiteralI16(i16),
    /// Represents an 32-bit signed integer.
    LiteralI32(i32),
    /// Represents an 64-bit signed integer.
    LiteralI64(i64),
    /// Represents an 128-bit signed integer.
    LiteralI128(i128),

    /// Represents an 8-bit unsigned integer.
    LiteralU8(u8),
    /// Represents an 16-bit unsigned integer.
    LiteralU16(u16),
    /// Represents an 32-bit unsigned integer.
    LiteralU32(u32),
    /// Represents an 64-bit unsigned integer.
    LiteralU64(u64),
    /// Represents an 128-bit unsigned integer.
    LiteralU128(u128),

    /// Represents a boolean literal.
    LiteralBoolean(bool),

    /// Represents a Character literal.
    LiteralCharacter(char),

    /// Represents a String literal.
    LiteralString(String),

    /// Represents a numeric literal.
    ///
    /// Currently, the inferred numeric type depends on the host architecture. This is currently
    /// represented with Rust's `isize` type.
    ///
    /// This is used when the integer type is not explicitly defined.
    LiteralInteger(isize),

    /// Represents an 32-bit floating point value.
    LiteralF32(f32),
    /// Represents an 64-bit floating point value.
    LiteralF64(f64),

    /// Represents a unary expression.
    ///
    /// A unary expression can either be prefix:
    ///
    /// `<op> <expr>`
    ///
    /// or post-fix:
    ///
    /// `<expr> <op>`
    ///
    ExpressionUnary {
        op: Operator,
        expr: Box<IronASTNode>,
    },

    /// Represents a binary expression.
    ///
    /// `<lhs> <op> <rhs>`
    ///
    ExpressionBinary {
        op: Operator,
        lhs: Box<IronASTNode>,
        rhs: Box<IronASTNode>,
    },

    /// Represents a ternary expression.
    ///
    /// `<primary_expr> <primary_op> <secondary_expr> <secondary_op> <ternary_expr>`
    ///
    /// At the moment, the only ternary expression is the boolean ternary expression that is
    /// common in other languages. It uses the `?` and `:` operators:
    ///
    /// `let result = some_bool ? option_one : option_2`
    ///
    ExpressionTernary {
        primary_op: Operator,
        secondary_op: Operator,
        primary_expr: Box<IronASTNode>,
        secondary_expr: Box<IronASTNode>,
        ternary_expr: Box<IronASTNode>,
    },

    /// Represents a variable definition.
    ///
    /// In Iron, a variable must be declared and initialized at the same time. At this time,
    /// it should also be specified whether the variable should be mutable or not. By default,
    /// variables are _immutable_, meaning they cannot be changed later.
    ///
    /// `let <mutable ? 'mutable' : ''> <identifier> = <value>`
    ///
    VariableDefinition {
        identifier: String,
        mutable: bool,
        value: Box<IronASTNode>,
    },

    /// Represents a function parameter.
    ///
    /// Function parameters are used by function definitions to indicate the types of data
    /// that must be passed into the function when called.
    FunctionParameter {
        label: Option<String>,
        name: String,
        kind: String,
    },

    /// Represents a function definition.
    ///
    /// The `parameters` vector must contain instances of `IronASTNode::FunctionParameter`.
    FunctionDefinition {
        public: bool,
        identifier: String,
        parameters: Vec<IronASTNode>,
        statements: Vec<IronASTNode>,
    },

    /// Represents a function parameter.
    ///
    /// In the Iron programming languages, function parameters might have labels depending on the
    /// signature of the called function.
    ///
    /// `<label>: <value>`
    ///
    FunctionCallArgument {
        label: Option<String>,
        value: Box<IronASTNode>,
    },

    /// Represents a function call.
    ///
    /// The `arguments` vector must contain instances of `IronASTNode::FunctionCallArgument`.
    ///
    /// `<identifier>(<parameters...>)`
    ///
    FunctionCall {
        identifier: String,
        arguments: Vec<IronASTNode>,
    },

    /// Represents a type definition.
    ///
    /// The `defn` property must be one of:
    ///
    /// - `IronASTNode::TypeDefinitionAlias`
    /// - `IronASTNode::TypeDefinitionEnum`
    /// - `IronASTNode::TypeDefinitionStructure`
    TypeDefinition {
        public: bool,
        identifier: String,
        defn: Box<IronASTNode>,
    },

    /// Represents an alias type.
    TypeDefinitionAlias {
        alias: String,
    },

    /// Represents an enum type.
    TypeDefinitionEnum {
        cases: Vec<IronASTNode>,
    },

    /// Represents a variant of an enum type.
    TypeDefinitionEnumCase {
        identifier: String,
        associated_value: Option<Box<IronASTNode>>,
    },

    /// Represents a structure
    TypeDefinitionStructure {
        properties: HashMap<String, String>,
    },

    /// Represents an import statement.
    ImportStatement {
        identifiers: Vec<String>,
    },

    /// Represents a return statement.
    ///
    ///
    /// `return <expr>`
    ///
    ReturnStatement {
        expr: Box<IronASTNode>,
    },

    /// Represents an Iron module.
    Module {
        /// Identifier for the module.
        identifier: String,
        /// Module-level function definitions.
        functions: HashMap<String, IronASTNode>,
    },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_module() {
        let hello: IronASTNode = IronASTNode::FunctionDefinition {
            public: true,
            identifier: "hello".to_string(),
            parameters: vec![IronASTNode::FunctionParameter {
                label: None,
                name: "name".to_string(),
                kind: "String".to_string(),
            }],
            statements: vec![
                IronASTNode::VariableDefinition {
                    identifier: "result".to_string(),
                    mutable: false,
                    value: Box::new(IronASTNode::LiteralString("Hello, {name}!".to_string())),
                },
                IronASTNode::ReturnStatement {
                    expr: Box::new(IronASTNode::Identifier("result".to_string())),
                },
            ],
        };

        // Create the function hashmap.
        let mut functions: HashMap<String, IronASTNode> = HashMap::new();
        functions.insert("hello".to_string(), hello);

        // Create the module.
        let _module: IronASTNode = IronASTNode::Module {
            identifier: "hello".to_string(),
            functions,
        };
    }
}
