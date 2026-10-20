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
use elise_shared_types::Span;

#[derive(Debug, PartialEq)]
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
    SlotData { span: Span },
}

// ==================================================================
//
// CALL START
//
// AAst node descriptors for all Call variants.
//
// ==================================================================

#[derive(Debug, PartialEq)]
pub enum AAstNodeCall {
    Typedef {
        // Raw alias for a new type created with
        // .typedef function.
        alias: String,
        annot: AAstNodeTypeAnnot,
    },
    Get {
        span: Span,

        // Binding path for type resolution only.
        // Does not include literal path segments like
        // Index(0) but rather abstract ones like
        // AbstractIndex.
        type_binding_path: BindingPath,

        // Binding path for data accessing.
        // Does not include abstract path segments but
        // literal ones, like Index(0).
        //
        // NOTE: Not 100% sure if we need this here,
        // but our compiler will probably need to know
        // about data access path in order to generate
        // op code.
        data_binding_path: BindingPath,

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
        return_type_binding: TypeBinding,
    },
}

// ==================================================================
//
// CALL END
//
// ==================================================================

// ==================================================================
//
// DATA START
//
// ==================================================================

#[derive(Debug, PartialEq)]
pub struct AAstNodeDataPrim {
    pub lexeme: String,
    pub span: Span,
}

#[derive(Debug, PartialEq)]
pub struct AAstNodeDataDictKey {
    pub span: Span,
    pub lexeme: String,
}

#[derive(Debug, PartialEq)]
pub enum AAstNodeData {
    Int(AAstNodeDataPrim),
    Str(AAstNodeDataPrim),
    List {
        span: Span,
        items: Vec<Box<AAstNodeData>>,
    },
    Dict {
        span: Span,
        entries: Vec<(AAstNodeDataDictKey, Box<AAstNodeData>)>,
    },
}

// ==================================================================
//
// DATA END
//
// ==================================================================

// ==================================================================
//
// TYPE ANNOTATIONS START
//
// ==================================================================

pub type AAstNodeTypeAnnotRecordEntries = Vec<(String, Box<AAstNodeTypeAnnot>)>;

#[derive(Debug, PartialEq, Clone)]
pub enum AAstNodeTypeAnnot {
    // Represents a reference to a custom data type.
    // For example:
    // .typedef (User :UserShape)
    // where :UserShape is a reference to the custom
    // data type.
    Custom {
        span: Span,
        alias: String,
    },
    Record {
        span: Span,
        entries: AAstNodeTypeAnnotRecordEntries,
    },
    List {
        span: Span,
        item_type: Box<AAstNodeTypeAnnot>,
    },
    Int {
        span: Span,
    },
}

// ==================================================================
//
// TYPE ANNOTATIONS END
//
// ==================================================================
