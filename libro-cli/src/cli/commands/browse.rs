use crate::lib::db_operations::Database;
use crate::lib::errors::LibroResult;
use crate::lib::models::BookFilter;
use crate::utils::database::get_db_path;
use crate::utils::error_handler::{print_info, validation::validate_year_option};
use crate::utils::output::{Displayable, OutputFormat};
use console::style;

/// Browse and search books
pub fn run(query: Option<String>, year: Option<u32>, json: bool) -> LibroResult<()> {
    let db = Database::new(&get_db_path())?;

    // Validate year input
    let filter_year = validate_year_option(year)?;

    // Create filter
    let filter = BookFilter {
        id: None,
        year: filter_year,
    };

    // Query books
    let mut books = db.get_books(&filter)?;

    // Apply text search if query provided
    if let Some(search_query) = &query {
        let search_lower = search_query.to_lowercase();
        books.retain(|book| {
            // Search in title
            book.book.title.to_lowercase().contains(&search_lower) ||
            // Search in authors
            book.authors.iter().any(|author| author.name.to_lowercase().contains(&search_lower)) ||
            // Search in genre
            book.book.genre.to_lowercase().contains(&search_lower)
        });
    }

    if books.is_empty() {
        if let Some(q) = &query {
            print_info(&format!("No books found matching '{}'", q));
        } else {
            print_info("No books found");
        }
        return Ok(());
    }

    // Show search info
    if let Some(q) = &query {
        println!(
            "{}",
            style(&format!("🔍 Search results for '{}'", q))
                .bold()
                .cyan()
        );
        println!("{}", "─".repeat(50));
    } else {
        println!("{}", style("📚 All Books").bold().cyan());
        println!("{}", "─".repeat(50));
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

    // Show summary
    if !json && books.len() > 1 {
        println!(
            "\n{}",
            style(&format!("Found {} book(s)", books.len())).dim()
        );
        if query.is_some() {
            println!(
                "{}",
                style("💡 Tip: Use 'libro-cli browse' without arguments to see all books").dim()
            );
        }
    }

    Ok(())
}
