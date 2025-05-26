use crate::utils::database::get_db_path;
use crate::utils::error_handler::{
    print_info, validation::validate_book_id, validation::validate_year_option,
};
use crate::utils::output::{Displayable, OutputFormat};
use libro_cli::db_operations::Database;
use libro_cli::errors::LibroResult;
use libro_cli::models::BookFilter;

/// Show book(s) with reviews by id or year
pub fn run(id: Option<u32>, year: Option<u32>, json: bool) -> LibroResult<()> {
    // Initialize database connection
    let db = Database::new(&get_db_path())?;

    // Validate inputs
    let book_id = if let Some(id) = id {
        Some(validate_book_id(Some(id))?)
    } else {
        None
    };

    let filter_year = validate_year_option(year)?;

    // Create filter
    let filter = BookFilter {
        id: book_id,
        year: filter_year,
    };

    // Query books
    let books = db.get_books(&filter)?;

    if books.is_empty() {
        print_info("No books found matching the criteria");
        return Ok(());
    }

    // Determine output format
    let format = if json {
        OutputFormat::Json
    } else if books.len() == 1 {
        OutputFormat::Table
    } else {
        OutputFormat::Summary
    };

    // Display results
    let output = books.display(&format)?;
    println!("{}", output);

    Ok(())
}
