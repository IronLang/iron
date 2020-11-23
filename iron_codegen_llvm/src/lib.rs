extern crate inkwell;

use inkwell::{
    builder::Builder,
    context::Context,
    module::Module,
    passes::PassManager,
    types::BasicTypeEnum,
    values::{BasicValue, BasicValueEnum, FloatValue, FunctionValue, PointerValue},
    FloatPredicate, OptimizationLevel,
};
use iron_ast::IronASTNode;
use iron_codegen::{IronCodegenError, IronCompilerBackend};

trait IronMachineCodeGeneratable {
    fn generate_code(&self) -> Result<(), IronCodegenError>;
}

pub struct IronCompilerBackendLLVM {
    context: Context,
}

impl IronCompilerBackendLLVM {
    pub fn new() -> IronCompilerBackendLLVM {
        IronCompilerBackendLLVM {
            context: Context::create(),
        }
    }
}

impl IronCompilerBackend for IronCompilerBackendLLVM {
    fn compile_module(&mut self, ast: IronASTNode) -> Result<(), IronCodegenError> {
        match ast {
            IronASTNode::Module {
                identifier,
                functions,
            } => {
                let mut module = self.context.create_module(identifier.as_str());
                let _results = functions
                    .values()
                    .map(|node| compile_function(&mut module, node));
                Ok(())
            }
            other => Err(IronCodegenError::UnexpectedASTNode(other)),
        }
    }
}

fn compile_function(module: &mut Module, node: &IronASTNode) -> Result<(), IronCodegenError> {
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {}
}
