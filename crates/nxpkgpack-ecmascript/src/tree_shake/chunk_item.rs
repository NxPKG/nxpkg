use anyhow::Result;
use nxpkg_tasks::Vc;
use nxpkgpack_core::{
    chunk::{AsyncModuleInfo, ChunkItem, ChunkType, ChunkingContext},
    ident::AssetIdent,
    module::Module,
    reference::ModuleReferences,
};

use super::{asset::EcmascriptModulePartAsset, part_of_module, split_module};
use crate::{
    chunk::{
        placeable::EcmascriptChunkPlaceable, EcmascriptChunkItem, EcmascriptChunkItemContent,
        EcmascriptChunkType, EcmascriptChunkingContext,
    },
    EcmascriptModuleContent,
};

/// This is an implementation of [ChunkItem] for
/// [Vc<EcmascriptModulePartAsset>].
///
/// This is a pointer to a part of an ES module.
#[nxpkg_tasks::value(shared)]
pub struct EcmascriptModulePartChunkItem {
    pub(super) module: Vc<EcmascriptModulePartAsset>,
    pub(super) chunking_context: Vc<Box<dyn EcmascriptChunkingContext>>,
}

#[nxpkg_tasks::value_impl]
impl EcmascriptChunkItem for EcmascriptModulePartChunkItem {
    #[nxpkg_tasks::function]
    fn content(self: Vc<Self>) -> Vc<EcmascriptChunkItemContent> {
        panic!("content() should never be called");
    }

    #[nxpkg_tasks::function]
    async fn content_with_async_module_info(
        self: Vc<Self>,
        async_module_info: Option<Vc<AsyncModuleInfo>>,
    ) -> Result<Vc<EcmascriptChunkItemContent>> {
        let this = self.await?;
        let module = this.module.await?;
        let async_module_options = module
            .full_module
            .get_async_module()
            .module_options(async_module_info);

        let split_data = split_module(module.full_module);
        let parsed = part_of_module(split_data, module.part);

        let content = EcmascriptModuleContent::new(
            parsed,
            module.full_module.ident(),
            this.chunking_context,
            this.module.analyze(),
            async_module_info,
        );

        Ok(EcmascriptChunkItemContent::new(
            content,
            this.chunking_context,
            async_module_options,
        ))
    }

    #[nxpkg_tasks::function]
    fn chunking_context(&self) -> Vc<Box<dyn EcmascriptChunkingContext>> {
        self.chunking_context
    }
}

#[nxpkg_tasks::value_impl]
impl ChunkItem for EcmascriptModulePartChunkItem {
    #[nxpkg_tasks::function]
    async fn references(&self) -> Vc<ModuleReferences> {
        self.module.references()
    }

    #[nxpkg_tasks::function]
    async fn asset_ident(&self) -> Result<Vc<AssetIdent>> {
        Ok(self.module.ident())
    }

    #[nxpkg_tasks::function]
    async fn chunking_context(&self) -> Vc<Box<dyn ChunkingContext>> {
        Vc::upcast(self.chunking_context)
    }

    #[nxpkg_tasks::function]
    async fn ty(&self) -> Result<Vc<Box<dyn ChunkType>>> {
        Ok(Vc::upcast(
            Vc::<EcmascriptChunkType>::default().resolve().await?,
        ))
    }

    #[nxpkg_tasks::function]
    fn module(&self) -> Vc<Box<dyn Module>> {
        Vc::upcast(self.module)
    }
}
