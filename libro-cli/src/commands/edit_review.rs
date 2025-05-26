use crate::utils::database::get_db_path;
use crate::utils::error_handler::{print_info, print_success, validation::validate_book_id};
use crate::utils::input::{prompt_edit_review, select_from_list};
use libro_cli::db_operations::Database;
use libro_cli::errors::LibroResult;
use libro_cli::models::BookFilter;

/// Edit a book review using system editor
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

    print_info(&format!("Editing review for: '{}'", book_title));

    // Check if reviews exist
    if book.reviews.is_empty() {
        return Err(libro_cli::errors::LibroError::ReviewNotFound { book_id });
    }

    // If multiple reviews, let user select which one to edit
    let review_to_edit = if book.reviews.len() == 1 {
        &book.reviews[0]
    } else {
        println!(
            "This book has {} reviews. Which one would you like to edit?",
            book.reviews.len()
        );

        let review_options: Vec<String> = book
            .reviews
            .iter()
            .enumerate()
            .map(|(i, review)| {
                format!(
                    "{}. Rating: {}/5 - {}",
                    i + 1,
                    review.rating,
                    if review.review.len() > 50 {
                        format!("{}...", &review.review[..50])
                    } else {
                        review.review.clone()
                    }
                )
            })
            .collect();

        let selection = select_from_list("Select review to edit:", &review_options)?;
        &book.reviews[selection]
    };

    // Get the review ID
    let review_id = review_to_edit
        .id
        .ok_or_else(|| libro_cli::errors::LibroError::internal("Review missing ID"))?;

    // Edit the review
    let updated_review = prompt_edit_review(review_to_edit, book_title)?;

    // Save changes to database
    db.update_review(review_id, &updated_review)?;

    print_success("Review updated successfully!");

    Ok(())
}
