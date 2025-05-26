use crate::utils::database::get_db_path;
use crate::utils::error_handler::print_info;
use libro_cli::db_operations::Database;
use libro_cli::errors::LibroResult;
use libro_cli::models::BookFilter;
use console::style;
use std::collections::HashMap;

/// Show authors summary
pub fn run(limit: u32) -> LibroResult<()> {
    let db = Database::new(&get_db_path())?;

    // Get all books
    let filter = BookFilter { id: None, year: None };
    let books = db.get_books(&filter)?;

    if books.is_empty() {
        print_info("No books found");
        return Ok(());
    }

    // Count books per author
    let mut author_stats: HashMap<String, (usize, f32, usize)> = HashMap::new(); // (book_count, avg_rating, total_reviews)

    for book in &books {
        for author in &book.authors {
            let entry = author_stats.entry(author.name.clone()).or_insert((0, 0.0, 0));
            entry.0 += 1; // book count

            if !book.reviews.is_empty() {
                let book_avg_rating = book.reviews.iter().map(|r| r.rating).sum::<i32>() as f32 / book.reviews.len() as f32;
                entry.1 = (entry.1 * (entry.0 - 1) as f32 + book_avg_rating) / entry.0 as f32; // running average
                entry.2 += book.reviews.len(); // total reviews
            }
        }
    }

    // Sort by book count (descending)
    let mut sorted_authors: Vec<_> = author_stats.into_iter().collect();
    sorted_authors.sort_by(|a, b| b.1.0.cmp(&a.1.0));
    sorted_authors.truncate(limit as usize);

    println!("{}", style(&format!("👥 Top {} Authors", sorted_authors.len())).bold().green());
    println!("{}", "═".repeat(50));

    for (i, (author_name, (book_count, avg_rating, review_count))) in sorted_authors.iter().enumerate() {
        println!("{}. {}",
            style(&format!("{:2}", i + 1)).dim(),
            style(author_name).bold()
        );

        print!("   📚 {} book{}", book_count, if *book_count == 1 { "" } else { "s" });

        if *review_count > 0 {
            print!(" • ⭐ {:.1}/5 ({} review{})",
                avg_rating,
                review_count,
                if *review_count == 1 { "" } else { "s" }
            );
        }

        println!();

        // Show some books by this author
        let author_books: Vec<_> = books.iter()
            .filter(|book| book.authors.iter().any(|a| a.name == *author_name))
            .take(3)
            .collect();

        for (j, book) in author_books.iter().enumerate() {
            let prefix = if j == author_books.len() - 1 && author_books.len() < *book_count { "└─" } else { "├─" };
            println!("   {} {}",
                style(prefix).dim(),
                style(&book.book.title).dim()
            );
        }

        if author_books.len() < *book_count {
            println!("   {} {} more...",
                style("└─").dim(),
                style(&format!("and {} ", book_count - author_books.len())).dim()
            );
        }

        if i < sorted_authors.len() - 1 {
            println!();
        }
    }

    Ok(())
}
