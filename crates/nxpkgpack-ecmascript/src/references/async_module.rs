use anyhow::Result;
use indexmap::IndexSet;
use serde::{Deserialize, Serialize};
use swc_core::{
    common::DUMMY_SP,
    ecma::ast::{ArrayLit, ArrayPat, Expr, Ident, Pat, Program},
    quote,
};
use nxpkg_tasks::{trace::TraceRawVcs, TryFlatJoinIterExt, Vc};
use nxpkgpack_core::chunk::{AsyncModuleInfo, ChunkableModule};

use super::esm::base::ReferencedAsset;
use crate::{
    chunk::{EcmascriptChunkPlaceable, EcmascriptChunkingContext},
    code_gen::{CodeGenerateableWithAsyncModuleInfo, CodeGeneration},
    create_visitor,
    references::esm::{base::insert_hoisted_stmt, EsmAssetReference},
};

/// Information needed for generating the async module wrapper for
/// [EcmascriptChunkItem](crate::chunk::EcmascriptChunkItem)s.
#[derive(PartialEq, Eq, Default, Debug, Clone, Serialize, Deserialize, TraceRawVcs)]
pub struct AsyncModuleOptions {
    pub has_top_level_await: bool,
}

/// Option<[AsyncModuleOptions]>.
#[nxpkg_tasks::value(transparent)]
pub struct OptionAsyncModuleOptions(Option<AsyncModuleOptions>);

#[nxpkg_tasks::value_impl]
impl OptionAsyncModuleOptions {
    #[nxpkg_tasks::function]
    pub(crate) fn none() -> Vc<Self> {
        Vc::cell(None)
    }
}

/// Contains the information necessary to decide if an ecmascript module is
/// async.
///
/// It will check if the current module or any of it's children contain a top
/// level await statement or is referencing an external ESM module.
#[nxpkg_tasks::value(shared)]
pub struct AsyncModule {
    pub placeable: Vc<Box<dyn EcmascriptChunkPlaceable>>,
    pub references: IndexSet<Vc<EsmAssetReference>>,
    pub has_top_level_await: bool,
}

/// Option<[AsyncModule]>.
#[nxpkg_tasks::value(transparent)]
pub struct OptionAsyncModule(Option<Vc<AsyncModule>>);

#[nxpkg_tasks::value_impl]
impl OptionAsyncModule {
    /// Create an empty [OptionAsyncModule].
    #[nxpkg_tasks::function]
    pub fn none() -> Vc<Self> {
        Vc::cell(None)
    }

    #[nxpkg_tasks::function]
    pub async fn module_options(
        self: Vc<Self>,
        async_module_info: Option<Vc<AsyncModuleInfo>>,
    ) -> Result<Vc<OptionAsyncModuleOptions>> {
        if let Some(async_module) = &*self.await? {
            return Ok(async_module.module_options(async_module_info));
        }

        Ok(OptionAsyncModuleOptions::none())
    }
}

#[nxpkg_tasks::value(transparent)]
struct AsyncModuleIdents(IndexSet<String>);

#[nxpkg_tasks::value_impl]
impl AsyncModule {
    #[nxpkg_tasks::function]
    async fn get_async_idents(
        self: Vc<Self>,
        chunking_context: Vc<Box<dyn EcmascriptChunkingContext>>,
        async_module_info: Vc<AsyncModuleInfo>,
    ) -> Result<Vc<AsyncModuleIdents>> {
        let this = self.await?;
        let async_module_info = async_module_info.await?;

        let reference_idents = this
            .references
            .iter()
            .map(|r| async {
                let referenced_asset = r.get_referenced_asset().await?;
                Ok(match &*referenced_asset {
                    ReferencedAsset::OriginalReferenceTypeExternal(_) => {
                        // TODO(WEB-1259): we need to detect if external modules are esm
                        None
                    }
                    ReferencedAsset::Some(placeable) => {
                        let chunk_item = placeable
                            .as_chunk_item(Vc::upcast(chunking_context))
                            .resolve()
                            .await?;
                        if async_module_info
                            .referenced_async_modules
                            .contains(&chunk_item)
                        {
                            referenced_asset.get_ident().await?
                        } else {
                            None
                        }
                    }
                    ReferencedAsset::None => None,
                })
            })
            .try_flat_join()
            .await?;

        Ok(Vc::cell(IndexSet::from_iter(reference_idents)))
    }

    #[nxpkg_tasks::function]
    pub(crate) fn is_self_async(&self) -> Vc<bool> {
        Vc::cell(self.has_top_level_await)
    }

    /// Returns
    #[nxpkg_tasks::function]
    pub async fn module_options(
        self: Vc<Self>,
        async_module_info: Option<Vc<AsyncModuleInfo>>,
    ) -> Result<Vc<OptionAsyncModuleOptions>> {
        if async_module_info.is_none() {
            return Ok(Vc::cell(None));
        }

        Ok(Vc::cell(Some(AsyncModuleOptions {
            has_top_level_await: self.await?.has_top_level_await,
        })))
    }
}

#[nxpkg_tasks::value_impl]
impl CodeGenerateableWithAsyncModuleInfo for AsyncModule {
    #[nxpkg_tasks::function]
    async fn code_generation(
        self: Vc<Self>,
        chunking_context: Vc<Box<dyn EcmascriptChunkingContext>>,
        async_module_info: Option<Vc<AsyncModuleInfo>>,
    ) -> Result<Vc<CodeGeneration>> {
        let mut visitors = Vec::new();

        if let Some(async_module_info) = async_module_info {
            let async_idents = self
                .get_async_idents(chunking_context, async_module_info)
                .await?;

            if !async_idents.is_empty() {
                visitors.push(create_visitor!(visit_mut_program(program: &mut Program) {
                    add_async_dependency_handler(program, &async_idents);
                }));
            }
        }

        Ok(CodeGeneration { visitors }.into())
    }
}

fn add_async_dependency_handler(program: &mut Program, idents: &IndexSet<String>) {
    let idents = idents
        .iter()
        .map(|ident| Ident::new(ident.clone().into(), DUMMY_SP))
        .collect::<Vec<_>>();

    let stmt = quote!(
        "var __nxpkgpack_async_dependencies__ = __nxpkgpack_handle_async_dependencies__($deps);"
            as Stmt,
        deps: Expr = Expr::Array(ArrayLit {
            span: DUMMY_SP,
            elems: idents
                .iter()
                .map(|ident| { Some(Expr::Ident(ident.clone()).into()) })
                .collect(),
        }),
    );

    insert_hoisted_stmt(program, stmt);

    let stmt = quote!(
        "($deps = __nxpkgpack_async_dependencies__.then ? (await \
         __nxpkgpack_async_dependencies__)() : __nxpkgpack_async_dependencies__);" as Stmt,
        deps: Pat = Pat::Array(ArrayPat {
            span: DUMMY_SP,
            elems: idents
                .into_iter()
                .map(|ident| { Some(ident.into()) })
                .collect(),
            optional: false,
            type_ann: None,
        }),
    );

    insert_hoisted_stmt(program, stmt);
}
