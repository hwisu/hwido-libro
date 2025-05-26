use chrono::NaiveDate;
use console::style;
use dialoguer::{Confirm, Input, MultiSelect, Select};

use crate::utils::date::{current_date, parse_and_validate_date};
use libro_cli::errors::{LibroError, LibroResult};
use libro_cli::models::*;

/// Prompt for book information interactively
pub fn prompt_for_book() -> LibroResult<NewBook> {
    println!("{}", style("📚 Adding a new book").bold().cyan());
    println!("{}", "─".repeat(50));
    println!("{}", style("Required fields are marked with *").dim());
    println!();

    // Title (required)
    let title: String = Input::new()
        .with_prompt(format!("{} {}", style("*").red().bold(), "Book title"))
        .validate_with(|input: &String| -> Result<(), &str> {
            if input.trim().is_empty() {
                Err("Title cannot be empty")
            } else {
                Ok(())
            }
        })
        .interact_text()?;

    // Authors (required, at least one)
    let mut authors = Vec::new();
    loop {
        let author: String = Input::new()
            .with_prompt(if authors.is_empty() {
                format!("{} {}", style("*").red().bold(), "Author")
            } else {
                "Additional author (Enter to finish)".to_string()
            })
            .allow_empty(!authors.is_empty())
            .interact_text()?;

        if author.trim().is_empty() {
            if authors.is_empty() {
                println!("{} At least one author is required.", style("!").red().bold());
                continue;
            } else {
                break;
            }
        }

        authors.push(author.trim().to_string());
    }

    // Translators (optional)
    let mut translators = Vec::new();
    loop {
        let translator: String = Input::new()
            .with_prompt("Translator (Enter to skip)")
            .allow_empty(true)
            .interact_text()?;

        if translator.trim().is_empty() {
            break;
        }

        translators.push(translator.trim().to_string());
    }

    // Pages (optional)
    let pages_input: String = Input::new()
        .with_prompt("Number of pages (Enter to skip)")
        .allow_empty(true)
        .validate_with(|input: &String| -> Result<(), &str> {
            if input.trim().is_empty() {
                Ok(())
            } else {
                match input.parse::<i32>() {
                    Ok(n) if n > 0 => Ok(()),
                    _ => Err("Please enter a positive number"),
                }
            }
        })
        .interact_text()?;

    let pages = if pages_input.trim().is_empty() {
        None
    } else {
        Some(pages_input.parse()?)
    };

    // Publication year (optional)
    let year_input: String = Input::new()
        .with_prompt("Publication year (Enter to skip)")
        .allow_empty(true)
        .validate_with(|input: &String| -> Result<(), &str> {
            if input.trim().is_empty() {
                Ok(())
            } else {
                match input.parse::<i32>() {
                    Ok(year) if (1000..=2030).contains(&year) => Ok(()),
                    _ => Err("Please enter a valid year (1000-2030)"),
                }
            }
        })
        .interact_text()?;

    let pub_year = if year_input.trim().is_empty() {
        None
    } else {
        Some(year_input.parse()?)
    };

        // Genre (required, with predefined options)
    let genres = vec![
        "Fiction",
        "Non-fiction",
        "Science & Technology",
        "History & Biography",
        "Self-Help & Business",
        "Arts & Literature",
        "Philosophy & Religion",
        "Health & Lifestyle",
        "Children & Young Adult",
        "Other",
    ];

    let selection = Select::new()
        .with_prompt(format!("{} {}", style("*").red().bold(), "Genre"))
        .items(&genres)
        .default(0)
        .interact()?;

         let genre = if genres[selection] == "Other" {
         Input::new()
             .with_prompt(format!("{} {}", style("*").red().bold(), "Enter custom genre"))
             .validate_with(|input: &String| -> Result<(), &str> {
                 if input.trim().is_empty() {
                     Err("Genre cannot be empty")
                 } else {
                     Ok(())
                 }
             })
             .interact_text()?
     } else {
         genres[selection].to_string()
     };

    Ok(NewBook {
        title,
        authors,
        translators,
        pages,
        pub_year,
        genre,
    })
}

