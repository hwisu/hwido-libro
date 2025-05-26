use crate::utils::date::{format_date, relative_date_description};
use console::style;
use libro_cli::errors::LibroResult;
use libro_cli::models::*;
use serde_json;
use tabled::{Table, Tabled};
use chrono::Datelike;

/// Display format options
#[derive(Debug, Clone)]
pub enum OutputFormat {
    Table,
    Json,
    Summary,
}

/// Trait for objects that can be displayed in different formats
pub trait Displayable {
    fn display(&self, format: &OutputFormat) -> LibroResult<String>;
}

/// Extended book display implementation
impl Displayable for ExtendedBook {
    fn display(&self, format: &OutputFormat) -> LibroResult<String> {
        match format {
            OutputFormat::Json => Ok(serde_json::to_string_pretty(self)?),
            OutputFormat::Table => Ok(format_book_table(self)),
            OutputFormat::Summary => Ok(format_book_summary(self)),
        }
    }
}

/// Display a list of books
impl Displayable for Vec<ExtendedBook> {
    fn display(&self, format: &OutputFormat) -> LibroResult<String> {
        match format {
            OutputFormat::Json => Ok(serde_json::to_string_pretty(self)?),
            OutputFormat::Table => Ok(format_books_table(self)),
            OutputFormat::Summary => Ok(format_books_summary(self)),
        }
    }
}

/// Format a single book as a detailed table
fn format_book_table(book: &ExtendedBook) -> String {
    let mut output = String::new();

    // Book header
    output.push_str(&format!("📚 {}\n", style(&book.book.title).bold().cyan()));
    output.push_str(&"─".repeat(50));
    output.push('\n');

    // Book details
    if let Some(id) = book.book.id {
        output.push_str(&format!("ID: {}\n", id));
    }

    if !book.authors.is_empty() {
        let authors: Vec<String> = book.authors.iter().map(|a| a.name.clone()).collect();
        output.push_str(&format!("Authors: {}\n", authors.join(", ")));
    }

    if !book.translators.is_empty() {
        let translators: Vec<String> = book.translators.iter().map(|t| t.name.clone()).collect();
        output.push_str(&format!("Translators: {}\n", translators.join(", ")));
    }

    if let Some(pages) = book.book.pages {
        output.push_str(&format!("Pages: {}\n", pages));
    }

    if let Some(year) = book.book.pub_year {
        output.push_str(&format!("Publication Year: {}\n", year));
    }

    if let Some(genre) = &book.book.genre {
        output.push_str(&format!("Genre: {}\n", genre));
    }

    // Reviews with detailed display
    if !book.reviews.is_empty() {
        let avg_rating = book.reviews.iter().map(|r| r.rating).sum::<i32>() as f32 / book.reviews.len() as f32;
        output.push('\n');
        output.push_str(&format!(
            "⭐ Reviews ({}) - Average: {:.1}/5\n",
            book.reviews.len(),
            avg_rating
        ));
        output.push_str(&"═".repeat(50));
        output.push('\n');

        for (i, review) in book.reviews.iter().enumerate() {
            output.push_str(&format!("{}. ", i + 1));
            output.push_str(&format_review_detailed(review));
            if i < book.reviews.len() - 1 {
                output.push_str(&"─".repeat(30));
                output.push('\n');
            }
        }
    } else {
        output.push('\n');
        output.push_str("📝 No reviews yet\n");
    }

    output
}

/// Format multiple books as a table
fn format_books_table(books: &[ExtendedBook]) -> String {
    if books.is_empty() {
        return "No books found.".to_string();
    }

    #[derive(Tabled)]
    struct BookRow {
        #[tabled(rename = "ID")]
        id: String,
        #[tabled(rename = "Title")]
        title: String,
        #[tabled(rename = "Authors")]
        authors: String,
        #[tabled(rename = "Year")]
        year: String,
        #[tabled(rename = "Pages")]
        pages: String,
        #[tabled(rename = "Reviews")]
        reviews: String,
    }

    let rows: Vec<BookRow> = books
        .iter()
        .map(|book| {
            let authors: Vec<String> = book.authors.iter().map(|a| a.name.clone()).collect();

            BookRow {
                id: book.book.id.map_or("N/A".to_string(), |id| id.to_string()),
                title: truncate_string(&book.book.title, 30),
                authors: truncate_string(&authors.join(", "), 25),
                year: book
                    .book
                    .pub_year
                    .map_or("N/A".to_string(), |y| y.to_string()),
                pages: book.book.pages.map_or("N/A".to_string(), |p| p.to_string()),
                reviews: book.reviews.len().to_string(),
            }
        })
        .collect();

    Table::new(rows).to_string()
}

/// Format books as a summary list
fn format_books_summary(books: &[ExtendedBook]) -> String {
    if books.is_empty() {
        return "No books found.".to_string();
    }

    let mut output = String::new();
    output.push_str(&format!("📚 Found {} book(s)\n\n", books.len()));

    for book in books {
        output.push_str(&format_book_summary(book));
        output.push('\n');
    }

    output
}

/// Format a single book as a summary
fn format_book_summary(book: &ExtendedBook) -> String {
    let authors: Vec<String> = book.authors.iter().map(|a| a.name.clone()).collect();
    let mut summary = format!(
        "• {} by {}",
        style(&book.book.title).bold(),
        authors.join(", ")
    );

    if let Some(year) = book.book.pub_year {
        summary.push_str(&format!(" ({})", year));
    }

    if !book.reviews.is_empty() {
        let avg_rating =
            book.reviews.iter().map(|r| r.rating).sum::<i32>() as f32 / book.reviews.len() as f32;
        summary.push_str(&format!(" - ⭐ {:.1}/5", avg_rating));
    }

    summary
}

