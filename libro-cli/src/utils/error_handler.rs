use console::style;
use std::process;

use libro_cli::errors::{LibroError, LibroResult};

/// Handle errors at the CLI level with user-friendly messages
pub fn handle_cli_error(error: LibroError) -> ! {
    match error {
        LibroError::Validation { message } => {
            eprintln!("{} {}", style("❌ Validation Error:").bold().red(), message);
            process::exit(1);
        }
        LibroError::BookNotFound { id } => {
            eprintln!(
                "{} Book with ID {} not found",
                style("❌ Error:").bold().red(),
                id
            );
            process::exit(1);
        }
        LibroError::ReviewNotFound { book_id } => {
            eprintln!(
                "{} No review found for book ID {}",
                style("❌ Error:").bold().red(),
                book_id
            );
            process::exit(1);
        }
        LibroError::WriterNotFound { name } => {
            eprintln!(
                "{} Writer '{}' not found",
                style("❌ Error:").bold().red(),
                name
            );
            process::exit(1);
        }
        LibroError::InvalidInput { message } => {
            eprintln!("{} {}", style("❌ Invalid Input:").bold().red(), message);
            eprintln!(
                "{} Please check your input and try again.",
                style("💡 Hint:").bold().yellow()
            );
            process::exit(1);
        }
        LibroError::Database(db_error) => {
            eprintln!(
                "{} Database error occurred",
                style("❌ Database Error:").bold().red()
            );

            // Check for common database errors
            let error_msg = db_error.to_string();
            if error_msg.contains("FOREIGN KEY constraint failed") {
                eprintln!(
                    "{} Cannot delete this item because it's referenced by other data.",
                    style("💡 Hint:").bold().yellow()
                );
                eprintln!("   Try deleting related items first (e.g., reviews before books).");
            } else if error_msg.contains("UNIQUE constraint failed") {
                eprintln!(
                    "{} This item already exists in the database.",
                    style("💡 Hint:").bold().yellow()
                );
            } else {
                eprintln!("{} {}", style("Details:").bold().white(), error_msg);
            }
            process::exit(1);
        }
        LibroError::Io(io_error) => {
            eprintln!(
                "{} File system error: {}",
                style("❌ IO Error:").bold().red(),
                io_error
            );
            process::exit(1);
        }
        LibroError::DateParse(date_error) => {
            eprintln!(
                "{} Invalid date format: {}",
                style("❌ Date Error:").bold().red(),
                date_error
            );
            eprintln!(
                "{} Use format YYYY-MM-DD (e.g., 2023-12-01)",
                style("💡 Hint:").bold().yellow()
            );
            process::exit(1);
        }
        LibroError::Json(json_error) => {
            eprintln!(
                "{} JSON processing error: {}",
                style("❌ JSON Error:").bold().red(),
                json_error
            );
            process::exit(1);
        }
        LibroError::UserCancelled => {
            eprintln!(
                "{} Operation cancelled by user",
                style("ℹ️  Info:").bold().blue()
            );
            process::exit(0);
        }
        LibroError::Internal { message } => {
            eprintln!(
                "{} Internal error: {}",
                style("❌ Internal Error:").bold().red(),
                message
            );
            eprintln!(
                "{} This is likely a bug. Please report it.",
                style("💡 Hint:").bold().yellow()
            );
            process::exit(1);
        }
    }
}

/// Handle results with automatic error handling
pub fn handle_result<T>(result: LibroResult<T>) -> T {
    match result {
        Ok(value) => value,
        Err(error) => handle_cli_error(error),
    }
}

/// Print a success message
pub fn print_success(message: &str) {
    println!("{} {}", style("✅ Success:").bold().green(), message);
}

/// Print an info message
pub fn print_info(message: &str) {
    println!("{} {}", style("ℹ️  Info:").bold().blue(), message);
}

/// Print a warning message
#[allow(dead_code)]
pub fn print_warning(message: &str) {
    println!("{} {}", style("⚠️  Warning:").bold().yellow(), message);
}

/// Print a debug message (only in debug mode)
#[allow(dead_code)]
pub fn print_debug(message: &str) {
    if cfg!(debug_assertions) {
        println!("{} {}", style("🐛 Debug:").bold().magenta(), message);
    }
}

/// Wrap a function call with error handling
#[allow(dead_code)]
pub fn with_error_handling<F, T>(f: F) -> T
where
    F: FnOnce() -> LibroResult<T>,
{
    handle_result(f())
}

/// Validate and handle common input scenarios
pub mod validation {
    use super::*;

    /// Validate that a book ID is provided and valid
    pub fn validate_book_id(id: Option<u32>) -> LibroResult<i64> {
        match id {
            Some(id) if id > 0 => Ok(id as i64),
            Some(_) => Err(LibroError::invalid_input("Book ID must be greater than 0")),
            None => Err(LibroError::invalid_input("Book ID is required")),
        }
    }

    /// Validate that a year is reasonable
    pub fn validate_year_option(year: Option<u32>) -> LibroResult<Option<i32>> {
        match year {
            Some(year) => {
                let year = year as i32;
                libro_cli::errors::validation::validate_year(year)?;
                Ok(Some(year))
            }
            None => Ok(None),
        }
    }

    /// Ensure database file exists or can be created
    #[allow(dead_code)]
    pub fn ensure_database_accessible(db_path: &str) -> LibroResult<()> {
        use std::path::Path;

        let path = Path::new(db_path);

        // If file doesn't exist, check if parent directory is writable
        if !path.exists() {
            if let Some(parent) = path.parent() {
                if !parent.exists() {
                    return Err(LibroError::invalid_input(format!(
                        "Directory '{}' does not exist",
                        parent.display()
                    )));
                }

                // Try to create a temporary file to test write permissions
                let temp_file = parent.join(".libro_test");
                if std::fs::write(&temp_file, "test").is_err() {
                    return Err(LibroError::invalid_input(format!(
                        "No write permission in directory '{}'",
                        parent.display()
                    )));
                }
                let _ = std::fs::remove_file(&temp_file);
            }
        } else {
            // File exists, check if it's readable
            if std::fs::metadata(path).is_err() {
                return Err(LibroError::invalid_input(format!(
                    "Cannot access database file '{}'",
                    path.display()
                )));
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_book_id() {
        assert!(validation::validate_book_id(Some(1)).is_ok());
        assert!(validation::validate_book_id(Some(0)).is_err());
        assert!(validation::validate_book_id(None).is_err());
    }

    #[test]
    fn test_validate_year_option() {
        assert!(validation::validate_year_option(Some(2023)).is_ok());
        assert!(validation::validate_year_option(None).is_ok());
        assert!(validation::validate_year_option(Some(500)).is_err());
    }

    #[test]
    fn test_print_functions() {
        // These functions should not panic
        print_success("Test success");
        print_info("Test info");
        print_warning("Test warning");
        print_debug("Test debug");
    }
}
