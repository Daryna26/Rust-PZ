use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Expr, Token, punctuated::Punctuated};


/// Procedural macro to create BTreeMap
#[proc_macro]
pub fn btreemap(input: TokenStream) -> TokenStream {
let pairs = parse_macro_input!(input with Punctuated::<Expr, Token![,]>::parse_terminated);


let mut inserts = Vec::new();
let mut iter = pairs.iter();
while let Some(key) = iter.next() {
if let Some(value) = iter.next() {
inserts.push(quote! { map.insert(#key, #value); });
}
}


let expanded = quote! {{
let mut map = ::std::collections::BTreeMap::new();
#(#inserts)*
map
}};


TokenStream::from(expanded)
}