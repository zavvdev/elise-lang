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

use elise_shared_proc_macros::EnumDelegate;
use elise_shared_types::Span;

use crate::{
    aast_node_call::AAstNodeCall, aast_node_data::AAstNodeData, aast_node_slot::AAstNodeSlot,
};

pub mod aast_node_call;
pub mod aast_node_data;
pub mod aast_node_slot;

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
