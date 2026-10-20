// Difference between proc_macro and proc_macro2.
//
// proc_macro is the compiler’s built-in API for writing macros, but it only
// works while the compiler is actually expanding a macro. proc_macro2 is a
// wrapper that mimics it and works everywhere. We need both because the compiler
// demands one (proc_macro) at the boundary, while the ecosystem and our own code
// want the other (proc_macro2). syn and quote depend on proc-macro2 internally.
//
// proc_macro
// 1. Ships with the compiler (part of the standard distribution). We don’t list it
//    in Cargo.toml; it’s available automatically in any crate with proc-macro = true.
// 2. Its types (TokenStream, TokenTree, Ident...) are thin handles into the
//    compiler’s internals.
// 3. It can only be used from inside a running macro expansion. Calling it from a unit
//    test, a build.rs, a CLI, or any ordinary binary panics with something like
//    “procedural macro API is used outside of a procedural macro”.
// 4. It can only be used in crates with proc-macro = true, so we can’t share code that
//    touches it with normal crates.
// 5. It’s the only type the compiler accepts for entry points: fn(TokenStream) -> TokenStream.
//
// proc_macro2 (https://crates.io/crates/proc-macro2)
// 1. A regular crate that re-implements the same API.
// 2. Inside a macro expansion, it wraps the real proc_macro types and forwards calls to them.
//    Outside one, it falls back to a pure-Rust implementation.
// 3. Because of that, code built on it can be unit-tested, used in a normal library, or reused
//    in a CLI or build script.
// 4. It adds a few things the compiler API lacks, such as the ability to construct and inspect
//    tokens outside expansions.
//
// Roughly how it works:
// compiler
//   │  proc_macro::TokenStream
//   ▼
// derive()          ← parse_macro_input! converts to syn types
//   │  DeriveInput (syn)
//   ▼
// expand()          ← pure logic, proc_macro2 + syn + quote only
//   │  proc_macro2::TokenStream
//   ▼
// derive()          ← TokenStream::from(...) converts back
//   │  proc_macro::TokenStream
//   ▼
// compiler

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{Data, DeriveInput, Fields, parse_macro_input};

// pub(crate) means the item is visible anywhere inside the current crate,
// but not to other crates that depend on it. It sits between private and fully pub.
//
// Rust’s visibility levels, from narrowest to widest:
// no modifier: visible only in the current module (and its child modules)
// pub(super): visible in the parent module too
// pub(crate): visible anywhere in the same crate
// pub: visible to anyone who can reach the item, including other crates

// `input` is a raw proc_macro::TokenStream: the struct definition as unstructured tokens.
// Ex: (pub struct AstExpr { ... }), with the #[derive(...)] attribute itself stripped off.
pub(crate) fn derive(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree.
    // `DeriveInput` is syn’s type for “anything you can put #[derive] on”, and has these fields:
    // 1. attrs: other attributes on the item
    // 2. vis: visibility (pub, etc.)
    // 3. ident: the name (AstExpr)
    // 4. generics: generic parameters and where clauses
    // 5. data: the body, as Data::Struct, Data::Enum or Data::Union
    // If parsing fails, it automatically returns TokenStream with compiler error.
    let input = parse_macro_input!(input as DeriveInput);

    // This does the real work and returns syn::Result<TokenStream2>. On success we get the
    // generated impl block. On error, we turn it into a TokenStream of compiler error.
    let expanded = match expand(input) {
        Ok(tokens) => tokens,
        Err(err) => err.into_compile_error(),
    };

    // This converts from proc_macro2::TokenStream to proc_macro::TokenStream.
    TokenStream::from(expanded)
}

// A syntax tree goes in, tokens (proc_macro2::TokenStream) come out.
fn expand(input: DeriveInput) -> syn::Result<TokenStream2> {
    // Get struct's name on which we call derive macro.
    let name = &input.ident;

    // input.data must be a Data::Struct, not an enum or union.
    // The struct’s fields must be Fields::Named, as in { a: T, b: U }. The other variants
    // are Unnamed (tuple structs like struct S(u32)) and Unit (struct S;), and neither can
    // have a field called `span`.
    let fields = match &input.data {
        Data::Struct(s) => match &s.fields {
            // In the success case, fields is &Punctuated<Field, Comma>, which is a list of
            // fields we can iterate over. Every failure path returns early with
            // syn::Error::new_spanned(name, "..."). That creates an error attached to
            // the struct name’s location, so the user sees the red squiggle on BinaryExpr
            // in their own file.
            Fields::Named(f) => &f.named,
            _ => {
                return Err(syn::Error::new_spanned(
                    name,
                    "HasSpan requires a struct with named fields",
                ));
            }
        },
        _ => {
            return Err(syn::Error::new_spanned(
                name,
                "HasSpan can only be derived for structs",
            ));
        }
    };

    // Find a field named `span`.
    let span_field = fields
        .iter()
        .find(|f| f.ident.as_ref().is_some_and(|i| *i == "span"))
        .ok_or_else(|| syn::Error::new_spanned(name, "HasSpan requires a field named `span`"))?;

    // This takes the field’s declared type as a syn::Type. We’re reusing whatever the user wrote
    // (Span, crate::Span, ast::Span), so the macro doesn’t need to know where Span lives.
    let ty = &span_field.ty;

    // This handles generic structs. It returns three pieces that print differently, so we can
    // build a correct impl header. For struct Foo<T: Clone> where T: Debug:
    // `impl_generics` is `<T: Clone>`
    // `ty_generics` is `<T>`
    // `where_clause` is `where T: Debug`
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    // `quote!` builds a token stream from a template. Anything written as #variable is replaced
    // by that variable’s tokens, which works for any type implementing ToTokens (idents, types,
    // generics pieces, and so on). Everything else is emitted literally. The result is a
    // proc_macro2::TokenStream.
    //
    // Ex:
    // impl AstExpr {
    //    pub fn span(&self) -> Span {
    //        self.span
    //    }
    // }
    Ok(quote! {
        // Used as `impl #impl_generics Foo #ty_generics #where_clause`, they give
        // `impl<T: Clone> Foo<T> where T: Debug`. For a non-generic struct all three print
        // as nothing, so the same template works for both cases.
        impl #impl_generics #name #ty_generics #where_clause {
            pub fn span(&self) -> #ty {
                self.span
            }
        }
    })
}
