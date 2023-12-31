use anyhow::Result;
use swc_core::quote;
use nxpkg_tasks::Vc;

use super::AstPath;
use crate::{
    chunk::EcmascriptChunkingContext,
    code_gen::{CodeGenerateable, CodeGeneration},
    create_visitor,
};

#[nxpkg_tasks::value]
pub struct Unreachable {
    path: Vc<AstPath>,
}

#[nxpkg_tasks::value_impl]
impl Unreachable {
    #[nxpkg_tasks::function]
    pub fn new(path: Vc<AstPath>) -> Vc<Self> {
        Self::cell(Unreachable { path })
    }
}

#[nxpkg_tasks::value_impl]
impl CodeGenerateable for Unreachable {
    #[nxpkg_tasks::function]
    async fn code_generation(
        &self,
        _context: Vc<Box<dyn EcmascriptChunkingContext>>,
    ) -> Result<Vc<CodeGeneration>> {
        let path = self.path.await?;
        let visitors = [
            // Unreachable might be used on Stmt or Expr
            create_visitor!(exact path, visit_mut_expr(expr: &mut Expr) {
                *expr = quote!("(\"NXPKGPACK unreachable\", undefined)" as Expr);
            }),
            create_visitor!(exact path, visit_mut_stmt(stmt: &mut Stmt) {
                // TODO(WEB-553) walk ast to find all `var` declarations and keep them
                // since they hoist out of the scope
                *stmt = quote!("{\"NXPKGPACK unreachable\";}" as Stmt);
            }),
        ]
        .into();

        Ok(CodeGeneration { visitors }.cell())
    }
}
