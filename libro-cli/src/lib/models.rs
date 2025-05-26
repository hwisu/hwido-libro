use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

/// Book entity representing a book in the database
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Book {
    pub id: Option<i64>,
    pub title: String,
    pub pages: Option<i32>,
    pub pub_year: Option<i32>,
    pub genre: Option<String>,
}

/// Review entity for book reviews
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Review {
    pub id: Option<i64>,
    pub book_id: i64,
    pub date_read: Option<NaiveDate>,
    pub rating: i32,
    pub review: String,
}

/// Writer entity for authors and translators
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Writer {
    pub id: Option<i64>,
    pub name: String,
    pub writer_type: WriterType,
}

/// Type of writer (author or translator)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WriterType {
    Author,
    Translator,
}

impl WriterType {
    pub fn as_str(&self) -> &'static str {
        match self {
            WriterType::Author => "author",
            WriterType::Translator => "translator",
        }
    }

    #[allow(clippy::should_implement_trait)]
    pub fn from_str(s: &str) -> Result<Self, String> {
        match s {
            "author" => Ok(WriterType::Author),
            "translator" => Ok(WriterType::Translator),
            _ => Err(format!("Invalid writer type: {}", s)),
        }
    }
}

impl std::str::FromStr for WriterType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        WriterType::from_str(s)
    }
}

/// Book-Writer relationship entity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BookWriter {
    pub book_id: i64,
    pub writer_id: i64,
    pub writer_type: WriterType,
}

/// Extended book with associated writers and reviews
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtendedBook {
    #[serde(flatten)]
    pub book: Book,
    pub authors: Vec<Writer>,
    pub translators: Vec<Writer>,
    pub reviews: Vec<Review>,
}

/// Input struct for creating a new book
#[derive(Debug, Clone)]
pub struct NewBook {
    pub title: String,
    pub authors: Vec<String>,
    pub translators: Vec<String>,
    pub pages: Option<i32>,
    pub pub_year: Option<i32>,
    pub genre: Option<String>,
}

/// Input struct for creating a new review
#[derive(Debug, Clone)]
pub struct NewReview {
    pub book_id: i64,
    pub date_read: Option<NaiveDate>,
    pub rating: i32,
    pub review: String,
}

/// Combined input for creating a book with an optional review
#[derive(Debug, Clone)]
pub struct NewBookWithReview {
    pub book: NewBook,
    pub review: Option<NewReview>,
}

/// Filter options for querying books
#[derive(Debug, Clone, Default)]
pub struct BookFilter {
    pub id: Option<i64>,
    pub year: Option<i32>,
}

/// Result of adding a book with optional review
#[derive(Debug, Clone)]
pub struct BookCreationResult {
    pub book_id: i64,
    pub review_id: Option<i64>,
}
