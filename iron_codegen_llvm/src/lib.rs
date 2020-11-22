extern crate inkwell;

use iron_ast::IronASTNode;
use iron_codegen::{IronCodegenError, IronCompilerBackend};

/// LLVM-based Iron compiler backend.
pub struct IronBackendLLVM;

impl IronCompilerBackend for IronBackendLLVM {
    fn compile_module(node: IronASTNode) -> Result<(), IronCodegenError> {
        Err(IronCodegenError::Unexpected)
    }
}
