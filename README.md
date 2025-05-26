# 📚 Libro CLI

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

## 🚀 Installation

### Prerequisites

- [Rust](https://rustup.rs/) (1.70.0 or later)

### From Source

```bash
# Clone the repository
git clone https://github.com/username/hwido-libro.git
cd hwido-libro

# Build the project
cargo build --release

# The binary will be available at ./target/release/libro-cli
```

### Using Cargo

```bash
cargo install --path libro-cli
```

## 📖 Usage

### Basic Commands

```bash
# Add a new book (interactive)
libro-cli add

# Show all books
libro-cli show

# Show a specific book by ID
libro-cli show 1

# Show books from a specific year
libro-cli show --year 2023

# Show books in JSON format
libro-cli show --json

# Generate reading reports
libro-cli report

# Generate author statistics
libro-cli report --author

# Generate year-by-year reading chart
libro-cli report --years

# Add a review for a book
libro-cli review 1

# Edit an existing review
libro-cli edit-review 1
```

### Command Reference

| Command       | Arguments | Options                                | Description                  |
| ------------- | --------- | -------------------------------------- | ---------------------------- |
| `add`         | -         | -                                      | Add a new book interactively |
| `show`        | `[id]`    | `--year <year>`, `--json`              | Show book(s) by ID or year   |
| `report`      | -         | `--author`, `--year <year>`, `--years` | Generate reading reports     |
| `review`      | `<id>`    | -                                      | Add a review for a book      |
| `edit-review` | `<id>`    | -                                      | Edit an existing review      |

### Examples

#### Adding a Book

```bash
$ libro-cli add
📚 Adding a new book
──────────────────────────────
Book title: The Rust Programming Language
Author (required): Steve Klabnik
Additional author (press Enter to skip): Carol Nichols
Add another author? No
Does this book have translators? No
Do you want to specify the number of pages? Yes
Number of pages: 552
Do you want to specify the publication year? Yes
Publication year: 2018
Do you want to specify a genre? Yes
Select a genre: [Use arrow keys]
> Non-fiction
  Fiction
  Science Fiction
  ...

Would you like to add a review for this book? Yes
📝 Adding a review for 'The Rust Programming Language'
────────────────────────────────────────────
Do you want to specify when you read this book? Yes
Date read (YYYY-MM-DD, or press Enter for today): 2023-12-01
Rating (1-5 stars): 5
Review text: Excellent introduction to Rust programming!

✅ Success: Book added successfully! Book ID: 1, Review ID: 1
```

#### Viewing Books

```bash
$ libro-cli show
📚 Found 3 book(s)

• The Rust Programming Language by Steve Klabnik, Carol Nichols (2018) - ⭐ 5.0/5
• Clean Code by Robert C. Martin (2008) - ⭐ 4.5/5
• Design Patterns by Gang of Four (1994)
```

#### Generating Reports

```bash
$ libro-cli report --author
👥 Author Statistics
════════════════════════════════════════
Total Authors: 5

Most Read Authors:
1. Robert C. Martin (2 books)
2. Steve Klabnik (1 book)
3. Carol Nichols (1 book)
4. Gang of Four (1 book)
5. Martin Fowler (1 book)
```

## 🛠️ Development

### Building

```bash
# Debug build
cargo build

# Release build
cargo build --release

# Run tests
cargo test

# Run with logging
RUST_LOG=debug cargo run -- show
```

### Testing

```bash
# Run all tests
cargo test

# Run specific test suite
cargo test --test db_operations
cargo test --test cli_parse
cargo test --test integration_tests

# Run tests with output
cargo test -- --nocapture
```

### Code Quality

```bash
# Format code
cargo fmt

# Run clippy lints
cargo clippy

# Check for security vulnerabilities
cargo audit
```

### Environment Variables

- `LIBRO_DB_PATH`: Custom database file path (default: `libro.db`)
- `RUST_LOG`: Logging level (`debug`, `info`, `warn`, `error`)
