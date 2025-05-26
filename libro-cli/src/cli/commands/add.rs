use crate::lib::db_operations::Database;
use crate::lib::errors::LibroResult;
use crate::utils::database::get_db_path;
use crate::utils::error_handler::{print_info, print_success};
use crate::utils::input::prompt_for_book_with_review;

/// Add a new book
pub fn run() -> LibroResult<()> {
    print_info("Adding a new book to your library");

    // Initialize database connection
    let mut db = Database::new(&get_db_path())?;

    // Get book information from user
    let book_with_review = prompt_for_book_with_review()?;

    // Save to database
    let result = db.add_book_with_review(&book_with_review)?;

    // Print success message
    if let Some(review_id) = result.review_id {
        print_success(&format!(
            "Book added successfully! Book ID: {}, Review ID: {}",
            result.book_id, review_id
        ));
    } else {
        print_success(&format!(
            "Book added successfully! Book ID: {}",
            result.book_id
        ));
    }

    Ok(())
}
