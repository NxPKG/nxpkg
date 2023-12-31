use nxpkg_tasks::Vc;

use crate::{asset::Asset, ident::AssetIdent};

/// (Unparsed) Source Code. Source Code is processed into [Module]s by the
/// [AssetContext]. All [Source]s have content and an identifier.
#[nxpkg_tasks::value_trait]
pub trait Source: Asset {
    /// The identifier of the [Source]. It's expected to be unique and capture
    /// all properties of the [Source].
    fn ident(&self) -> Vc<AssetIdent>;
}

#[nxpkg_tasks::value(transparent)]
pub struct OptionSource(Option<Vc<Box<dyn Source>>>);

#[nxpkg_tasks::value(transparent)]
pub struct Sources(Vec<Vc<Box<dyn Source>>>);

// TODO All Vc::try_resolve_downcast::<Box<dyn Source>> calls should be removed
