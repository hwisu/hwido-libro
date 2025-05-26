use chrono::NaiveDate;
use libro_cli::db_operations::Database;
use libro_cli::errors::LibroError;
use libro_cli::models::*;

/// Helper function to create a temporary in-memory database for testing
fn create_test_db() -> Database {
    Database::new(":memory:").expect("Failed to create test database")
}

/// Helper function to create a sample book
fn create_sample_book() -> NewBook {
    NewBook {
        title: "Test Book".to_string(),
        authors: vec!["Test Author".to_string()],
        translators: vec!["Test Translator".to_string()],
        pages: Some(200),
        pub_year: Some(2023),
        genre: Some("Fiction".to_string()),
    }
}

/// Helper function to create a sample review
fn create_sample_review(book_id: i64) -> NewReview {
    NewReview {
        book_id,
        date_read: Some(NaiveDate::from_ymd_opt(2023, 12, 1).unwrap()),
        rating: 4,
        review: "Great book!".to_string(),
    }
}

#[test]
fn test_database_creation() {
    let _db = create_test_db();
    // If we get here without panicking, database creation succeeded
    assert!(true);
}

#[test]
fn test_add_book_basic() {
    let mut db = create_test_db();
    let book = create_sample_book();

    let book_id = db.add_book(&book).expect("Failed to add book");
    assert!(book_id > 0);
}

#[test]
fn test_add_book_validation() {
    let mut db = create_test_db();

    // Test empty title
    let invalid_book = NewBook {
        title: "".to_string(),
        authors: vec!["Author".to_string()],
        translators: vec![],
        pages: None,
        pub_year: None,
        genre: None,
    };

    let result = db.add_book(&invalid_book);
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), LibroError::Validation { .. }));

    // Test no authors
    let invalid_book = NewBook {
        title: "Valid Title".to_string(),
        authors: vec![],
        translators: vec![],
        pages: None,
        pub_year: None,
        genre: None,
    };

    let result = db.add_book(&invalid_book);
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), LibroError::Validation { .. }));
}

#[test]
fn test_add_review() {
    let mut db = create_test_db();
    let book = create_sample_book();

    let book_id = db.add_book(&book).expect("Failed to add book");
    let review = create_sample_review(book_id);

    let review_id = db.add_review(&review).expect("Failed to add review");
    assert!(review_id > 0);
}

#[test]
fn test_add_review_validation() {
    let mut db = create_test_db();
    let book = create_sample_book();

    let book_id = db.add_book(&book).expect("Failed to add book");

    // Test invalid rating
    let invalid_review = NewReview {
        book_id,
        date_read: None,
        rating: 6, // Invalid rating
        review: "Test review".to_string(),
    };

    let result = db.add_review(&invalid_review);
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), LibroError::Validation { .. }));

    // Test empty review text
    let invalid_review = NewReview {
        book_id,
        date_read: None,
        rating: 4,
        review: "".to_string(), // Empty review
    };

    let result = db.add_review(&invalid_review);
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), LibroError::Validation { .. }));

    // Test non-existent book
    let invalid_review = NewReview {
        book_id: 999, // Non-existent book
        date_read: None,
        rating: 4,
        review: "Test review".to_string(),
    };

    let result = db.add_review(&invalid_review);
    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        LibroError::BookNotFound { .. }
    ));
}

#[test]
fn test_add_book_with_review() {
    let mut db = create_test_db();
    let book = create_sample_book();
    let review = NewReview {
        book_id: 0, // Will be set by the function
        date_read: Some(NaiveDate::from_ymd_opt(2023, 12, 1).unwrap()),
        rating: 5,
        review: "Excellent book!".to_string(),
    };

    let input = NewBookWithReview {
        book,
        review: Some(review),
    };

    let result = db
        .add_book_with_review(&input)
        .expect("Failed to add book with review");
    assert!(result.book_id > 0);
    assert!(result.review_id.is_some());
    assert!(result.review_id.unwrap() > 0);
}

