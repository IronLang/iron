extern crate clap;
extern crate iron_parser;

use clap::{App, Arg};
use iron_ast::IronASTNode;
use iron_codegen::{IronCodegenError, IronCompilerBackend};
use iron_codegen_llvm::IronCompilerBackendLLVM;
use iron_parser::Module;
use std::collections::HashMap;
use std::str::FromStr;

fn main() {
    // let matches = App::new("ironc")
    //     .version("0.1")
    //     .about("Compiles an Iron module.")
    //     .arg(
    //         Arg::with_name("INPUT")
    //             .help("Path to file containing Iron source code")
    //             .required(true),
    //     )
    //     .get_matches();

    // let path = matches.value_of("INPUT").expect("expected INPUT");
    // let _module = Module::from_str(path).expect("module should be parsed successfully");

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
    let module: IronASTNode = IronASTNode::Module {
        identifier: "hello".to_string(),
        functions,
    };

    // Compile the module.
    let mut llvm = IronCompilerBackendLLVM::new();
    let result = llvm.compile_module(module);
    match result {
        Ok(_) => println!("Compiled successfully!"),
        Err(err) => println!("There was an error compiling this module."),
    }
}