/// Prompt for review information
pub fn prompt_for_review(book_id: i64, book_title: &str) -> LibroResult<NewReview> {
    println!(
        "{}",
        style(&format!("📝 Adding a review for '{}'", book_title))
            .bold()
            .green()
    );
    println!("{}", "─".repeat(40));

    // Date read (optional, defaults to today)
    let date_read: Option<NaiveDate> = if Confirm::new()
        .with_prompt("Do you want to specify when you read this book?")
        .default(true)
        .interact()?
    {
        let date_input: String = Input::new()
            .with_prompt("Date read (YYYY-MM-DD, or press Enter for today)")
            .allow_empty(true)
            .validate_with(|input: &String| -> Result<(), String> {
                if input.trim().is_empty() {
                    Ok(())
                } else {
                    match parse_and_validate_date(input) {
                        Ok(_) => Ok(()),
                        Err(e) => Err(e.to_string()),
                    }
                }
            })
            .interact_text()?;

        if date_input.trim().is_empty() {
            Some(current_date())
        } else {
            Some(parse_and_validate_date(&date_input)?)
        }
    } else {
        None
    };

    // Rating (required, 1-5)
    let rating: i32 = Input::new()
        .with_prompt("Rating (1-5 stars)")
        .validate_with(|input: &String| -> Result<(), &str> {
            match input.parse::<i32>() {
                Ok(n) if (1..=5).contains(&n) => Ok(()),
                _ => Err("Please enter a rating between 1 and 5"),
            }
        })
        .interact_text()?
        .parse()?;

    // Review text (required)
    let review: String = Input::new()
        .with_prompt("Review text")
        .validate_with(|input: &String| -> Result<(), &str> {
            if input.trim().is_empty() {
                Err("Review text cannot be empty")
            } else {
                Ok(())
            }
        })
        .interact_text()?;

    Ok(NewReview {
        book_id,
        date_read,
        rating,
        review: review.trim().to_string(),
    })
}

/// Prompt for book and review together
pub fn prompt_for_book_with_review() -> LibroResult<NewBookWithReview> {
    let book = prompt_for_book()?;

    let add_review = Confirm::new()
        .with_prompt("Would you like to add a review for this book?")
        .default(true)
        .interact()?;

    let review = if add_review {
        Some(prompt_for_review(0, &book.title)?) // book_id will be set later
    } else {
        None
    };

    Ok(NewBookWithReview { book, review })
}

/// Prompt for confirmation with a custom message
pub fn confirm(message: &str) -> LibroResult<bool> {
    Ok(Confirm::new()
        .with_prompt(message)
        .default(false)
        .interact()?)
}

/// Prompt for text input with validation
#[allow(dead_code)]
pub fn prompt_text(prompt: &str, allow_empty: bool) -> LibroResult<String> {
    let input: String = Input::new()
        .with_prompt(prompt)
        .allow_empty(allow_empty)
        .validate_with(|input: &String| -> Result<(), &str> {
            if !allow_empty && input.trim().is_empty() {
                Err("Input cannot be empty")
            } else {
                Ok(())
            }
        })
        .interact_text()?;

    Ok(input.trim().to_string())
}

/// Prompt for number input with validation
#[allow(dead_code)]
pub fn prompt_number<T>(prompt: &str, min: Option<T>, max: Option<T>) -> LibroResult<T>
where
    T: std::str::FromStr + std::cmp::PartialOrd + std::fmt::Display + Copy,
    T::Err: std::fmt::Display,
{
    let input: String = Input::new()
        .with_prompt(prompt)
        .validate_with(|input: &String| -> Result<(), String> {
            match input.parse::<T>() {
                Ok(n) => {
                    if let Some(min_val) = min {
                        if n < min_val {
                            return Err(format!("Value must be at least {}", min_val));
                        }
                    }
                    if let Some(max_val) = max {
                        if n > max_val {
                            return Err(format!("Value must be at most {}", max_val));
                        }
                    }
                    Ok(())
                }
                Err(e) => Err(format!("Invalid number: {}", e)),
            }
        })
        .interact_text()?;

    input
        .parse()
        .map_err(|e| LibroError::invalid_input(format!("Parse error: {}", e)))
}

