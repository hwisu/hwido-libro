use chrono::NaiveDate;
use rusqlite::{params, Connection, OptionalExtension};

use crate::lib::errors::{validation, LibroError, LibroResult};
use crate::lib::models::*;

/// Database operations struct that wraps a SQLite connection
pub struct Database {
    conn: Connection,
}

impl Database {
    /// Create a new database instance and initialize schema
    pub fn new(path: &str) -> LibroResult<Self> {
        let conn = crate::lib::db::init_db(path)?;
        Ok(Database { conn })
    }

    /// Get or add a writer by name and type
    pub fn get_or_add_writer(&mut self, name: &str, writer_type: WriterType) -> LibroResult<i64> {
        validation::validate_non_empty(name, "Writer name")?;

        // Check if writer already exists
        let existing_id: Option<i64> = self
            .conn
            .query_row(
                "SELECT id FROM writers WHERE name = ? AND type = ?",
                params![name, writer_type.as_str()],
                |row| row.get(0),
            )
            .optional()?;

        if let Some(id) = existing_id {
            Ok(id)
        } else {
            // Add new writer
            self.conn.execute(
                "INSERT INTO writers (name, type) VALUES (?, ?)",
                params![name, writer_type.as_str()],
            )?;
            Ok(self.conn.last_insert_rowid())
        }
    }

    /// Add a link between a book and a writer
    pub fn add_book_writer_link(
        &mut self,
        book_id: i64,
        writer_id: i64,
        writer_type: WriterType,
    ) -> LibroResult<()> {
        self.conn.execute(
            "INSERT INTO book_writers (book_id, writer_id, type) VALUES (?, ?, ?)",
            params![book_id, writer_id, writer_type.as_str()],
        )?;
        Ok(())
    }

    /// Add a new book to the database
    pub fn add_book(&mut self, book: &NewBook) -> LibroResult<i64> {
        validation::validate_non_empty(&book.title, "Title")?;

        if book.authors.is_empty() {
            return Err(LibroError::validation("At least one author is required"));
        }

        if let Some(pages) = book.pages {
            validation::validate_pages(pages)?;
        }

        if let Some(year) = book.pub_year {
            validation::validate_year(year)?;
        }

        // Insert book
        self.conn.execute(
            "INSERT INTO books (title, pages, pub_year, genre) VALUES (?, ?, ?, ?)",
            params![book.title, book.pages, book.pub_year, book.genre],
        )?;
        let book_id = self.conn.last_insert_rowid();

        // Add authors
        for author_name in &book.authors {
            let author_id = self.get_or_add_writer(author_name, WriterType::Author)?;
            self.add_book_writer_link(book_id, author_id, WriterType::Author)?;
        }

        // Add translators
        for translator_name in &book.translators {
            let translator_id = self.get_or_add_writer(translator_name, WriterType::Translator)?;
            self.add_book_writer_link(book_id, translator_id, WriterType::Translator)?;
        }

        Ok(book_id)
    }

    /// Add a new review to the database
    pub fn add_review(&mut self, review: &NewReview) -> LibroResult<i64> {
        validation::validate_rating(review.rating)?;
        validation::validate_non_empty(&review.review, "Review text")?;

        // Check if book exists
        let book_exists: bool = self
            .conn
            .query_row(
                "SELECT 1 FROM books WHERE id = ?",
                params![review.book_id],
                |_| Ok(true),
            )
            .optional()?
            .unwrap_or(false);

        if !book_exists {
            return Err(LibroError::BookNotFound { id: review.book_id });
        }

        let date_str = review
            .date_read
            .map(|d| d.format("%Y-%m-%d").to_string())
            .unwrap_or_else(|| chrono::Utc::now().format("%Y-%m-%d").to_string());

        self.conn.execute(
            "INSERT INTO reviews (book_id, date_read, rating, review) VALUES (?, ?, ?, ?)",
            params![review.book_id, date_str, review.rating, review.review],
        )?;

        Ok(self.conn.last_insert_rowid())
    }

