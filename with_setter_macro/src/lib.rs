extern crate proc_macro;

use darling::{ast, FromDeriveInput, FromField};
use proc_macro::TokenStream;
use quote::{format_ident, quote, quote_spanned};
use syn::spanned::Spanned;
use syn::{parse_macro_input, DeriveInput, Visibility};

#[derive(FromDeriveInput)]
#[darling(
    attributes(setters),
    forward_attrs(allow, doc, cfg),
    supports(struct_any)
)]
struct WithSetterAttr {
    /// The struct ident.
    ident: syn::Ident,

    data: ast::Data<(), FieldReceiver>,

    /// The type's generics. You'll need these any time your trait is expected
    /// to work with types that declare generics.
    generics: syn::Generics,

    #[darling(rename = "vis")]
    visibility: Option<Visibility>,
}

#[derive(Debug, FromField)]
#[darling(attributes(setters))]
struct FieldReceiver {
    ident: Option<syn::Ident>,

    vis: Visibility,

    ty: syn::Type,

    /// Field name override
    name: Option<syn::Ident>,

    /// Visibility override
    #[darling(rename = "vis")]
    visibility: Option<Visibility>,

    /// Skips the field
    skip: darling::util::Flag,
}

#[proc_macro_derive(WithSetters, attributes(setters))]
pub fn with_setters(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);
    let data = match WithSetterAttr::from_derive_input(&input) {
        Ok(data) => data,
        Err(err) => return err.write_errors().into(),
    };

    let fields = data
        .data
        .as_ref()
        .take_struct()
        .expect("Should never be an enum")
        .fields;

    let (imp, ty, wher) = data.generics.split_for_impl();

    let mut setters = vec![];
    for (i, field) in fields.iter().enumerate() {
        if field.skip.is_present() {
            continue;
        }
        let Some(field_ident) = field.ident.as_ref().or(field.name.as_ref()) else {
            return syn::Error::new(field.ty.span(), "Unnamed field should either be skipped or annotated with #[setter(name=\"custom_name\")]").into_compile_error().into();
        };

        let accessor = field
            .ident
            .as_ref()
            .map(|name| quote!(#name))
            .unwrap_or_else(|| quote!(#i));

        let with_name = format_ident!("with_{}", field_ident);
        let vis = field
            .visibility
            .as_ref()
            .or(data.visibility.as_ref())
            .unwrap_or(&field.vis);

        let ty = &field.ty;
        setters.push(quote_spanned! {field_ident.span()=>
            #vis fn #with_name(mut self, #field_ident: impl Into<#ty>) -> Self {
                self.#accessor = #field_ident.into();
                self
            }
        });
    }

    let ident = &data.ident;

    let impl_block = quote! {
        impl #imp #ident #ty #wher {
            #(#setters)*
        }
    };

    TokenStream::from(impl_block)
}
