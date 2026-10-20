use elise_shared_proc_macros::EnumDelegate;

#[derive(EnumDelegate)]
#[enum_delegate(
    fn span(&self) -> Span;
)]
pub enum Some {
    A(VariantA),
    B(VariantB),
}

#[derive(Clone)]
struct Span;

pub struct VariantA {
    span: Span,
}
impl VariantA {
    fn span(&self) -> Span {
        self.span.clone()
    }
}

pub struct VariantB {
    span: Span,
}
impl VariantB {
    fn span(&self) -> Span {
        self.span.clone()
    }
}
