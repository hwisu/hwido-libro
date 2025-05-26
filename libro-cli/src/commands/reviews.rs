use crate::utils::database::get_db_path;
use crate::utils::error_handler::print_info;
use crate::utils::date::{format_date, relative_date_description};
use libro_cli::db_operations::Database;
use libro_cli::errors::LibroResult;
use libro_cli::models::BookFilter;
use console::style;

/// Show latest reviews summary
pub fn run(limit: u32) -> LibroResult<()> {
    let db = Database::new(&get_db_path())?;

    // Get all books to collect reviews
    let filter = BookFilter { id: None, year: None };
    let books = db.get_books(&filter)?;

    // Collect all reviews with book info
    let mut reviews_with_books = Vec::new();
    for book in &books {
        for review in &book.reviews {
            reviews_with_books.push((review, book));
        }
    }

    if reviews_with_books.is_empty() {
        print_info("No reviews found");
        return Ok(());
    }

    // Sort by review ID (latest first) and limit
    reviews_with_books.sort_by(|a, b| b.0.id.cmp(&a.0.id));
    reviews_with_books.truncate(limit as usize);

    println!("{}", style(&format!("📝 Latest {} Reviews", reviews_with_books.len())).bold().green());
    println!("{}", "═".repeat(50));

    for (i, (review, book)) in reviews_with_books.iter().enumerate() {
        let authors: Vec<String> = book.authors.iter().map(|a| a.name.clone()).collect();
        let stars = "⭐".repeat(review.rating as usize);

        println!("{}. {} {}/5 - {}",
            style(&format!("{:2}", i + 1)).dim(),
            style(&stars).yellow(),
            review.rating,
            style(&book.book.title).bold()
        );

        println!("   {} {}",
            style("by").dim(),
            style(&authors.join(", ")).dim()
        );

        if let Some(date) = review.date_read {
            println!("   {} {} ({})",
                style("📅").dim(),
                format_date(&date),
                style(&relative_date_description(&date)).dim()
            );
        }

        // Show review text (truncated)
        let review_lines: Vec<&str> = review.review.lines().collect();
        if !review_lines.is_empty() {
            let first_line = review_lines[0];
            if first_line.len() > 80 {
                println!("   \"{}...\"", &first_line[..77]);
            } else {
                println!("   \"{}\"", first_line);
            }

            if review_lines.len() > 1 {
                println!("   {}", style("(continued...)").dim());
            }
        }

        if i < reviews_with_books.len() - 1 {
            println!();
        }
    }

    Ok(())
}
