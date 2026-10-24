use elise_shared_types::Span;
use getset::CopyGetters;

#[derive(Debug, PartialEq, CopyGetters)]
pub struct AAstNodeSlot {
    #[getset(get_copy = "pub")]
    pub span: Span,
}
