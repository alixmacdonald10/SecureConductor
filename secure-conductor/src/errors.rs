use miette::Diagnostic;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, SecureConductorError>;

#[derive(Diagnostic, Error, Debug, Clone)]
pub enum SecureConductorError {

    #[diagnostic(code(secure_conductor::secure_conductor_feature_not_implemented), help("The feature `{0}` is not available on this version of secure_conductor ({1})"))]
    #[error("Feature is not available on this version of secure_conductor\n{2}:{3}")]
    SecureConductorFeatureNotImplemented(String, String, String, u32),

    #[diagnostic(code(secure_conductor::missing_required_argument), help("This version of secure_conductor ({0}) requires the argument `{1}` to be specified on startup or in the environment variables"))]
    #[error("There are missing required arguments\n{2}:{3}")]
    MissingRequiredArgument(String, String, String, u32),

    #[diagnostic(code(secure_conductor::tracing_error), help("{0}"))]
    #[error("There was an Error when setting up tracing\n{1}:{2}")]
    TracingError(String, String, u32),

    #[diagnostic(code(secure_conductor::pseudo_panic), help("There was a generic error:\n{0}"))]
    #[error("Pseudo panic")]
    PseudoPanic (String, String, u32),
}
