// Difference between proc_macro and proc_macro2.
//
// proc_macro is the compiler’s built-in API for writing macros, but it only
// works while the compiler is actually expanding a macro. proc_macro2 is a
// wrapper that mimics it and works everywhere. We need both because the compiler
// demands one (proc_macro) at the boundary, while the ecosystem and our own code
// want the other (proc_macro2). `syn` and `quote` depend on proc-macro2 internally.
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
// rust compiler
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
// rust compiler

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{
    Data, DeriveInput, Fields, FnArg, Pat, TraitItemFn, parse::ParseStream, parse_macro_input,
};

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
    // 3. ident: the name (AstExpr for example if it's a `struct AstExpr`)
    // 4. generics: generic parameters and `where` clauses
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

fn expand(input: DeriveInput) -> syn::Result<TokenStream2> {
    // `input` is a set of tokens that represent an enum that we used
    // our derive macro on. Ex:
    //
    // #[derive(EnumDelegate)]
    // pub enum Some { ... }
    //
    // so `input` has something like:
    // {
    //   vis: Visibility::Public(
    //      Token![pub],
    //   ),
    //   ident: Ident {
    //      ident: "Some",
    //      span: #0 bytes(126..130),
    //   },
    //   data: Data::Enum { .. }
    // }
    //
    // Here we select the name of our enum.
    let name = &input.ident;

    // `input.data` holds a type of Data we used our derive macro on.
    // Its variants are:
    // pub enum Data {
    //     Struct(DataStruct),
    //     Enum(DataEnum),
    //     Union(DataUnion),
    // }
    //
    // So here we need to match that it's an enum.
    let data = match &input.data {
        Data::Enum(data) => data,
        _ => {
            return Err(syn::Error::new_spanned(
                name,
                "EnumDelegate can only be derived for enums",
            ));
        }
    };

    // Collect methods from every #[enum_delegate(...)] attribute.
    // `input.attrs` is a list of attribute macros. For example:
    //
    // #[enum_delegate(
    //    fn span(&self) -> Span;
    // )]
    // pub enum Some { .. }
    //
    // In this case, `input.attrs` will contain something like:
    //
    // [
    //    {
    //       pound_token: Token![#],
    //       meta: Meta::List {
    //          path: Path {
    //              segments: [
    //                  PathSegment {
    //                      ident: Ident {
    //                          ident: "enum_delegate",
    //                      },
    //                  },
    //              ],
    //          },
    //          delimiter: MacroDelimiter::Paren(Paren),
    //          tokens: TokenStream [
    //              Ident { ident: "fn" },
    //              Ident { ident: "span" },
    //              Group {
    //                  delimiter: Parenthesis,
    //                  stream: TokenStream [
    //                      Punct { ch: '&' },
    //                      Ident { ident: "self" },
    //                  ],
    //              },
    //              Punct { ch: '-' },
    //              Punct { ch: '>' },
    //              Ident { ident: "Span" },
    //              Punct { ch: ';' },
    //          ],
    //       }
    //    }
    // ]
    let mut methods: Vec<TraitItemFn> = Vec::new();

    for attr in input
        .attrs
        .iter()
        // Get only our target attribute.
        .filter(|a| a.path().is_ident("enum_delegate"))
    {
        // Takes the tokens inside the parentheses (see `tokens` on Attribute above)
        // and passes them to our closure where we parse each of them as TraitItemFn.
        //
        // Why do we need to parse each attr?
        //
        // Because the attribute's contents arrive as raw tokens. The compiler checks
        // that #[enum_delegate(...)] is a valid attribute, but it doesn’t look inside
        // the parentheses. `attr` holds the tokens `fn span (&self) -> Span;` as a flat
        // list, with no idea what they mean. Parsing is how we turn that list into
        // something we can ask questions about.
        //
        // What the macro needs to know. To generate this:
        //
        // impl Some {
        //    pub fn rename(&mut self, name: String) {
        //       match self {
        //           Self::Expr(inner) => inner.rename(name),
        //           ...
        //       }
        //    }
        // }
        //
        // the macro has to answer questions the flat tokens can't:
        //
        // 1. What's the method name, so we can write inner.rename(...)?
        // 2. What are the parameter names, so we can forward name?
        // 3. Is there a self receiver at all?
        // 4. Where does one method end and the next begin?
        // 5. Did the user write a body by mistake?
        //
        // Answering these from a flat token list means scanning by hand:
        // find fn, take the next token as the name, find the parenthesized
        // group, split it on commas (but not commas inside generics like
        // HashMap<K, V>), skip the receiver, and take the identifier before
        // each :. That’s a parser, and a fragile one. `syn` already has a
        // tested one, so we use it.
        //
        // What parsing gives us:
        // 1. Structure: m.sig.ident, m.sig.inputs, m.sig.receiver() and m.default
        // are typed fields instead of token positions.
        // 2. Validation for free: a malformed signature like `fn span(&self -> Span;`
        // fails with a syn error pointing at the broken token, instead of producing
        // garbage code.
        // 3. Reusable pieces: `sig` can be pasted into the output with `#sig` and prints
        // as valid syntax.
        methods.extend(attr.parse_args_with(|token_stream: ParseStream| {
            let mut v = Vec::new();
            while !token_stream.is_empty() {
                // What is TraitItemFn and why we use it for parsing:
                //
                // TraitItemFn is a plain struct from `syn` that describes a method
                // declaration as it can appear inside a trait. It holds no behavior.
                // It’s data, produced by a parser from tokens.
                //
                // Its definition is roughly:
                //
                // pub struct TraitItemFn {
                //     pub attrs: Vec<Attribute>,       // doc comments and other #[...] on the method
                //     pub sig: Signature,              // the signature
                //     pub default: Option<Block>,      // Some(body) if the method has a body
                //     pub semi_token: Option<Token![;]>, // the trailing `;` when there's no body
                // }
                //
                // and Signature holds the parts of the function header:
                //
                // pub struct Signature {
                //     pub ident: Ident,                        // method name
                //     pub generics: Generics,                  // <T> and where clauses
                //     pub inputs: Punctuated<FnArg, Token![,]>, // parameters, including self
                //     pub output: ReturnType,                  // `-> Span` or nothing
                //     // plus const / async / unsafe / abi / variadic markers
                // }
                //
                // TraitItemFn is one of a family of similar syn types, each describing a function
                // in a different position:
                //
                // 1. ItemFn: a free function with a body, like `fn foo() {}` at module level
                // 2. ImplItemFn: a method inside an `impl block`, which must have a body
                // 3. TraitItemFn: a method inside a trait, where the body is optional
                //
                // We pick TraitItemFn because its syntax is exactly what we want the user to write.
                v.push(token_stream.parse::<TraitItemFn>()?);
            }
            Ok(v)
        })?);
    }

    if methods.is_empty() {
        return Err(syn::Error::new_spanned(
            name,
            "EnumDelegate needs at least one method, e.g. #[enum_delegate(fn span(&self) -> Span;)]",
        ));
    }

    // Every variant must look like `Variant(Inner)`.
    let variants = data
        .variants
        .iter()
        .map(|v| match &v.fields {
            Fields::Unnamed(f) if f.unnamed.len() == 1 => Ok(&v.ident),
            _ => Err(syn::Error::new_spanned(
                v,
                "EnumDelegate requires every variant to have exactly one unnamed field",
            )),
        })
        .collect::<syn::Result<Vec<_>>>()?;

    // One generated method per declared signature.
    let fns = methods
        .iter()
        .map(|m| {
            // In a trait, a method can optionally come with a default implementation:
            // trait Example {
            //     fn required(&self) -> u32;          // no body: implementors must provide it
            //
            //     fn provided(&self) -> u32 { 42 }    // has a body: the "default" implementation
            // }
            // syn’s TraitItemFn mirrors this grammar, so `default` is the optional body
            if let Some(body) = &m.default {
                return Err(syn::Error::new_spanned(
                    body,
                    "write only the signature, ending with `;`",
                ));
            }
            if m.sig.receiver().is_none() {
                return Err(syn::Error::new_spanned(
                    &m.sig,
                    "the method must take `self`, `&self` or `&mut self`",
                ));
            }

            // Names of the non-self parameters, forwarded in the inner call.
            //
            // `sig.inputs` is a list of FnArg values, one per parameter, and
            // FnArg has two variants:
            //    pub enum FnArg {
            //        Receiver(Receiver),   // self, &self, &mut self
            //        Typed(PatType),       // pattern: Type
            //    }
            // PatType is the `name: String` shape.
            //
            // For fn rename(&mut self, name: String, count: usize); the list is:
            // [ Receiver(&mut self), Typed(name: String), Typed(count: usize) ]
            let args = m
                .sig
                .inputs
                .iter()
                .filter_map(|a| match a {
                    FnArg::Typed(pt) => Some(match &*pt.pat {
                        Pat::Ident(p) => Ok(&p.ident),
                        // What 'plain parameters' means.
                        // The left side of the colon is a Pat, syn’s type for any Rust pattern,
                        // the same grammar we use in let and match. A plain identifier is just
                        // a name, like `name`. Parameters can be other patterns too:
                        //
                        // fn a(&self, name: String); --> Pat::Ident - plain identifier
                        // fn b(&self, _: u32); --> Pat::Wild - underscore, no name at all
                        // fn c(&self, (x, y): (u32, u32)); --> Pat::Tuple - destructures a tuple
                        // fn f(&self, [a, b]: [u8; 2]); --> Pat::Slice
                        //
                        // Why we reject the other patterns
                        //
                        // The generated code has to forward each argument: inner.rename(name).
                        // That needs one name per parameter.
                        // `_: u32` has no name, so there’s nothing to write in the call.
                        // `(x, y): (u32, u32)` binds two names, `x` and `y`, from one argument.
                        // We’d have to rebuild the tuple `(x, y)` to forward it, and that’s only
                        // possible for simple patterns. Rejecting them with a clear error is
                        // simpler than supporting a subset by accident.
                        other => Err(syn::Error::new_spanned(
                            other,
                            "parameters must be plain identifiers",
                        )),
                    }),
                    FnArg::Receiver(_) => None,
                })
                .collect::<syn::Result<Vec<_>>>()?;

            let attrs = &m.attrs;
            let sig = &m.sig;
            let method = &m.sig.ident;

            // Built outside the variants repetition so `args` isn't iterated twice.
            let call_args = quote! { #(#args),* };

            Ok(quote! {
                #(#attrs)*
                pub #sig {
                    match self {
                        #(Self::#variants(inner) => inner.#method(#call_args),)*
                    }
                }
            })
        })
        .collect::<syn::Result<Vec<_>>>()?;

    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    Ok(quote! {
        impl #impl_generics #name #ty_generics #where_clause {
            #(#fns)*
        }
    })
}
