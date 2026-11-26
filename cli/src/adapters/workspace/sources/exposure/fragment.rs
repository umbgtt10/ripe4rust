// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use syn::Attribute;
use syn::Generics;
use syn::ItemExternCrate;
use syn::ItemUse;
use syn::Path;
use syn::Signature;
use syn::Type;
use syn::TypeParamBound;

/// A part of an item that a crate's users see: a type, generics and their
/// bounds, a signature, a re-export, a derive, a trait's path.
#[derive(Clone, Copy)]
pub enum Fragment<'a> {
    Attribute(&'a Attribute),
    Bound(&'a TypeParamBound),
    ExternCrate(&'a ItemExternCrate),
    Generics(&'a Generics),
    Path(&'a Path),
    Signature(&'a Signature),
    Type(&'a Type),
    Use(&'a ItemUse),
}