#[test]
fn test_get_books_all() {
    let mut db = create_test_db();
    let book1 = create_sample_book();
    let mut book2 = create_sample_book();
    book2.title = "Second Book".to_string();

    db.add_book(&book1).expect("Failed to add first book");
    db.add_book(&book2).expect("Failed to add second book");

    let filter = BookFilter::default();
    let books = db.get_books(&filter).expect("Failed to get books");

    assert_eq!(books.len(), 2);
    assert_eq!(books[0].book.title, "Test Book");
    assert_eq!(books[1].book.title, "Second Book");
}

#[test]
fn test_get_books_by_id() {
    let mut db = create_test_db();
    let book = create_sample_book();

    let book_id = db.add_book(&book).expect("Failed to add book");

    let filter = BookFilter {
        id: Some(book_id),
        year: None,
    };
    let books = db.get_books(&filter).expect("Failed to get books");

    assert_eq!(books.len(), 1);
    assert_eq!(books[0].book.title, "Test Book");
    assert_eq!(books[0].book.id, Some(book_id));
}

#[test]
fn test_get_books_by_year() {
    let mut db = create_test_db();
    let mut book1 = create_sample_book();
    book1.pub_year = Some(2022);
    let mut book2 = create_sample_book();
    book2.pub_year = Some(2023);
    book2.title = "Book 2023".to_string();

    db.add_book(&book1).expect("Failed to add book 1");
    db.add_book(&book2).expect("Failed to add book 2");

    let filter = BookFilter {
        id: None,
        year: Some(2023),
    };
    let books = db.get_books(&filter).expect("Failed to get books");

    assert_eq!(books.len(), 1);
    assert_eq!(books[0].book.title, "Book 2023");
}

#[test]
fn test_get_book_writers() {
    let mut db = create_test_db();
    let book = create_sample_book();

    let book_id = db.add_book(&book).expect("Failed to add book");
    let writers = db.get_book_writers(book_id).expect("Failed to get writers");

    assert_eq!(writers.len(), 2); // 1 author + 1 translator

    let authors: Vec<_> = writers
        .iter()
        .filter(|w| matches!(w.writer_type, WriterType::Author))
        .collect();
    let translators: Vec<_> = writers
        .iter()
        .filter(|w| matches!(w.writer_type, WriterType::Translator))
        .collect();

    assert_eq!(authors.len(), 1);
    assert_eq!(translators.len(), 1);
    assert_eq!(authors[0].name, "Test Author");
    assert_eq!(translators[0].name, "Test Translator");
}

#[test]
fn test_get_reviews() {
    let mut db = create_test_db();
    let book = create_sample_book();

    let book_id = db.add_book(&book).expect("Failed to add book");
    let review = create_sample_review(book_id);

    db.add_review(&review).expect("Failed to add review");
    let reviews = db.get_reviews(book_id).expect("Failed to get reviews");

    assert_eq!(reviews.len(), 1);
    assert_eq!(reviews[0].rating, 4);
    assert_eq!(reviews[0].review, "Great book!");
    assert_eq!(reviews[0].book_id, book_id);
}

#[test]
fn test_update_book() {
    let mut db = create_test_db();
    let book = create_sample_book();

    let book_id = db.add_book(&book).expect("Failed to add book");

    let updated_book = Book {
        id: Some(book_id),
        title: "Updated Title".to_string(),
        pages: Some(300),
        pub_year: Some(2024),
        genre: Some("Non-fiction".to_string()),
    };

    db.update_book(book_id, &updated_book)
        .expect("Failed to update book");

    let filter = BookFilter {
        id: Some(book_id),
        year: None,
    };
    let books = db.get_books(&filter).expect("Failed to get updated book");

    assert_eq!(books.len(), 1);
    assert_eq!(books[0].book.title, "Updated Title");
    assert_eq!(books[0].book.pages, Some(300));
    assert_eq!(books[0].book.pub_year, Some(2024));
    assert_eq!(books[0].book.genre, Some("Non-fiction".to_string()));
}

