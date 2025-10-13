extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields, GenericArgument, PathArguments, Type};

#[proc_macro_derive(HasVector)]
pub fn has_vec_derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let struct_name = &ast.ident;
    let (field_name, vec_inner_type) = find_vec_field(&ast.data);
    let gen = quote! {
        impl HasVec<#vec_inner_type> for #struct_name {
            fn get_vec(&self) -> &Vec<#vec_inner_type> {
                &self.#field_name
            }
            fn take_vec(self) -> Vec<#vec_inner_type> {
                self.#field_name
            }
        }
    };
    gen.into()
}

fn find_vec_field(data: &Data) -> (proc_macro2::Ident, &Type) {
    if let Data::Struct(s) = data {
        if let Fields::Named(fields) = &s.fields {
            for field in &fields.named {
                if let Type::Path(type_path) = &field.ty {
                    if let Some(last_segment) = type_path.path.segments.last() {
                        if last_segment.ident == "Vec" {
                            if let PathArguments::AngleBracketed(args) = &last_segment.arguments {
                                if let Some(GenericArgument::Type(inner_type)) = args.args.first() {
                                    return (field.ident.clone().unwrap(), inner_type);
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    panic!("HasVec derive macro requires a field of type Vec<T>");
}
