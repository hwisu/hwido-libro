use crate::utils::database::get_db_path;
use crate::utils::error_handler::{print_info, validation::validate_year_option};
use crate::utils::output::{format_reading_stats};
use crate::utils::date::{format_date, relative_date_description};
use libro_cli::db_operations::Database;
use libro_cli::errors::LibroResult;
use libro_cli::models::BookFilter;
use chrono::Datelike;
use console::style;
use std::collections::HashMap;

/// Generate reading reports and summaries
pub fn run(show_authors: bool, show_books: bool, show_reviews: bool, year: Option<u32>, years: bool, limit: u32) -> LibroResult<()> {
    // Initialize database connection
    let db = Database::new(&get_db_path())?;

    // Validate year input
    let filter_year = validate_year_option(year)?;

    // Create filter
    let filter = BookFilter {
        id: None,
        year: filter_year,
    };

    // Query books
    let books = db.get_books(&filter)?;

    if books.is_empty() {
        print_info("No books found for generating reports");
        return Ok(());
    }

    // Generate reports based on flags
    if show_authors {
        // Author statistics
        show_authors_summary(&books, limit);
    } else if show_books {
        // Latest books
        show_books_summary(&books, limit);
    } else if show_reviews {
        // Latest reviews
        show_reviews_summary(&books, limit);
    } else if years {
        // Years chart - show reading stats with year breakdown
        let reading_stats = format_reading_stats(&books);
        println!("{}", reading_stats);

        // Additional year-by-year breakdown based on reading dates
        println!("\n📅 Year-by-Year Reading Chart (by read date):");
        println!("{}", "═".repeat(50));

        let mut year_counts = std::collections::HashMap::new();
        for book in &books {
            for review in &book.reviews {
                if let Some(date_read) = review.date_read {
                    let read_year = date_read.year();
                    *year_counts.entry(read_year).or_insert(0) += 1;
                }
            }
        }

        if year_counts.is_empty() {
            println!("No reading dates available for chart generation.");
        } else {
            let mut years: Vec<_> = year_counts.iter().collect();
            years.sort_by_key(|(year, _)| *year);

            for (year, count) in years {
                let bar = "█".repeat(*count);
                println!("{}: {} ({} book{})", year, bar, count, if *count == 1 { "" } else { "s" });
            }
        }
    } else {
        // Default: general reading statistics
        let reading_stats = format_reading_stats(&books);
        println!("{}", reading_stats);
    }

    Ok(())
}

/// Show latest books summary
fn show_books_summary(books: &[libro_cli::models::ExtendedBook], limit: u32) {
    let mut sorted_books = books.to_vec();
    sorted_books.sort_by(|a, b| b.book.id.cmp(&a.book.id));
    sorted_books.truncate(limit as usize);

    println!("{}", style(&format!("📚 Latest {} Books", sorted_books.len())).bold().green());
    println!("{}", "═".repeat(50));

    for (i, book) in sorted_books.iter().enumerate() {
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

        print!(" • 🏷️ {}", &book.book.genre);

        println!();

        if i < sorted_books.len() - 1 {
            println!();
        }
    }
}

/// Show latest reviews summary
fn show_reviews_summary(books: &[libro_cli::models::ExtendedBook], limit: u32) {
    let mut reviews_with_books = Vec::new();
    for book in books {
        for review in &book.reviews {
            reviews_with_books.push((review, book));
        }
    }

    if reviews_with_books.is_empty() {
        print_info("No reviews found");
        return;
    }

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

        let review_lines: Vec<&str> = review.review.lines().collect();
        if !review_lines.is_empty() {
            let first_line = review_lines[0];
            if first_line.chars().count() > 80 {
                let truncated: String = first_line.chars().take(77).collect();
                println!("   \"{}...\"", truncated);
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
}

/// Show authors summary
fn show_authors_summary(books: &[libro_cli::models::ExtendedBook], limit: u32) {
    let mut author_stats: HashMap<String, usize> = HashMap::new();

    for book in books {
        for author in &book.authors {
            *author_stats.entry(author.name.clone()).or_insert(0) += 1;
        }
    }

    let mut sorted_authors: Vec<_> = author_stats.into_iter().collect();
    sorted_authors.sort_by(|a, b| b.1.cmp(&a.1));
    sorted_authors.truncate(limit as usize);

    println!("{}", style(&format!("👥 Top {} Authors", sorted_authors.len())).bold().green());
    println!("{}", "═".repeat(50));

    for (i, (author_name, book_count)) in sorted_authors.iter().enumerate() {
        println!("{}. {}",
            style(&format!("{:2}", i + 1)).dim(),
            style(author_name).bold()
        );

        println!("   📚 {} book{}", book_count, if *book_count == 1 { "" } else { "s" });

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
}