/// Format a review summary
#[allow(dead_code)]
fn format_review_summary(review: &Review) -> String {
    let mut output = String::new();

    // Rating stars
    let stars = "⭐".repeat(review.rating as usize);
    output.push_str(&format!("{} {}/5", stars, review.rating));

    // Date
    if let Some(date) = review.date_read {
        output.push_str(&format!(
            " - {} ({})",
            format_date(&date),
            relative_date_description(&date)
        ));
    }

    output.push('\n');

    // Review text (truncated)
    let review_text = truncate_string(&review.review, 100);
    output.push_str(&format!("  {}", review_text));

    output
}

/// Format a detailed review
fn format_review_detailed(review: &Review) -> String {
    let mut output = String::new();

    // Rating stars
    let stars = "⭐".repeat(review.rating as usize);
    output.push_str(&format!("{} {}/5", style(&stars).yellow(), review.rating));

    // Date
    if let Some(date) = review.date_read {
        output.push_str(&format!(
            " - {} ({})",
            style(&format_date(&date)).dim(),
            style(&relative_date_description(&date)).dim()
        ));
    }

    output.push('\n');

    // Review text with proper formatting
    let lines: Vec<&str> = review.review.lines().collect();
    for line in lines {
        output.push_str(&format!("   {}\n", line));
    }

    output
}

/// Truncate a string to a maximum length with ellipsis
fn truncate_string(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else {
        format!("{}...", &s[..max_len.saturating_sub(3)])
    }
}

/// Format reading statistics
pub fn format_reading_stats(books: &[ExtendedBook]) -> String {
    let mut output = String::new();

    output.push_str(&format!(
        "📊 {}\n",
        style("Reading Statistics").bold().green()
    ));
    output.push_str(&"═".repeat(40));
    output.push('\n');

    // Basic stats
    let total_books = books.len();
    let total_pages: i32 = books.iter().filter_map(|b| b.book.pages).sum();
    let total_reviews = books.iter().map(|b| b.reviews.len()).sum::<usize>();

    output.push_str(&format!("Total Books: {}\n", total_books));
    output.push_str(&format!("Total Pages: {}\n", total_pages));
    output.push_str(&format!("Total Reviews: {}\n", total_reviews));

    if total_reviews > 0 {
        let total_rating: i32 = books
            .iter()
            .flat_map(|b| &b.reviews)
            .map(|r| r.rating)
            .sum();
        let avg_rating = total_rating as f32 / total_reviews as f32;
        output.push_str(&format!("Average Rating: {:.1}/5\n", avg_rating));
    }

    // Year breakdown by reading dates
    let mut year_counts = std::collections::HashMap::new();
    for book in books {
        for review in &book.reviews {
            if let Some(date_read) = review.date_read {
                let read_year = date_read.year();
                *year_counts.entry(read_year).or_insert(0) += 1;
            }
        }
    }

    if !year_counts.is_empty() {
        output.push('\n');
        output.push_str("Books by Year:\n");
        let mut years: Vec<_> = year_counts.iter().collect();
        years.sort_by_key(|(year, _)| *year);
        for (year, count) in years {
            output.push_str(&format!("  {}: {} book(s)\n", year, count));
        }
    }

    output
}

/// Format author statistics
pub fn format_author_stats(books: &[ExtendedBook]) -> String {
    let mut output = String::new();

    output.push_str(&format!(
        "👥 {}\n",
        style("Author Statistics").bold().blue()
    ));
    output.push_str(&"═".repeat(40));
    output.push('\n');

    // Count books per author
    let mut author_counts = std::collections::HashMap::new();
    for book in books {
        for author in &book.authors {
            *author_counts.entry(author.name.clone()).or_insert(0) += 1;
        }
    }

    if author_counts.is_empty() {
        output.push_str("No authors found.\n");
        return output;
    }

    // Sort by count (descending)
    let mut authors: Vec<_> = author_counts.iter().collect();
    authors.sort_by(|a, b| b.1.cmp(a.1));

    output.push_str(&format!("Total Authors: {}\n\n", authors.len()));
    output.push_str("Most Read Authors:\n");

    for (i, (author, count)) in authors.iter().take(10).enumerate() {
        output.push_str(&format!(
            "{}. {} ({} book{})\n",
            i + 1,
            author,
            count,
            if **count == 1 { "" } else { "s" }
        ));
    }

    output
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    fn create_test_book() -> ExtendedBook {
        ExtendedBook {
            book: Book {
                id: Some(1),
                title: "Test Book".to_string(),
                pages: Some(200),
                pub_year: Some(2023),
                genre: Some("Fiction".to_string()),
            },
            authors: vec![Writer {
                id: Some(1),
                name: "Test Author".to_string(),
                writer_type: WriterType::Author,
            }],
            translators: vec![],
            reviews: vec![Review {
                id: Some(1),
                book_id: 1,
                date_read: Some(NaiveDate::from_ymd_opt(2023, 12, 1).unwrap()),
                rating: 4,
                review: "Great book!".to_string(),
            }],
        }
    }

    #[test]
    fn test_book_display_json() {
        let book = create_test_book();
        let result = book.display(&OutputFormat::Json);
        assert!(result.is_ok());
        assert!(result.unwrap().contains("Test Book"));
    }

    #[test]
    fn test_book_display_table() {
        let book = create_test_book();
        let result = book.display(&OutputFormat::Table);
        assert!(result.is_ok());
        assert!(result.unwrap().contains("Test Book"));
    }

    #[test]
    fn test_truncate_string() {
        assert_eq!(truncate_string("short", 10), "short");
        assert_eq!(
            truncate_string("this is a very long string", 10),
            "this is..."
        );
    }
}
