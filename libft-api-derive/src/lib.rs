//! Procedural macros for the `libft-api` crate.
//!
//! This crate provides procedural macros that are used to reduce boilerplate code
//! in the main `libft-api` crate. The macros are implemented as derive macros
//! that automatically generate trait implementations for data structures.
//!
//! # Available Macros
//!
//! * `HasVector` - Derives the `HasVec` trait for structs that contain exactly one `Vec<T>` field
//!
//! # Example
//!
//! ```rust
//! use libft_api_derive::HasVector;
//! use libft_api::api::HasVec;
//!
//! #[derive(HasVector)]
//! struct FtApiUsersResponse {
//!     users: Vec<String>,
//! }
//!
//! // This generates an implementation of the HasVec trait automatically:
//! // impl HasVec<String> for FtApiUsersResponse {
//! //     fn get_vec(&self) -> &Vec<String> { &self.users }
//! //     fn take_vec(self) -> Vec<String> { self.users }
//! // }
//! ```

extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse_macro_input, spanned::Spanned, Data, DeriveInput, Error, Fields, GenericArgument,
    PathArguments, Type,
};

/// Derives the `HasVec` trait for structs that contain exactly one `Vec<T>` field.
///
/// This macro automatically implements the `HasVec` trait for structs that have
/// exactly one named field of type `Vec<T>`. The generated implementation provides
/// methods to access and take ownership of the vector field.
///
/// # Requirements
/// * The struct must have exactly one field of type `Vec<T>`
/// * The struct must have named fields (not tuple or unit structs)
/// * The field type must be exactly `Vec<T>`, not an alias or reference
///
/// # Example
///
/// ```rust
/// use libft_api_derive::HasVector;
/// use libft_api::api::HasVec;
///
/// #[derive(HasVector)]
/// struct FtApiUsersResponse {
///     users: Vec<String>,
/// }
///
/// // This generates:
/// // impl HasVec<String> for FtApiUsersResponse {
/// //     fn get_vec(&self) -> &Vec<String> { &self.users }
/// //     fn take_vec(self) -> Vec<String> { self.users }
/// // }
/// ```
#[proc_macro_derive(HasVector)]
pub fn has_vec_derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    expand_has_vec(ast)
        .unwrap_or_else(|e| e.to_compile_error())
        .into()
}

fn expand_has_vec(ast: DeriveInput) -> Result<proc_macro2::TokenStream, syn::Error> {
    let struct_name = &ast.ident;

    let field = match &ast.data {
        Data::Struct(s) => match &s.fields {
            Fields::Named(named) => {
                named.named.iter().find(|f| is_vec(&f.ty)).ok_or_else(|| {
                    Error::new(
                        s.fields.span(),
                        "HasVector requires exactly one named field of type Vec<T>",
                    )
                })?
            }
            _ => {
                return Err(Error::new(
                    s.fields.span(),
                    "HasVector currently supports only named-field structs",
                ))
            }
        },
        _ => {
            return Err(Error::new(
                ast.span(),
                "HasVector can only be derived for structs",
            ))
        }
    };

    let field_ident = field
        .ident
        .clone()
        .ok_or_else(|| Error::new(field.span(), "expected a named field"))?;

    let inner_ty = extract_vec_inner_ty(&field.ty).ok_or_else(|| {
        Error::new(
            field.ty.span(),
            "field must be exactly Vec<T> (no aliases or refs)",
        )
    })?;

    Ok(quote! {
        impl HasVec<#inner_ty> for #struct_name {
            fn get_vec(&self) -> &Vec<#inner_ty> { &self.#field_ident }
            fn take_vec(self) -> Vec<#inner_ty> { self.#field_ident }
        }
    })
}

fn is_vec(ty: &Type) -> bool {
    matches!(
        ty,
        Type::Path(tp)
            if tp.path.segments.last().map(|s| s.ident == "Vec").unwrap_or(false)
    )
}

fn extract_vec_inner_ty(ty: &Type) -> Option<Type> {
    if let Type::Path(tp) = ty {
        if let Some(seg) = tp.path.segments.last() {
            if seg.ident == "Vec" {
                if let PathArguments::AngleBracketed(args) = &seg.arguments {
                    if let Some(GenericArgument::Type(inner)) = args.args.first() {
                        return Some(inner.clone());
                    }
                }
            }
        }
    }
    None
}
