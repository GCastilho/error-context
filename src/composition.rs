/// Allows for converting from a child error type to a parent
/// error type while preserving any context on the child error.
///
/// This is intended to be used when:
///   1. Both Source and Target are context enriched thiserror enums
///   2. Source is a variant of Target's inner error
///
/// ** Example **
/// ```ignore
/// // Some inner error type
/// #[derive(Debug, Error)]
/// pub enum InnerError {
///     #[error("dummy")]
///     Dummy,
/// }
/// impl_context(Inner(InnerError));
///
/// // And some outer error type, which contains
/// // a variant of the inner error type
/// #[derive(Debug, Error)]
/// pub enum OuterError {
///     #[error("inner error")]
///     // we explicitly do _not_ use #[from] here, instead
///     // opting to use the macro to create the conversion
///     // and handle the context propagation.
///     Inner(InnerError),
/// }
/// impl_context(Outer(OuterError));
///
/// // Then we use the macro to implement the conversion
/// // from Inner to Outer
/// impl_from_carry_context!(Inner, Outer, OuterError::Inner);
/// ```
#[macro_export]
macro_rules! impl_from_carry_context {
    ($source: ident, $target: ident, $variant: path) => {
        impl From<$source> for $target {
            fn from(mut value: $source) -> Self {
                let mut contexts = vec![];

                let inner = loop {
                    match value {
                        $source::Base(x) => break x,
                        $source::Context { context, error } => {
                            contexts.push(context);
                            value = *error;
                        }
                    }
                };
                let inner = $source::Base(inner);

                let mut x = $target::Base($variant(inner));

                for ctx in contexts.into_iter().rev() {
                    x = $target::Context {
                        context: ctx,
                        error: Box::new(x),
                    };
                }

                x
            }
        }
    };
}
