use nxpkg_tasks::Vc;
use nxpkgpack_core::{chunk::AsyncModuleInfo, output::OutputAsset};

use super::item::EcmascriptChunkItem;

type EcmascriptChunkItemWithAsyncInfo = (
    Vc<Box<dyn EcmascriptChunkItem>>,
    Option<Vc<AsyncModuleInfo>>,
);

#[nxpkg_tasks::value(shared)]
pub struct EcmascriptChunkContent {
    pub chunk_items: Vec<EcmascriptChunkItemWithAsyncInfo>,
    pub referenced_output_assets: Vec<Vc<Box<dyn OutputAsset>>>,
}
