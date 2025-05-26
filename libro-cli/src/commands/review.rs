use crate::utils::database::get_db_path;
use crate::utils::error_handler::{print_info, print_success, validation::validate_book_id};
use crate::utils::input::{confirm, prompt_for_review};
use libro_cli::db_operations::Database;
use libro_cli::errors::LibroResult;
use libro_cli::models::BookFilter;

/// Add or edit a review for a book
pub fn run(id: u32) -> LibroResult<()> {
    // Validate book ID
    let book_id = validate_book_id(Some(id))?;

    // Initialize database connection
    let mut db = Database::new(&get_db_path())?;

    // Check if book exists
    let filter = BookFilter {
        id: Some(book_id),
        year: None,
    };
    let books = db.get_books(&filter)?;

    if books.is_empty() {
        return Err(libro_cli::errors::LibroError::BookNotFound { id: book_id });
    }

    let book = &books[0];
    let book_title = &book.book.title;

    print_info(&format!("Managing review for: '{}'", book_title));

    // Check if review already exists
    if !book.reviews.is_empty() {
        println!("This book already has {} review(s):", book.reviews.len());
        for (i, review) in book.reviews.iter().enumerate() {
            println!(
                "  {}. Rating: {}/5 - {}",
                i + 1,
                review.rating,
                if review.review.len() > 50 {
                    format!("{}...", &review.review[..50])
                } else {
                    review.review.clone()
                }
            );
        }

        if !confirm("Would you like to add another review?")? {
            return Ok(());
        }
    }

    // Get review information from user
    let new_review = prompt_for_review(book_id, book_title)?;

    // Save review to database
    let review_id = db.add_review(&new_review)?;

    print_success(&format!(
        "Review added successfully! Review ID: {}",
        review_id
    ));

    Ok(())
}
