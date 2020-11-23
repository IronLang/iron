mod error;

pub use error::IronCodegenError;
use iron_ast::IronASTNode;

/// Trait required for Iron compiler backends.
///
///
pub trait IronCompilerBackend {
    fn compile_module(&mut self, ast: IronASTNode) -> Result<(), IronCodegenError>;
}
