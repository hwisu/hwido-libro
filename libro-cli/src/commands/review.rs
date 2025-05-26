use crate::utils::database::get_db_path;
use crate::utils::error_handler::{print_info, print_success, validation::validate_book_id};
use crate::utils::input::{prompt_for_review, prompt_edit_review, select_from_list};
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

        // Ask user what they want to do
        let options = vec![
            "Add a new review",
            "Edit an existing review",
            "Cancel"
        ];

        let choice = select_from_list("What would you like to do?", &options)?;

        match choice {
            0 => {
                // Add new review
                let new_review = prompt_for_review(book_id, book_title)?;
                let review_id = db.add_review(&new_review)?;
                print_success(&format!("Review added successfully! Review ID: {}", review_id));
            }
            1 => {
                // Edit existing review
                let review_to_edit = if book.reviews.len() == 1 {
                    &book.reviews[0]
                } else {
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

                let review_id = review_to_edit
                    .id
                    .ok_or_else(|| libro_cli::errors::LibroError::internal("Review missing ID"))?;

                let updated_review = prompt_edit_review(review_to_edit, book_title)?;
                db.update_review(review_id, &updated_review)?;
                print_success("Review updated successfully!");
            }
            _ => {
                print_info("Operation cancelled");
                return Ok(());
            }
        }
    } else {
        // No existing reviews, add new one
        let new_review = prompt_for_review(book_id, book_title)?;
        let review_id = db.add_review(&new_review)?;
        print_success(&format!("Review added successfully! Review ID: {}", review_id));
    }

    Ok(())
}
