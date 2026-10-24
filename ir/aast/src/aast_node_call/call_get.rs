use elise_bindings::{binding_path::BindingPath, type_bindings::TypeBinding};
use elise_shared_types::Span;
use getset::CopyGetters;

#[derive(Debug, PartialEq, CopyGetters)]
pub struct AAstNodeCallGet {
    #[getset(get_copy = "pub")]
    pub span: Span,

    // Binding path for type resolution only.
    // Does not include literal path segments like
    // Index(0) but rather abstract ones like
    // AbstractIndex.
    pub type_binding_path: BindingPath,

    // Binding path for data accessing.
    // Does not include abstract path segments but
    // literal ones, like Index(0).
    //
    // NOTE: Not 100% sure if we need this here,
    // but our compiler will probably need to know
    // about data access path in order to generate
    // op code.
    pub data_binding_path: BindingPath,

    // Return type of the .get function represented
    // by TypeBinding structure, where each key has
    // a type_binding_path removed.
    //
    // For example, if we have this type definition:
    //
    // .typedef (User :Record<{
    //                  "id" :Int,
    //                  "name" :String }>)
    // .typedef (Data :List<User>)
    //
    // Which produces the next binding:
    // [Root] => List
    // [Root, AbstractIndex] => Record
    // [Root, AbstractIndex, Field("id")] => Int
    // [Root, AbstractIndex, Field("name")] => Str
    //
    // then after accessing data with:
    // .get (@data, 0)
    // that generates the next binding path:
    // [Root, AbstractIndex]
    // our return_type_binding must be:
    //
    // [Root] => Record
    // [Root, Field("id")] => Int
    // [Root, Field("name")] => Str
    //
    // where we strip each path by prefix represented
    // by path generated from .get data access.
    pub return_type_binding: TypeBinding,
}
