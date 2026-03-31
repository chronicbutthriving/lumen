//! A registry for UUID kinds used in Lumen and related projects.
//!
//! See this crate's `README.adoc` for more information.

#[doc(no_inline)]
pub use newtype_uuid::{GenericUuid, ParseError, TagError, TypedUuid, TypedUuidKind, TypedUuidTag};

use newtype_uuid_macros::impl_typed_uuid_kinds;

impl_typed_uuid_kinds! {
    settings = {
        schemars08 = {
            attrs = [#[cfg(feature = "schemars08")]],
            rust_type = {
                crate = "lumen-uuid-kinds",
                version = "*",
                path = "lumen_uuid_kinds",
            },
        },
    },

    kinds = {
        // Auth Service
        AccessToken = {},
        User = {},
        UserProvider = {},

        // Storage Service
        Object = {},

        // Resource Hub Service
        Resource = {},
        ResourceItem = {},
    }
}
