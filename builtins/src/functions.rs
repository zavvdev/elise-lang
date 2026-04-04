use elise_ast::AstNodeKind;
use phf::phf_map;

pub struct Arg {
    pub expected_type: AstNodeKind,
}

pub struct BuiltinFn {
    pub name: &'static str,
    // length implicitly defines arity.
    pub args: &'static [Arg],
    // if true, last Arg applies to all remaining arguments.
    pub variadic: bool,
}

pub static BUILTIN_FUNCTIONS: phf::Map<&'static str, BuiltinFn> = phf_map! {
    "let"  => BuiltinFn {
                name: "let",
                args: &[
                    Arg { expected_type: AstNodeKind::List },
                    Arg { expected_type: AstNodeKind::Any },
                ],
                variadic: true,
            },
};
