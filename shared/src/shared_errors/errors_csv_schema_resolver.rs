use crate::shared_types::Span;

#[derive(Debug, PartialEq)]
pub enum CsvSchemaResolverErr {
    RootInval { span: Span },
    RootArgsLen { span: Span },

    RowInval { span: Span },
    RowArgsLen { span: Span },

    ColInvalName { span: Span },
    ColInvalType { span: Span },
    ColTypeNoArgs { span: Span },

    OptArgsLen { span: Span },

    OptEmpty { span: Span },
}
