mod has_span;

use proc_macro::TokenStream;

// Functions annotated with #[proc_macro_derive] must live
// in the root module (lib.rs)

// When the compiler sees #[derive(HasSpan)] on a struct, it calls derive_has_span
// in lib.rs with the struct’s tokens. That function delegates to has_span::derive,
// which calls expand. The tokens that come back are appended after the struct in the
// user’s code; a derive never replaces the original item.
// It's required that we accept TokenStream and return it back.
#[proc_macro_derive(HasSpan)]
pub fn derive_has_span(input: TokenStream) -> TokenStream {
    has_span::derive(input)
}
