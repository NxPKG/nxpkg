pub mod module;
pub mod output_asset;
pub mod source;
pub mod utils;

use indexmap::IndexSet;
use nxpkg_tasks::Vc;

type VcDynIntrospectable = Vc<Box<dyn Introspectable>>;

#[nxpkg_tasks::value(transparent)]
pub struct IntrospectableChildren(IndexSet<(Vc<String>, VcDynIntrospectable)>);

#[nxpkg_tasks::value_trait]
pub trait Introspectable {
    fn ty(self: Vc<Self>) -> Vc<String>;
    fn title(self: Vc<Self>) -> Vc<String> {
        Vc::<String>::default()
    }
    fn details(self: Vc<Self>) -> Vc<String> {
        Vc::<String>::default()
    }
    fn children(self: Vc<Self>) -> Vc<IntrospectableChildren> {
        Vc::cell(IndexSet::new())
    }
}
