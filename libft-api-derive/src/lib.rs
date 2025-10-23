extern crate proc_macro;

use proc_macro::TokenStream;
use quote::{format_ident, quote, ToTokens};
use syn::{
    parse_macro_input, spanned::Spanned, AngleBracketedGenericArguments, Attribute, Data,
    DataStruct, DeriveInput, Fields, GenericArgument, Ident, Path, PathArguments,
    Result as SynResult,
};

/// #[derive(HasItems)]
/// #[has_items(
///     crate = "libft_api",        // optional: prefix for HasItems/Values/Entries (e.g., "libft_api")
///     field = "users",            // optional when only one supported field exists
///     modes = "both"              // "values" | "entries" | "both" ; default:
///                                 //   Vec<T>  -> "values"
///                                 //   HashMap -> "both"
/// )]
#[proc_macro_derive(HasItems, attributes(has_items))]
pub fn derive_has_items(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    expand_has_items(ast)
        .unwrap_or_else(|e| e.to_compile_error())
        .into()
}

#[derive(Default)]
struct DeriveOpts {
    /// crate path prefix for HasItems, Values, Entries. e.g., libft_api
    crate_path: Option<Path>,
    /// name of the field to use (if multiple candidates exist)
    field_name: Option<Ident>,
    /// modes: values | entries | both
    modes: Option<Modes>,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Modes {
    Values,
    Entries,
    Both,
}

impl Modes {
    fn parse(s: &str) -> Option<Self> {
        match s.to_ascii_lowercase().as_str() {
            "values" => Some(Modes::Values),
            "entries" => Some(Modes::Entries),
            "both" => Some(Modes::Both),
            _ => None,
        }
    }
}

fn expand_has_items(input: DeriveInput) -> SynResult<proc_macro2::TokenStream> {
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let struct_ident = &input.ident;

    let opts = parse_attrs(&input.attrs)?;

    let ds = match input.data {
        Data::Struct(ds) => ds,
        _ => {
            return Err(syn::Error::new(
                input.span(),
                "HasItems derive only supports structs",
            ))
        }
    };

    let (field_ident, kind) = pick_field(&ds, opts.field_name.as_ref())?;

    // Default modes by kind if not specified
    let modes = opts.modes.unwrap_or(match kind {
        FieldKind::Vec { .. } => Modes::Values,
        FieldKind::HashMap { .. } => Modes::Both,
    });

    let crate_prefix = opts
        .crate_path
        .map(|p| p.to_token_stream())
        .unwrap_or_else(|| quote!());

    // Paths for traits/markers (with optional crate prefix)
    let path_has_items = if crate_prefix.is_empty() {
        quote!(HasItems)
    } else {
        quote!(#crate_prefix::HasItems)
    };
    let path_values = if crate_prefix.is_empty() {
        quote!(Values)
    } else {
        quote!(#crate_prefix::Values)
    };
    let path_entries = if crate_prefix.is_empty() {
        quote!(Entries)
    } else {
        quote!(#crate_prefix::Entries)
    };

    // Generate impl(s)
    let mut impls = Vec::new();

    let context = HasItemContext::new(
        struct_ident,
        impl_generics,
        ty_generics,
        &field_ident,
        &path_has_items,
        &path_values,
        &path_entries,
    )
    .with_where_clause(where_clause);
    match kind {
        FieldKind::Vec { t } => {
            // Vec<T> supports only Values
            if matches!(modes, Modes::Entries) {
                return Err(syn::Error::new(
                    field_ident.span(),
                    "modes = \"entries\" is not supported for Vec<T>. Use modes = \"values\".",
                ));
            }
            impls.push(gen_impl_vec_values(&context, &t));
        }
        FieldKind::HashMap { k, v } => {
            match modes {
                Modes::Values | Modes::Both => impls.push(gen_impl_map_values(&context, &k, &v)),
                _ => {}
            }
            match modes {
                Modes::Entries | Modes::Both => impls.push(gen_impl_map_entries(&context, &k, &v)),
                _ => {}
            }
        }
    }

    Ok(quote! { #(#impls)* })
}

fn parse_attrs(attrs: &[Attribute]) -> SynResult<DeriveOpts> {
    let mut opts = DeriveOpts::default();

    for attr in attrs {
        if !attr.path().is_ident("has_items") {
            continue;
        }
        attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("crate") {
                let value = meta.value()?;
                let s: syn::LitStr = value.parse()?;
                opts.crate_path = Some(syn::parse_str(&s.value())?);
            }
            if meta.path.is_ident("field") {
                let value = meta.value()?;
                let s: syn::LitStr = value.parse()?;
                opts.field_name = Some(format_ident!("{}", s.value()));
            }
            if meta.path.is_ident("modes") {
                let value = meta.value()?;
                let s: syn::LitStr = value.parse()?;
                opts.modes = Modes::parse(&s.value());
            }
            Ok(())
        })?;
    }

    Ok(opts)
}

enum FieldKind {
    Vec {
        t: Box<syn::Type>,
    },
    HashMap {
        k: Box<syn::Type>,
        v: Box<syn::Type>,
    },
}

fn pick_field(ds: &DataStruct, wanted: Option<&Ident>) -> SynResult<(Ident, FieldKind)> {
    let named = match &ds.fields {
        Fields::Named(n) => &n.named,
        Fields::Unnamed(_) | Fields::Unit => {
            return Err(syn::Error::new(
                ds.struct_token.span,
                "tuple/unit structs are not supported; use a named-field struct",
            ))
        }
    };

    let mut candidates: Vec<(Ident, FieldKind)> = Vec::new();

    for f in named {
        let ident = f.ident.clone().unwrap();
        if let Some(w) = wanted {
            if &ident != w {
                continue;
            }
        }
        if let syn::Type::Path(tp) = &f.ty {
            if let Some(seg) = tp.path.segments.last() {
                if seg.ident == "Vec" {
                    if let PathArguments::AngleBracketed(AngleBracketedGenericArguments {
                        args,
                        ..
                    }) = &seg.arguments
                    {
                        if let Some(GenericArgument::Type(t)) = args.first() {
                            candidates.push((
                                ident,
                                FieldKind::Vec {
                                    t: Box::new(t.clone()),
                                },
                            ));
                            continue;
                        }
                    }
                } else if seg.ident == "HashMap" {
                    if let PathArguments::AngleBracketed(AngleBracketedGenericArguments {
                        args,
                        ..
                    }) = &seg.arguments
                    {
                        if args.len() == 2 {
                            if let (
                                Some(GenericArgument::Type(k)),
                                Some(GenericArgument::Type(v)),
                            ) = (args.first(), args.iter().nth(1))
                            {
                                candidates.push((
                                    ident,
                                    FieldKind::HashMap {
                                        k: Box::new(k.clone()),
                                        v: Box::new(v.clone()),
                                    },
                                ));
                                continue;
                            }
                        }
                    }
                }
            }
        }
    }

    if let Some(w) = wanted {
        candidates
            .into_iter()
            .next()
            .ok_or_else(|| syn::Error::new(w.span(), "specified field not found or unsupported"))
    } else {
        match candidates.len() {
            0 => Err(syn::Error::new(
                ds.struct_token.span,
                "no supported field found (need Vec<T> or HashMap<K, V>)",
            )),
            1 => Ok(candidates.into_iter().next().unwrap()),
            _ => Err(syn::Error::new(
                ds.struct_token.span,
                "multiple candidate fields. Use #[has_items(field = \"...\")]",
            )),
        }
    }
}

struct HasItemContext<'a> {
    struct_ident: &'a Ident,
    impl_generics: syn::ImplGenerics<'a>,
    ty_generics: syn::TypeGenerics<'a>,
    where_clause: Option<&'a syn::WhereClause>,
    field_ident: &'a Ident,
    path_has_items: &'a proc_macro2::TokenStream,
    path_values: &'a proc_macro2::TokenStream,
    path_entries: &'a proc_macro2::TokenStream,
}

impl<'a> HasItemContext<'a> {
    fn new(
        struct_ident: &'a Ident,
        impl_generics: syn::ImplGenerics<'a>,
        ty_generics: syn::TypeGenerics<'a>,
        field_ident: &'a Ident,
        path_has_items: &'a proc_macro2::TokenStream,
        path_values: &'a proc_macro2::TokenStream,
        path_entries: &'a proc_macro2::TokenStream,
    ) -> Self {
        Self {
            struct_ident,
            impl_generics,
            ty_generics,
            where_clause: None,
            field_ident,
            path_has_items,
            path_values,
            path_entries,
        }
    }

    fn with_where_clause(mut self, where_clause: Option<&'a syn::WhereClause>) -> Self {
        self.where_clause = where_clause;
        self
    }
}

// type HasItemContext<'a> =  {
//     struct_ident: &'a Ident,
//     impl_generics: syn::ImplGenerics<'a>,
//     ty_generics: syn::TypeGenerics<'a>,
//     where_clause: Option<&'a syn::WhereClause>,
//     field_ident: &'a Ident,
//     t: &'a syn::Type,
//     path_has_items: &'a proc_macro2::TokenStream,
//     path_values: &'a proc_macro2::TokenStream,
// };
//

fn gen_impl_vec_values(
    context: &HasItemContext,
    t: &syn::Type, // struct_ident: &Ident,
                   // impl_generics: syn::ImplGenerics<'_>,
                   // ty_generics: syn::TypeGenerics<'_>,
                   // where_clause: Option<&syn::WhereClause>,
                   // field_ident: &Ident,
                   // t: &syn::Type,
                   // path_has_items: &proc_macro2::TokenStream,
                   // path_values: &proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
    let HasItemContext {
        struct_ident,
        impl_generics,
        ty_generics,
        where_clause,
        field_ident,
        path_has_items,
        path_values,
        path_entries: _,
    } = context;
    quote! {
        impl #impl_generics #path_has_items<#path_values> for #struct_ident #ty_generics #where_clause {
            type OwnedItem = #t;
            type BorrowedItem<'a> = &'a #t;
            type IntoItems = ::std::vec::Vec<#t>;
            type IterItems<'a> = ::std::slice::Iter<'a, #t>;

            fn into_items(self) -> Self::IntoItems {
                self.#field_ident
            }
            fn iter_items(&self) -> Self::IterItems<'_> {
                self.#field_ident.iter()
            }
        }
    }
}

fn gen_impl_map_values(
    context: &HasItemContext,
    // struct_ident: &Ident,
    // impl_generics: syn::ImplGenerics<'_>,
    // ty_generics: syn::TypeGenerics<'_>,
    // where_clause: Option<&syn::WhereClause>,
    // field_ident: &Ident,
    // path_has_items: &proc_macro2::TokenStream,
    // path_values: &proc_macro2::TokenStream,
    k: &syn::Type,
    v: &syn::Type,
) -> proc_macro2::TokenStream {
    let HasItemContext {
        struct_ident,
        impl_generics,
        ty_generics,
        where_clause,
        field_ident,
        path_has_items,
        path_values,
        path_entries: _,
    } = context;
    quote! {
        impl #impl_generics #path_has_items<#path_values> for #struct_ident #ty_generics #where_clause {
            type OwnedItem = #v;
            type BorrowedItem<'a> = &'a #v;
            type IntoItems = ::std::collections::hash_map::IntoValues<#k, #v>;
            type IterItems<'a> = ::std::collections::hash_map::Values<'a, #k, #v>;

            fn into_items(self) -> Self::IntoItems {
                self.#field_ident.into_values()
            }
            fn iter_items(&self) -> Self::IterItems<'_> {
                self.#field_ident.values()
            }
        }
    }
}

