extern crate proc_macro;

use anyhow::Context;
use convert_case::{Case, Casing};
use proc_macro::TokenStream;
use std::fs::DirEntry;
use std::path::{Path, PathBuf};

use proc_macro2::{Ident, TokenStream as TokenStream2};
use quote::{format_ident, quote};
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{parse_macro_input, LitStr, Token};

fn sanitize_name(raw: &str) -> Ident {
    format_ident!("{}", raw.to_case(Case::Snake))
}

fn get_directory_tokens(
    path: PathBuf,
    dir: &Path,
    is_pub: bool,
    name_override: Option<Ident>,
) -> anyhow::Result<TokenStream2> {
    let mut items = Vec::new();
    let dir_name = path
        .file_name()
        .and_then(|x| x.to_str())
        .with_context(|| format!("Failed to obtain name of directory at {}", path.display()))?;

    for entry in path
        .read_dir()
        .with_context(|| format!("Failed to read directory `{}`", path.display()))?
    {
        let entry = entry.with_context(|| {
            format!(
                "Failed to fetch info of file in directory `{}`",
                path.display()
            )
        })?;
        items.push(get_entry_tokens(entry, dir)?)
    }

    let dir_name = match name_override {
        None => sanitize_name(dir_name),
        Some(name) => name,
    };

    if is_pub {
        Ok(quote! {
            pub mod #dir_name {
                #(#items)*
            }
        })
    } else {
        Ok(quote! {
            mod #dir_name {
                #(#items)*
            }
        })
    }
}

fn get_entry_tokens(file: DirEntry, dir: &Path) -> anyhow::Result<TokenStream2> {
    let path = file.path();

    let file_type = file
        .file_type()
        .with_context(|| format!("Failed to get type of file `{}`", path.display()))?;
    if file_type.is_dir() {
        return get_directory_tokens(path, dir, true, None);
    }

    let stripped_full_path = file
        .path()
        .strip_prefix(dir)
        .with_context(|| format!("File `{}` is outside of assets folder", path.display()))?
        .to_str()
        .with_context(|| format!("Failed to convert OS string, at file `{}`", path.display()))?
        .replace('\\', "/");

    let full_path_no_ext = path.with_extension("");
    let file_name = full_path_no_ext
        .file_name()
        .and_then(|x| x.to_str())
        .unwrap_or_else(|| panic!("Failed to obtain name of file at {}", path.display()))
        .to_uppercase();

    let file_name = sanitize_name(&file_name);

    let asset_literal = LitStr::new(stripped_full_path.as_str(), proc_macro2::Span::call_site());

    Ok(quote! {
        pub const #file_name: &'static str = #asset_literal;
    })
}

mod kv {
    syn::custom_keyword!(from);
}

// #[derive(Parse)]
struct DeclarationInput {
    is_pub: bool,
    root_module_name: Ident,
    path: String,
}

impl Parse for DeclarationInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let is_pub = input.parse::<Token![pub]>().is_ok();
        input.parse::<Token![mod]>()?;
        let root_module_name = input.parse::<Ident>()?;
        let path = if input.parse::<Token![=]>().is_ok() {
            input.parse::<LitStr>()?.value()
        } else {
            root_module_name.to_string()
        };
        Ok(DeclarationInput {
            is_pub,
            root_module_name,
            path,
        })
    }
}

struct MacroInput {
    root_path: String,
    declarations: Punctuated<DeclarationInput, Token![;]>,
}

impl Parse for MacroInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input.parse::<Token![use]>()?;
        let root_path = input.parse::<LitStr>()?.value();
        input.parse::<Token![;]>()?;
        let declarations = input.parse_terminated(DeclarationInput::parse, Token![;])?;
        Ok(MacroInput {
            root_path,
            declarations,
        })
    }
}

fn pprint_err(err: anyhow::Error) -> String {
    let mut msg = format!("{}", err);

    for err in err.chain() {
        msg.push_str(&format!("\nCaused by: {}", err));
    }

    msg
}

fn generate_assets_impl(arg: MacroInput) -> Result<TokenStream2, syn::Error> {
    // Get the directory of the current crate
    let dir = std::env::var("CARGO_MANIFEST_DIR").expect("Failed to fetch manifest dir");

    let asset_dir = Path::new(&dir).join(arg.root_path);

    if !asset_dir.is_dir() {
        return Err(syn::Error::new(
            proc_macro2::Span::call_site(),
            format!("Directory `{}` does not exist", asset_dir.display()),
        ));
    }

    let mut items = Vec::new();
    for declaration in arg.declarations {
        let span = declaration.root_module_name.span();
        items.push(
            get_directory_tokens(
                asset_dir.join(PathBuf::from(declaration.path)),
                &asset_dir,
                declaration.is_pub,
                Some(declaration.root_module_name),
            )
            .map_err(|err| syn::Error::new(span, pprint_err(err)))?,
        )
    }

    Ok(quote! {
        #(#items)*
    })
}

#[proc_macro]
pub fn generate_assets(args: TokenStream) -> TokenStream {
    let arg = parse_macro_input!(args as MacroInput);

    match generate_assets_impl(arg) {
        Ok(tokens) => TokenStream::from(tokens),
        Err(err) => err.to_compile_error().into(),
    }
}
