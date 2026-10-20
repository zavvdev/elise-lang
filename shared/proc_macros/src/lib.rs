mod enum_delegate;

use proc_macro::TokenStream;

// Functions annotated with #[proc_macro_derive] must live
// in the root module (lib.rs)

// When the compiler sees #[derive(EnumDelegate)] on a struct, it calls
// derive_enum_delegate in lib.rs with the struct’s tokens. That function delegates
// to enum_delegate::derive, which calls expand. The tokens that come back are appended
// after the struct in the user’s code; a derive never replaces the original item.
// It's required that we accept TokenStream and return it back.

// Generates inherent methods on an enum that forward to the value
// held by each variant.
//
// #[derive(EnumDelegate)]
// #[enum_delegate(
//     /// Doc comments are copied to the generated method.
//     fn span(&self) -> Span;
// )]
// enum Node {
//     A(VariantA),
//     B(VariantB),
// }
// Where VariantA and VariantB must have a function called `span`.
#[proc_macro_derive(EnumDelegate, attributes(enum_delegate))]
pub fn derive_enum_delegate(input: TokenStream) -> TokenStream {
    enum_delegate::derive(input)
}