fn gen_impl_map_entries(
    // struct_ident: &Ident,
    // impl_generics: syn::ImplGenerics<'_>,
    // ty_generics: syn::TypeGenerics<'_>,
    // where_clause: Option<&syn::WhereClause>,
    // field_ident: &Ident,
    // path_has_items: &proc_macro2::TokenStream,
    // path_entries: &proc_macro2::TokenStream,
    context: &HasItemContext,
    k: &syn::Type,
    v: &syn::Type,
) -> proc_macro2::TokenStream {
    let HasItemContext {
        struct_ident,
        impl_generics,
        ty_generics,
        where_clause,
        field_ident,
        path_has_items,
        path_values: _,
        path_entries,
    } = context;
    quote! {
        impl #impl_generics #path_has_items<#path_entries> for #struct_ident #ty_generics #where_clause {
            type OwnedItem = (#k, #v);
            type BorrowedItem<'a> = (&'a #k, &'a #v);
            type IntoItems = ::std::collections::hash_map::IntoIter<#k, #v>;
            type IterItems<'a> = ::std::collections::hash_map::Iter<'a, #k, #v>;

            fn into_items(self) -> Self::IntoItems {
                self.#field_ident.into_iter()
            }
            fn iter_items(&self) -> Self::IterItems<'_> {
                self.#field_ident.iter()
            }
        }
    }
}
