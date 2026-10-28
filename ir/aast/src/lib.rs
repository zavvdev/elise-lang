//! AAST - Annotated Abstract Syntax Tree
//!
//! This data structure is an extension of AAS (Abstract Syntax Tree).
//! It's the result of the Semantic Analysis (Harmony) that adds as
//! much type information as possible and provides more granular
//! differentiation between known nodes such as functions or slots.
//!
//! For example, if our AAST node generalizes function calls as
//! one node with a lexeme, or Slot is also represented as one
//! node, AAST, on the other hand, creates a dedicated nodes for
//! each of them which gives us as much information about a
//! particular entity as possible. We do so, because each function
//! can behave differently and can have different properties,
//! so it's not possible anymore to generalize them under one type.

use elise_bindings::{binding_path::BindingPath, type_bindings::TypeBinding};
use elise_shared_proc_macros::EnumDelegate;
use elise_shared_types::Span;
use getset::CopyGetters;

use crate::{
    aast_node_data::AAstNodeData, aast_node_slot::AAstNodeSlot, aast_type_annot::AAstNodeTypeAnnot,
};

pub mod aast_node_data;
pub mod aast_node_slot;
pub mod aast_type_annot;

#[derive(Debug, PartialEq, EnumDelegate)]
#[enum_delegate(
    fn span(&self) -> Span;
)]
pub enum AAstNode {
    // Call variant represents a set of known function calls
    // where each of them contains it's own properties.
    Call(AAstNodeCall),

    // Data variant represents a set of nodes that act as a
    // literal data, such as: integers, strings, lists etc.
    // This allows us to do more granular validation of
    // arguments for functions that must accept only
    // Data nodes.
    Data(AAstNodeData),

    // Represents a dedicated @data slot that references
    // data being manipulated.
    SlotData(AAstNodeSlot),
}

// ==================================================================
//
// CALL START
//
// We keep AAst nodes for calls here because they require
// references to AAstNode itself because of nesting.
//
// ==================================================================

#[derive(Debug, PartialEq, EnumDelegate)]
#[enum_delegate(
    fn span(&self) -> Span;
)]
pub enum AAstNodeCall {
    Typedef(AAstNodeCallTypedef),
    Get(AAstNodeCallGet),
}

// ==================================================================
// CALL TYPEDEF START
// ==================================================================

#[derive(Debug, PartialEq, CopyGetters)]
pub struct AAstNodeCallTypedef {
    // Raw alias for a new type created with
    // .typedef function.
    pub alias: String,
    pub annot: AAstNodeTypeAnnot,

    #[getset(get_copy = "pub")]
    pub span: Span,
}

// ==================================================================
// CALL TYPEDEF END
// ==================================================================

// ==================================================================
// CALL GET START
// ==================================================================

#[derive(Debug, PartialEq, CopyGetters)]
pub struct AAstNodeCallGet {
    #[getset(get_copy = "pub")]
    pub span: Span,

    // We still keep body here, because it is possible
    // to have some complex computation that derive data
    // access path segments. For example:
    //
    // .get(
    //    .get(@data, 0),
    //    .add(.get(@data, 0, 1, "index")))
    //
    // We still can infer types, but since the second
    // argument of the first .get call requires runtime
    // computation (because the access index is being
    // derived from the data itself), we need to preserve
    // function body in order to be able to derive that
    // index at runtime.
    // So during compilation, we basically check if
    // `data_binding_path` is not None, if it is not None,
    // then we use it, otherwise, we emit op codes for
    // runtime accessing.
    pub body: Vec<Box<AAstNode>>,

    // Binding path for type resolution only.
    // Does not include literal path segments like
    // Index(0) but rather abstract ones like
    // AbstractIndex.
    // Must not be optional because we can always
    // infer type since we know all data types
    // at compile time.
    pub type_binding_path: BindingPath,

    // Binding path for data accessing.
    // Does not include abstract path segments but
    // literal ones, like Index(0).
    //
    // It is wrapped in Option because there may be
    // a case when we are not able to build a literal
    // data access path because of runtime computation
    // that derives an access key.
    pub data_binding_path: Option<BindingPath>,

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

// ==================================================================
// CALL GET END
// ==================================================================

// ==================================================================
//
// CALL END
//
// ==================================================================
