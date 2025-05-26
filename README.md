# 📚 Libro CLI

[![CI](https://github.com/username/hwido-libro/workflows/CI/badge.svg)](https://github.com/username/hwido-libro/actions)
[![codecov](https://codecov.io/gh/username/hwido-libro/branch/main/graph/badge.svg)](https://codecov.io/gh/username/hwido-libro)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A command-line book tracking tool with data stored in SQLite. Track your reading
progress, manage reviews, and generate insightful reports about your reading
habits.

## ✨ Features

- 📖 **Book Management**: Add books with detailed metadata (authors,
  translators, pages, publication year, genre)
- ⭐ **Review System**: Rate and review books with multiple reviews per book
- 📊 **Reading Reports**: Generate statistics and visualizations of your reading
  habits
- 🎨 **Multiple Output Formats**: Table, JSON, and summary views
- 🔍 **Flexible Filtering**: Search by ID, year, or view all books
- 💾 **SQLite Storage**: Reliable local database with data integrity
- 🌈 **Colored Output**: Beautiful terminal interface with colored text and
  emojis

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

## 📁 Project Structure

```
hwido-libro/
├── libro-cli/                 # Main CLI application
│   ├── src/
│   │   ├── commands/          # CLI command implementations
│   │   │   ├── add.rs
│   │   │   ├── show.rs
│   │   │   ├── report.rs
│   │   │   ├── review.rs
│   │   │   └── edit_review.rs
│   │   ├── lib/               # Core library
│   │   │   ├── db.rs          # Database initialization
│   │   │   ├── models.rs      # Data models
│   │   │   ├── errors.rs      # Error handling
│   │   │   └── db_operations.rs # Database operations
│   │   ├── utils/             # Utility functions
│   │   │   ├── date.rs        # Date handling
│   │   │   ├── output.rs      # Output formatting
│   │   │   ├── input.rs       # Interactive input
│   │   │   ├── error_handler.rs # CLI error handling
│   │   │   └── database.rs    # Database utilities
│   │   ├── lib.rs             # Library entry point
│   │   └── main.rs            # CLI entry point
│   └── tests/                 # Integration tests
├── .github/workflows/         # CI/CD configuration
├── planner/                   # Project planning documents
└── README.md
```

## 🗄️ Database Schema

The application uses SQLite with the following schema:

```sql
-- Books table
CREATE TABLE books (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    title TEXT NOT NULL,
    pages INTEGER,
    pub_year INTEGER,
    genre TEXT
);

-- Writers table (authors and translators)
CREATE TABLE writers (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL UNIQUE,
    writer_type TEXT NOT NULL CHECK (writer_type IN ('author', 'translator'))
);

-- Book-writer relationships
CREATE TABLE book_writers (
    book_id INTEGER NOT NULL,
    writer_id INTEGER NOT NULL,
    writer_type TEXT NOT NULL,
    FOREIGN KEY (book_id) REFERENCES books(id) ON DELETE CASCADE,
    FOREIGN KEY (writer_id) REFERENCES writers(id),
    PRIMARY KEY (book_id, writer_id, writer_type)
);

-- Reviews table
CREATE TABLE reviews (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    book_id INTEGER NOT NULL,
    date_read TEXT,
    rating INTEGER NOT NULL CHECK (rating >= 1 AND rating <= 5),
    review TEXT NOT NULL,
    FOREIGN KEY (book_id) REFERENCES books(id) ON DELETE CASCADE
);
```

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Make your changes
4. Run tests (`cargo test`)
5. Run formatting and linting (`cargo fmt && cargo clippy`)
6. Commit your changes (`git commit -m 'Add amazing feature'`)
7. Push to the branch (`git push origin feature/amazing-feature`)
8. Open a Pull Request

## 📝 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file
for details.

## 🙏 Acknowledgments

- Originally implemented in Deno/TypeScript
- Migrated to Rust for better performance and distribution
- Inspired by personal reading tracking needs

## 📊 Roadmap

- [ ] **Step 8**: ✅ Documentation & CI/CD (Current)
- [ ] **Step 9**: Deployment & Packaging
  - [ ] GitHub Releases automation
  - [ ] Homebrew Formula
  - [ ] Scoop package for Windows
  - [ ] Docker image
- [ ] **Step 10**: TUI Application
  - [ ] Interactive terminal UI with `ratatui`
  - [ ] Real-time book browsing
  - [ ] Visual charts and graphs
  - [ ] Keyboard shortcuts

## 🐛 Known Issues

- Interactive input requires a TTY (won't work in non-interactive environments)
- Date parsing is limited to specific formats
- No data import/export functionality yet

## 📈 Performance

- **Startup time**: ~50ms
- **Database operations**: <10ms for typical queries
- **Memory usage**: ~5MB for typical workloads
- **Binary size**: ~8MB (release build)
