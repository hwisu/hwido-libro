use crate::utils::database::get_db_path;
use crate::utils::error_handler::print_info;
use libro_cli::db_operations::Database;
use libro_cli::errors::LibroResult;
use libro_cli::models::BookFilter;
use console::style;

/// Show latest books summary
pub fn run(limit: u32) -> LibroResult<()> {
    let db = Database::new(&get_db_path())?;

    // Get all books (we'll limit in display)
    let filter = BookFilter { id: None, year: None };
    let mut books = db.get_books(&filter)?;

    if books.is_empty() {
        print_info("No books found");
        return Ok(());
    }

    // Sort by ID (latest first) and limit
    books.sort_by(|a, b| b.book.id.cmp(&a.book.id));
    books.truncate(limit as usize);

    println!("{}", style(&format!("📚 Latest {} Books", books.len())).bold().green());
    println!("{}", "═".repeat(50));

    for (i, book) in books.iter().enumerate() {
        let authors: Vec<String> = book.authors.iter().map(|a| a.name.clone()).collect();

        println!("{}. {} {}",
            style(&format!("{:2}", i + 1)).dim(),
            style(&book.book.title).bold(),
            style(&format!("by {}", authors.join(", "))).dim()
        );

        if let Some(year) = book.book.pub_year {
            print!("   📅 {}", year);
        }

        if let Some(pages) = book.book.pages {
            print!(" • 📄 {} pages", pages);
        }

        if let Some(genre) = &book.book.genre {
            print!(" • 🏷️ {}", genre);
        }

        if !book.reviews.is_empty() {
            let avg_rating = book.reviews.iter().map(|r| r.rating).sum::<i32>() as f32 / book.reviews.len() as f32;
            print!(" • ⭐ {:.1}/5 ({} reviews)", avg_rating, book.reviews.len());
        }

        println!();

        if i < books.len() - 1 {
            println!();
        }
    }

    Ok(())
}
