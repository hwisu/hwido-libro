use thiserror::Error;

/// Main error type for the libro-cli application
#[derive(Error, Debug)]
pub enum LibroError {
    #[error("Database error: {0}")]
    Database(#[from] rusqlite::Error),

    #[error("Validation error: {message}")]
    Validation { message: String },

    #[error("Book not found with ID: {id}")]
    BookNotFound { id: i64 },

    #[error("Review not found for book ID: {book_id}")]
    ReviewNotFound { book_id: i64 },

    #[error("Writer not found: {name}")]
    WriterNotFound { name: String },

    #[error("Invalid input: {message}")]
    InvalidInput { message: String },

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Date parsing error: {0}")]
    DateParse(#[from] chrono::ParseError),

    #[error("JSON serialization error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("User cancelled operation")]
    UserCancelled,

    #[error("Internal error: {message}")]
    Internal { message: String },
}

/// Result type alias for convenience
pub type LibroResult<T> = Result<T, LibroError>;

impl LibroError {
    /// Create a validation error
    pub fn validation(message: impl Into<String>) -> Self {
        LibroError::Validation {
            message: message.into(),
        }
    }

    /// Create an invalid input error
    pub fn invalid_input(message: impl Into<String>) -> Self {
        LibroError::InvalidInput {
            message: message.into(),
        }
    }

    /// Create an internal error
    pub fn internal(message: impl Into<String>) -> Self {
        LibroError::Internal {
            message: message.into(),
        }
    }

    /// Check if this error is user-recoverable
    pub fn is_user_error(&self) -> bool {
        matches!(
            self,
            LibroError::Validation { .. }
                | LibroError::InvalidInput { .. }
                | LibroError::BookNotFound { .. }
                | LibroError::ReviewNotFound { .. }
                | LibroError::WriterNotFound { .. }
                | LibroError::UserCancelled
        )
    }
}

/// Validation helper functions
pub mod validation {
    use super::LibroError;
    use chrono::Datelike;

    /// Validate that a string is not empty
    pub fn validate_non_empty(value: &str, field_name: &str) -> Result<(), LibroError> {
        if value.trim().is_empty() {
            Err(LibroError::validation(format!(
                "{} cannot be empty",
                field_name
            )))
        } else {
            Ok(())
        }
    }

    /// Validate rating is within valid range (1-5)
    pub fn validate_rating(rating: i32) -> Result<(), LibroError> {
        if !(1..=5).contains(&rating) {
            Err(LibroError::validation(
                "Rating must be between 1 and 5".to_string(),
            ))
        } else {
            Ok(())
        }
    }

    /// Validate year is reasonable
    pub fn validate_year(year: i32) -> Result<(), LibroError> {
        let current_year = chrono::Utc::now().year();
        if year < 1000 || year > current_year + 10 {
            Err(LibroError::validation(format!(
                "Year must be between 1000 and {}",
                current_year + 10
            )))
        } else {
            Ok(())
        }
    }

    /// Validate pages is positive
    pub fn validate_pages(pages: i32) -> Result<(), LibroError> {
        if pages <= 0 {
            Err(LibroError::validation(
                "Pages must be a positive number".to_string(),
            ))
        } else {
            Ok(())
        }
    }
}

// Additional From implementations for external crates
impl From<dialoguer::Error> for LibroError {
    fn from(error: dialoguer::Error) -> Self {
        match error {
            dialoguer::Error::IO(io_error) => LibroError::Io(io_error),
            #[allow(unreachable_patterns)]
            _ => LibroError::UserCancelled,
        }
    }
}

impl From<std::num::ParseIntError> for LibroError {
    fn from(error: std::num::ParseIntError) -> Self {
        LibroError::invalid_input(format!("Invalid number: {}", error))
    }
}