    /// Add a book with an optional review in a transaction
    pub fn add_book_with_review(
        &mut self,
        input: &NewBookWithReview,
    ) -> LibroResult<BookCreationResult> {
        let tx = self.conn.transaction()?;

        let book_id = {
            // Insert book within transaction
            tx.execute(
                "INSERT INTO books (title, pages, pub_year, genre) VALUES (?, ?, ?, ?)",
                params![
                    input.book.title,
                    input.book.pages,
                    input.book.pub_year,
                    input.book.genre
                ],
            )?;
            tx.last_insert_rowid()
        };

        // Add authors within transaction
        for author_name in &input.book.authors {
            let author_id = {
                let existing_id: Option<i64> = tx
                    .query_row(
                        "SELECT id FROM writers WHERE name = ? AND type = ?",
                        params![author_name, "author"],
                        |row| row.get(0),
                    )
                    .optional()?;

                if let Some(id) = existing_id {
                    id
                } else {
                    tx.execute(
                        "INSERT INTO writers (name, type) VALUES (?, ?)",
                        params![author_name, "author"],
                    )?;
                    tx.last_insert_rowid()
                }
            };

            tx.execute(
                "INSERT INTO book_writers (book_id, writer_id, type) VALUES (?, ?, ?)",
                params![book_id, author_id, "author"],
            )?;
        }

        // Add translators within transaction
        for translator_name in &input.book.translators {
            let translator_id = {
                let existing_id: Option<i64> = tx
                    .query_row(
                        "SELECT id FROM writers WHERE name = ? AND type = ?",
                        params![translator_name, "translator"],
                        |row| row.get(0),
                    )
                    .optional()?;

                if let Some(id) = existing_id {
                    id
                } else {
                    tx.execute(
                        "INSERT INTO writers (name, type) VALUES (?, ?)",
                        params![translator_name, "translator"],
                    )?;
                    tx.last_insert_rowid()
                }
            };

            tx.execute(
                "INSERT INTO book_writers (book_id, writer_id, type) VALUES (?, ?, ?)",
                params![book_id, translator_id, "translator"],
            )?;
        }

        let review_id = if let Some(review) = &input.review {
            let date_str = review
                .date_read
                .map(|d| d.format("%Y-%m-%d").to_string())
                .unwrap_or_else(|| chrono::Utc::now().format("%Y-%m-%d").to_string());

            tx.execute(
                "INSERT INTO reviews (book_id, date_read, rating, review) VALUES (?, ?, ?, ?)",
                params![book_id, date_str, review.rating, review.review],
            )?;
            Some(tx.last_insert_rowid())
        } else {
            None
        };

        // Commit the transaction
        tx.commit()?;

        Ok(BookCreationResult { book_id, review_id })
    }

    /// Get books with optional filtering
    pub fn get_books(&self, filter: &BookFilter) -> LibroResult<Vec<ExtendedBook>> {
        if let Some(id) = filter.id {
            let mut stmt = self.conn.prepare(
                "SELECT DISTINCT b.id, b.title, b.pages, b.pub_year, b.genre FROM books b WHERE b.id = ? ORDER BY b.id"
            )?;
            let book_rows = stmt.query_map(params![id], |row| {
                Ok(Book {
                    id: Some(row.get(0)?),
                    title: row.get(1)?,
                    pages: row.get(2)?,
                    pub_year: row.get(3)?,
                    genre: row.get(4)?,
                })
            })?;
            self.process_book_rows(book_rows)
        } else if let Some(year) = filter.year {
            let mut stmt = self.conn.prepare(
                "SELECT DISTINCT b.id, b.title, b.pages, b.pub_year, b.genre FROM books b WHERE b.pub_year = ? ORDER BY b.id"
            )?;
            let book_rows = stmt.query_map(params![year], |row| {
                Ok(Book {
                    id: Some(row.get(0)?),
                    title: row.get(1)?,
                    pages: row.get(2)?,
                    pub_year: row.get(3)?,
                    genre: row.get(4)?,
                })
            })?;
            self.process_book_rows(book_rows)
        } else {
            let mut stmt = self.conn.prepare(
                "SELECT DISTINCT b.id, b.title, b.pages, b.pub_year, b.genre FROM books b ORDER BY b.id"
            )?;
            let book_rows = stmt.query_map([], |row| {
                Ok(Book {
                    id: Some(row.get(0)?),
                    title: row.get(1)?,
                    pages: row.get(2)?,
                    pub_year: row.get(3)?,
                    genre: row.get(4)?,
                })
            })?;
            self.process_book_rows(book_rows)
        }
    }

    /// Helper method to process book rows and add related data
    fn process_book_rows(
        &self,
        book_rows: rusqlite::MappedRows<impl FnMut(&rusqlite::Row) -> rusqlite::Result<Book>>,
    ) -> LibroResult<Vec<ExtendedBook>> {
        let mut extended_books = Vec::new();

        for book_result in book_rows {
            let book = book_result?;
            let book_id = book.id.unwrap();

            // Get writers for this book
            let all_writers = self.get_book_writers(book_id)?;
            let authors: Vec<Writer> = all_writers
                .iter()
                .filter(|w| matches!(w.writer_type, WriterType::Author))
                .cloned()
                .collect();
            let translators: Vec<Writer> = all_writers
                .iter()
                .filter(|w| matches!(w.writer_type, WriterType::Translator))
                .cloned()
                .collect();

            // Get reviews for this book
            let reviews = self.get_reviews(book_id)?;

            extended_books.push(ExtendedBook {
                book,
                authors,
                translators,
                reviews,
            });
        }

        Ok(extended_books)
    }

