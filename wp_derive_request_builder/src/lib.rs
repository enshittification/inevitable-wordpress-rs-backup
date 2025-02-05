use proc_macro::TokenStream;
use syn::parse_macro_input;

mod generate;
mod parse;
mod variant_attr;

#[proc_macro_derive(
    WpDerivedRequest,
    attributes(SparseField, Namespace, contextual_get, delete, get, post)
)]
pub fn derive(input: TokenStream) -> TokenStream {
    let parsed_enum = parse_macro_input!(input as parse::ParsedEnum);

    if cfg!(feature = "generate_request_builder") {
        generate::generate_types(&parsed_enum).into()
    } else {
        TokenStream::new()
    }
}
