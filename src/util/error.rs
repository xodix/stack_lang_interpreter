#[derive(thiserror::Error, Debug)]
pub enum ParsingError {
    #[error("\tFailed to extract {what}.\n\t{reason}")]
    ExtractionError { what: String, reason: String },
    #[error("\tFailed to register {what}.\n\t{reason}")]
    RegistrationError { what: String, reason: String },
    #[error("\t{reason}")]
    KeywordError { reason: String },
    #[error("Parsing Error\n\tExpected {expected} but got {got}.")]
    MismatchedTypes { expected: String, got: String },
}

#[derive(thiserror::Error, Debug)]
pub enum RuntimeError {
    #[error("\tInvalid operation `{operation:?}`.")]
    InvalidOperation { operation: crate::OperationType },
    // TODO one argument not one arguments
    #[error("\tExpected {needed} argument(s) got {got}.\n\tArguments: {value_stack:?}")]
    InsufficientArguments {
        needed: usize,
        got: usize,
        value_stack: Vec<crate::ValueType>,
    },
    #[error("\tExpected {expected} but got {got}.")]
    MismatchedTypes { expected: String, got: String },
}

pub mod parsing {
    pub type Result<T> = std::result::Result<T, super::ParsingError>;
}

pub mod runtime {
    pub type Result<T> = std::result::Result<T, super::RuntimeError>;
}