    /// Get all writers for a specific book
    pub fn get_book_writers(&self, book_id: i64) -> LibroResult<Vec<Writer>> {
        let mut stmt = self.conn.prepare(
            "SELECT w.id, w.name, w.type
             FROM writers w
             JOIN book_writers bw ON w.id = bw.writer_id
             WHERE bw.book_id = ?
             ORDER BY w.type, w.name",
        )?;

        let writer_rows = stmt.query_map(params![book_id], |row| {
            let writer_type_str: String = row.get(2)?;
            let writer_type = WriterType::from_str(&writer_type_str).map_err(|_| {
                rusqlite::Error::InvalidColumnType(
                    2,
                    "type".to_string(),
                    rusqlite::types::Type::Text,
                )
            })?;

            Ok(Writer {
                id: Some(row.get(0)?),
                name: row.get(1)?,
                writer_type,
            })
        })?;

        let mut writers = Vec::new();
        for writer_result in writer_rows {
            writers.push(writer_result?);
        }

        Ok(writers)
    }

    /// Get all reviews for a specific book
    pub fn get_reviews(&self, book_id: i64) -> LibroResult<Vec<Review>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, book_id, date_read, rating, review
             FROM reviews
             WHERE book_id = ?
             ORDER BY date_read DESC",
        )?;

        let review_rows = stmt.query_map(params![book_id], |row| {
            let date_str: Option<String> = row.get(2)?;
            let date_read = date_str.and_then(|s| NaiveDate::parse_from_str(&s, "%Y-%m-%d").ok());

            Ok(Review {
                id: Some(row.get(0)?),
                book_id: row.get(1)?,
                date_read,
                rating: row.get(3)?,
                review: row.get(4)?,
            })
        })?;

        let mut reviews = Vec::new();
        for review_result in review_rows {
            reviews.push(review_result?);
        }

        Ok(reviews)
    }

    /// Update an existing book
    pub fn update_book(&mut self, book_id: i64, updates: &Book) -> LibroResult<()> {
        validation::validate_non_empty(&updates.title, "Title")?;

        if let Some(pages) = updates.pages {
            validation::validate_pages(pages)?;
        }

        if let Some(year) = updates.pub_year {
            validation::validate_year(year)?;
        }

        let rows_affected = self.conn.execute(
            "UPDATE books SET title = ?, pages = ?, pub_year = ?, genre = ? WHERE id = ?",
            params![
                updates.title,
                updates.pages,
                updates.pub_year,
                updates.genre,
                book_id
            ],
        )?;

        if rows_affected == 0 {
            return Err(LibroError::BookNotFound { id: book_id });
        }

        Ok(())
    }

    /// Update an existing review
    pub fn update_review(&mut self, review_id: i64, updates: &Review) -> LibroResult<()> {
        validation::validate_rating(updates.rating)?;
        validation::validate_non_empty(&updates.review, "Review text")?;

        let date_str = updates.date_read.map(|d| d.format("%Y-%m-%d").to_string());

        let rows_affected = self.conn.execute(
            "UPDATE reviews SET date_read = ?, rating = ?, review = ? WHERE id = ?",
            params![date_str, updates.rating, updates.review, review_id],
        )?;

        if rows_affected == 0 {
            return Err(LibroError::ReviewNotFound { book_id: review_id });
        }

        Ok(())
    }

    /// Delete a book and all associated data
    pub fn delete_book(&mut self, book_id: i64) -> LibroResult<()> {
        // 트랜잭션 시작
        let tx = self.conn.transaction()?;

        // 먼저 관련 리뷰들 삭제
        tx.execute("DELETE FROM reviews WHERE book_id = ?", params![book_id])?;

        // 관련 book_writers 링크 삭제 (ON DELETE CASCADE가 있지만 명시적으로)
        tx.execute(
            "DELETE FROM book_writers WHERE book_id = ?",
            params![book_id],
        )?;

        // 마지막으로 도서 삭제
        let rows_affected = tx.execute("DELETE FROM books WHERE id = ?", params![book_id])?;

        if rows_affected == 0 {
            return Err(LibroError::BookNotFound { id: book_id });
        }

        // 트랜잭션 커밋
        tx.commit()?;
        Ok(())
    }

    /// Delete a review
    pub fn delete_review(&mut self, review_id: i64) -> LibroResult<()> {
        let rows_affected = self
            .conn
            .execute("DELETE FROM reviews WHERE id = ?", params![review_id])?;

        if rows_affected == 0 {
            return Err(LibroError::ReviewNotFound { book_id: review_id });
        }

        Ok(())
    }
}
