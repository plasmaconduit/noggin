//! Do not import or use this crate directly, import and use `noggin` instead.
//! See: [noggin](https://docs.rs/noggin/latest/noggin/)

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::{format_ident, quote};
use syn::punctuated::Punctuated;
use syn::Data;
use syn::DataStruct;
use syn::DeriveInput;
use syn::GenericArgument;
use syn::Ident;
use syn::PathArguments;
use syn::Token;
use syn::Type;
use syn::{Field, GenericParam};

fn extend_decoding_params(
    params: &Punctuated<GenericParam, Token![,]>,
) -> proc_macro2::TokenStream {
    let lifetimes: Vec<_> = params
        .iter()
        .filter_map(|p| match p {
            GenericParam::Lifetime(l) => Some(&l.lifetime),
            _ => None,
        })
        .collect();
    if lifetimes.is_empty() {
        quote! { 'de, #params }
    } else {
        quote! { 'de: #(#lifetimes)+*, #params }
    }
}

fn is_type_container(name: &str, ty: &Type) -> bool {
    if let Type::Path(type_path) = &ty {
        let option = Ident::new(name, Span::call_site());
        let first_segment = type_path.path.segments.first().unwrap();
        return first_segment.ident == option;
    }
    false
}

fn is_type_option(ty: &Type) -> bool {
    is_type_container("Option", ty)
}

fn is_type_vec(ty: &Type) -> bool {
    is_type_container("Vec", ty)
}

fn get_field_ident(field: &Field) -> &Ident {
    field.ident.as_ref().unwrap()
}

fn get_first_generic_type(ty: &Type) -> &Type {
    let type_path = match ty {
        Type::Path(type_path) => type_path,
        _ => panic!("type doesn't have generic arguments"),
    };
    let last_segment = type_path.path.segments.last().unwrap();
    let generics = match &last_segment.arguments {
        PathArguments::AngleBracketed(generics) => &generics.args,
        _ => panic!("type doesn't have generic arguments"),
    };
    let generic_type = generics
        .iter()
        .find_map(|g| match g {
            GenericArgument::Type(gt) => Some(gt),
            _ => None,
        })
        .expect("type doesn't have generic arguments");
    generic_type
}

enum HeaderField<'a> {
    RequiredSingle(&'a Ident, &'a Type),
    RequiredRepeated(&'a Ident, &'a Type),
    OptionalSingle(&'a Ident, &'a Type),
    OptionalRepeated(&'a Ident, &'a Type),
}

impl<'a> HeaderField<'a> {
    pub(crate) fn parse_all(data: &DataStruct) -> Vec<HeaderField> {
        data.fields
            .iter()
            .map(|field| {
                let ident = get_field_ident(field);
                if is_type_option(&field.ty) {
                    let optional_type = get_first_generic_type(&field.ty);
                    if is_type_vec(optional_type) {
                        let repeated_type = get_first_generic_type(optional_type);
                        HeaderField::OptionalRepeated(ident, repeated_type)
                    } else {
                        HeaderField::OptionalSingle(ident, optional_type)
                    }
                } else if is_type_vec(&field.ty) {
                    let repeated_type = get_first_generic_type(&field.ty);
                    HeaderField::RequiredRepeated(ident, repeated_type)
                } else {
                    HeaderField::RequiredSingle(ident, &field.ty)
                }
            })
            .collect()
    }

    pub(crate) fn make_declaration(&self) -> proc_macro2::TokenStream {
        match self {
            HeaderField::RequiredSingle(ident, ty) | HeaderField::OptionalSingle(ident, ty) => {
                let maybe_ident = format_ident!("maybe_{ident}");
                quote! {
                    let mut #maybe_ident: Option<#ty> = None;
                }
            }
            HeaderField::RequiredRepeated(ident, ty) | HeaderField::OptionalRepeated(ident, ty) => {
                let maybe_ident = format_ident!("maybe_{ident}");
                quote! {
                    let mut #maybe_ident: Vec<#ty> = vec![];
                }
            }
        }
    }

    pub(crate) fn make_extractor(&self, key: &Ident, value: &Ident) -> proc_macro2::TokenStream {
        match self {
            HeaderField::RequiredSingle(ident, ty) | HeaderField::OptionalSingle(ident, ty) => {
                let maybe_ident = format_ident!("maybe_{ident}");
                let header_key = ident.to_string().replace('_', "-");
                quote! {
                    if #maybe_ident.is_none() && #key.eq_ignore_ascii_case(#header_key) {
                        let #ident: #ty = noggin::FromHeaderValue::parse_header_value(#value)
                            .ok_or(noggin::Error::InvalidHeaderValue(#header_key))?;
                        #maybe_ident = Some(#ident);
                    }
                }
            }
            HeaderField::RequiredRepeated(ident, ty) | HeaderField::OptionalRepeated(ident, ty) => {
                let maybe_ident = format_ident!("maybe_{ident}");
                let header_key = ident.to_string().replace('_', "-");
                quote! {
                    if #key.eq_ignore_ascii_case(#header_key) {
                        let #ident: Vec<#ty> = noggin::FromHeaderValue::parse_header_value(#value)
                            .ok_or(noggin::Error::InvalidHeaderValue(#header_key))?;
                        #maybe_ident.extend(#ident);
                    }
                }
            }
        }
    }

    pub(crate) fn make_validator(&self) -> proc_macro2::TokenStream {
        match self {
            HeaderField::RequiredSingle(ident, _) => {
                let maybe_ident = format_ident!("maybe_{ident}");
                let header_key = ident.to_string().replace('_', "-");
                quote! {
                    if #maybe_ident.is_none() {
                        return Err(noggin::Error::MissingHeader(#header_key));
                    }
                }
            }
            HeaderField::RequiredRepeated(ident, _) => {
                let maybe_ident = format_ident!("maybe_{ident}");
                let header_key = ident.to_string().replace('_', "-");
                quote! {
                    if #maybe_ident.is_empty() {
                        return Err(noggin::Error::MissingHeader(#header_key));
                    }
                }
            }
            _ => quote! {},
        }
    }

    pub(crate) fn make_builders(&self) -> proc_macro2::TokenStream {
        match self {
            HeaderField::RequiredSingle(ident, _) => {
                let maybe_ident = format_ident!("maybe_{ident}");
                quote! {
                    #ident: #maybe_ident.unwrap()
                }
            }
            HeaderField::RequiredRepeated(ident, _) | HeaderField::OptionalSingle(ident, _) => {
                let maybe_ident = format_ident!("maybe_{ident}");
                quote! {
                    #ident: #maybe_ident
                }
            }
            HeaderField::OptionalRepeated(ident, _) => {
                let maybe_ident = format_ident!("maybe_{ident}");
                quote! {
                    #ident: (!#maybe_ident.is_empty()).then_some(#maybe_ident)
                }
            }
        }
    }
}

#[proc_macro_derive(Noggin)]
pub fn noggin_derive(input: TokenStream) -> TokenStream {
    let derive_input = syn::parse_macro_input!(input as DeriveInput);
    match &derive_input.data {
        Data::Struct(data) => {
            let name = &derive_input.ident;
            let params = &derive_input.generics.params;
            let extended_params = extend_decoding_params(params);
            let fields = HeaderField::parse_all(data);
            let key = Ident::new("key", Span::call_site());
            let value = Ident::new("value", Span::call_site());
            let declarations: Vec<_> = fields.iter().map(|f| f.make_declaration()).collect();
            let extractors: Vec<_> = fields
                .iter()
                .map(|f| f.make_extractor(&key, &value))
                .collect();
            let validators: Vec<_> = fields.iter().map(|f| f.make_validator()).collect();
            let builders: Vec<_> = fields.iter().map(|f| f.make_builders()).collect();
            let result = quote! {
                impl<#extended_params> noggin::HeadParser<'de> for #name<#params> {
                    fn parse_head_section(head: &'de str) -> Result<Self, noggin::Error> {
                        #(
                            #declarations
                        )*
                        for header in head.split("\r\n") {
                            let (key, value) = header.split_once(':')
                                .ok_or(noggin::Error::MalformedHeader)?;
                            #(
                                #extractors
                            )*
                        }
                        #(
                            #validators
                        )*
                        let result = #name {
                            #(
                                #builders
                            ),*
                        };
                        Ok(result)
                    }
                }
            };
            result.into()
        }
        _ => panic!("Noggin derive macro only works on struct types"),
    }
}