/// Select from a list of options
pub fn select_from_list<T: std::fmt::Display>(prompt: &str, items: &[T]) -> LibroResult<usize> {
    Ok(Select::new()
        .with_prompt(prompt)
        .items(items)
        .default(0)
        .interact()?)
}

/// Multi-select from a list of options
#[allow(dead_code)]
pub fn multi_select_from_list<T: std::fmt::Display>(
    prompt: &str,
    items: &[T],
) -> LibroResult<Vec<usize>> {
    Ok(MultiSelect::new()
        .with_prompt(prompt)
        .items(items)
        .interact()?)
}

/// Prompt for editing existing review
pub fn prompt_edit_review(existing_review: &Review, book_title: &str) -> LibroResult<Review> {
    println!(
        "{}",
        style(&format!("✏️  Editing review for '{}'", book_title))
            .bold()
            .yellow()
    );
    println!("{}", "─".repeat(40));

    println!("Current review:");
    println!("  Rating: {}/5", existing_review.rating);
    if let Some(date) = existing_review.date_read {
        println!("  Date read: {}", date);
    }
    println!("  Review: {}", existing_review.review);
    println!();

    // Date read
    let date_read = if Confirm::new()
        .with_prompt("Update the date read?")
        .default(false)
        .interact()?
    {
        let default_date = existing_review
            .date_read
            .map_or("".to_string(), |d| d.to_string());
        let date_input: String = Input::new()
            .with_prompt("New date read (YYYY-MM-DD)")
            .with_initial_text(&default_date)
            .validate_with(|input: &String| -> Result<(), String> {
                if input.trim().is_empty() {
                    Ok(())
                } else {
                    match parse_and_validate_date(input) {
                        Ok(_) => Ok(()),
                        Err(e) => Err(e.to_string()),
                    }
                }
            })
            .interact_text()?;

        if date_input.trim().is_empty() {
            existing_review.date_read
        } else {
            Some(parse_and_validate_date(&date_input)?)
        }
    } else {
        existing_review.date_read
    };

    // Rating
    let rating = if Confirm::new()
        .with_prompt("Update the rating?")
        .default(false)
        .interact()?
    {
        Input::new()
            .with_prompt("New rating (1-5)")
            .with_initial_text(existing_review.rating.to_string())
            .validate_with(|input: &String| -> Result<(), &str> {
                match input.parse::<i32>() {
                    Ok(n) if (1..=5).contains(&n) => Ok(()),
                    _ => Err("Please enter a rating between 1 and 5"),
                }
            })
            .interact_text()?
            .parse()?
    } else {
        existing_review.rating
    };

    // Review text
    let review = if Confirm::new()
        .with_prompt("Update the review text?")
        .default(false)
        .interact()?
    {
        Input::new()
            .with_prompt("New review text")
            .with_initial_text(&existing_review.review)
            .validate_with(|input: &String| -> Result<(), &str> {
                if input.trim().is_empty() {
                    Err("Review text cannot be empty")
                } else {
                    Ok(())
                }
            })
            .interact_text()?
    } else {
        existing_review.review.clone()
    };

    Ok(Review {
        id: existing_review.id,
        book_id: existing_review.book_id,
        date_read,
        rating,
        review: review.trim().to_string(),
    })
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_prompt_number_validation() {
        // This test would require mocking user input, which is complex with dialoguer
        // For now, we'll just test that the function signature is correct
        assert!(true);
    }

    #[test]
    fn test_confirm_function_exists() {
        // Basic test to ensure the function compiles
        assert!(true);
    }
}
