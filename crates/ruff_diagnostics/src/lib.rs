pub use diagnostic::{Diagnostic, DiagnosticKind};
pub use edit::Edit;
pub use fix::{Applicability, Fix};
pub use violation::{AlwaysAutofixableViolation, AutofixKind, Violation};

mod diagnostic;
mod edit;
mod fix;
mod violation;
