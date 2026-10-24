pub mod call_get;
pub mod call_typedef;
pub mod type_annot;

use elise_shared_proc_macros::EnumDelegate;
use elise_shared_types::Span;

use crate::aast_node_call::{call_get::AAstNodeCallGet, call_typedef::AAstNodeCallTypedef};

#[derive(Debug, PartialEq, EnumDelegate)]
#[enum_delegate(
    fn span(&self) -> Span;
)]
pub enum AAstNodeCall {
    Typedef(AAstNodeCallTypedef),
    Get(AAstNodeCallGet),
}
