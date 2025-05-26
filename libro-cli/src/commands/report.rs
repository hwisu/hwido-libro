use crate::utils::database::get_db_path;
use crate::utils::error_handler::{print_info, validation::validate_year_option};
use crate::utils::output::{format_author_stats, format_reading_stats};
use libro_cli::db_operations::Database;
use libro_cli::errors::LibroResult;
use libro_cli::models::BookFilter;
use chrono::Datelike;

/// Generate reading reports
pub fn run(author: bool, year: Option<u32>, years: bool) -> LibroResult<()> {
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
    if author {
        // Author statistics
        let author_stats = format_author_stats(&books);
        println!("{}", author_stats);
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
