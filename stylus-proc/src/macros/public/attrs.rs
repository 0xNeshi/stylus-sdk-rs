// Copyright 2022-2024, Offchain Labs, Inc.
// For licensing, see https://github.com/OffchainLabs/stylus-sdk-rs/blob/main/licenses/COPYRIGHT.md

use syn::{
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    Token,
};

/// Implement parent trait routes.
///
/// Used for the `#[implements(Parent1, Parent2)` attribute.
///
/// The contract must implement whatever traits are specified.
pub struct Implements {
    pub types: Punctuated<syn::Type, Token![,]>,
}

impl Parse for Implements {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            types: Punctuated::parse_terminated(input)?,
        })
    }
}

/// Selector name overloading for public functions.
///
/// Used for the `#[selector(name = "...")]` attribute.
#[derive(Debug)]
pub struct Selector {
    _name: kw::name,
    _eq_token: Token![=],
    pub value: syn::LitStr,
}

impl Parse for Selector {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            _name: input.parse()?,
            _eq_token: input.parse()?,
            value: input.parse()?,
        })
    }
}

/// Associated type mapping for supertraits.
///
/// Used for the `#[supertrait_associated_types(AssocType1 = Type1, AssocType2 = Type2)]` attribute.
pub struct SupertraitAssociatedTypes {
    pub types: Punctuated<AssociatedTypeMapping, Token![,]>,
}

impl Parse for SupertraitAssociatedTypes {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            types: Punctuated::parse_terminated(input)?,
        })
    }
}

/// A single associated type mapping (ident = type).
pub struct AssociatedTypeMapping {
    pub ident: syn::Ident,
    pub ty: syn::Type,
}

impl Parse for AssociatedTypeMapping {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let ident = input.parse()?;
        let _eq_token: Token![=] = input.parse()?;
        let ty = input.parse()?;
        Ok(Self { ident, ty })
    }
}

mod kw {
    syn::custom_keyword!(name);
}