#[test]
fn test_update_review() {
    let mut db = create_test_db();
    let book = create_sample_book();

    let book_id = db.add_book(&book).expect("Failed to add book");
    let review = create_sample_review(book_id);

    let review_id = db.add_review(&review).expect("Failed to add review");

    let updated_review = Review {
        id: Some(review_id),
        book_id,
        date_read: Some(NaiveDate::from_ymd_opt(2023, 12, 15).unwrap()),
        rating: 5,
        review: "Updated review text".to_string(),
    };

    db.update_review(review_id, &updated_review)
        .expect("Failed to update review");

    let reviews = db
        .get_reviews(book_id)
        .expect("Failed to get updated review");

    assert_eq!(reviews.len(), 1);
    assert_eq!(reviews[0].rating, 5);
    assert_eq!(reviews[0].review, "Updated review text");
}

#[test]
fn test_delete_book() {
    let mut db = create_test_db();
    let book = create_sample_book();

    let book_id = db.add_book(&book).expect("Failed to add book");

    // Verify book exists
    let filter = BookFilter {
        id: Some(book_id),
        year: None,
    };
    let books = db.get_books(&filter).expect("Failed to get books");
    assert_eq!(books.len(), 1);

    // Delete book
    db.delete_book(book_id).expect("Failed to delete book");

    // Verify book is deleted
    let books = db
        .get_books(&filter)
        .expect("Failed to get books after deletion");
    assert_eq!(books.len(), 0);
}

#[test]
fn test_delete_review() {
    let mut db = create_test_db();
    let book = create_sample_book();

    let book_id = db.add_book(&book).expect("Failed to add book");
    let review = create_sample_review(book_id);

    let review_id = db.add_review(&review).expect("Failed to add review");

    // Verify review exists
    let reviews = db.get_reviews(book_id).expect("Failed to get reviews");
    assert_eq!(reviews.len(), 1);

    // Delete review
    db.delete_review(review_id)
        .expect("Failed to delete review");

    // Verify review is deleted
    let reviews = db
        .get_reviews(book_id)
        .expect("Failed to get reviews after deletion");
    assert_eq!(reviews.len(), 0);
}

#[test]
fn test_writer_deduplication() {
    let mut db = create_test_db();

    // Add first book with author "John Doe"
    let book1 = NewBook {
        title: "Book 1".to_string(),
        authors: vec!["John Doe".to_string()],
        translators: vec![],
        pages: None,
        pub_year: None,
        genre: None,
    };

    // Add second book with same author "John Doe"
    let book2 = NewBook {
        title: "Book 2".to_string(),
        authors: vec!["John Doe".to_string()],
        translators: vec![],
        pages: None,
        pub_year: None,
        genre: None,
    };

    let book1_id = db.add_book(&book1).expect("Failed to add book 1");
    let book2_id = db.add_book(&book2).expect("Failed to add book 2");

    let writers1 = db
        .get_book_writers(book1_id)
        .expect("Failed to get writers for book 1");
    let writers2 = db
        .get_book_writers(book2_id)
        .expect("Failed to get writers for book 2");

    // Both books should have the same author ID (writer deduplication)
    assert_eq!(writers1.len(), 1);
    assert_eq!(writers2.len(), 1);
    assert_eq!(writers1[0].id, writers2[0].id);
    assert_eq!(writers1[0].name, "John Doe");
    assert_eq!(writers2[0].name, "John Doe");
}

#[test]
fn test_foreign_key_constraints() {
    let mut db = create_test_db();
    let book = create_sample_book();

    let book_id = db.add_book(&book).expect("Failed to add book");
    let review = create_sample_review(book_id);

    db.add_review(&review).expect("Failed to add review");

    // Try to delete the book - this should fail due to foreign key constraint
    let result = db.delete_book(book_id);
    assert!(result.is_err());

    // The error should be a database error (foreign key constraint)
    match result.unwrap_err() {
        LibroError::Database(_) => {
            // This is expected - foreign key constraint prevents deletion
            assert!(true);
        }
        _ => {
            panic!("Expected database error due to foreign key constraint");
        }
    }

    // Verify book still exists
    let filter = BookFilter {
        id: Some(book_id),
        year: None,
    };
    let books = db.get_books(&filter).expect("Failed to get books");
    assert_eq!(books.len(), 1);

    // Verify review still exists
    let reviews = db.get_reviews(book_id).expect("Failed to get reviews");
    assert_eq!(reviews.len(), 1);
}
