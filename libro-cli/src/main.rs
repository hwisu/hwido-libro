use clap::{Parser, Subcommand};
mod commands;
mod utils;

use utils::error_handler::handle_result;

#[derive(Parser)]
#[command(
    name = "libro-cli",
    version,
    about = "A command-line book tracking tool with data stored in SQLite"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Add a new book
    Add,
    /// Browse and search books (default command)
    Browse {
        /// Search query (title, author, or genre)
        query: Option<String>,
        /// Show only books from specific year
        #[arg(long)]
        year: Option<u32>,
        /// Show in JSON format
        #[arg(long)]
        json: bool,
    },
    /// Generate reading reports and summaries
    Report {
        /// 작가별 통계
        #[arg(long)]
        authors: bool,
        /// 최신 책 목록
        #[arg(long)]
        books: bool,
        /// 최신 리뷰 목록
        #[arg(long)]
        reviews: bool,
        /// 연도별 필터
        #[arg(long)]
        year: Option<u32>,
        /// 연도별 차트
        #[arg(long)]
        years: bool,
        /// 표시할 항목 수 (books, reviews, authors용)
        #[arg(short, long, default_value = "10")]
        limit: u32,
    },
    /// Add or edit a review for a book
    Review {
        /// 리뷰할 책 ID
        id: u32,
    },

}

fn main() {
    let cli = Cli::parse();

    let result = match cli.command {
        Commands::Add => commands::add::run(),
        Commands::Browse { query, year, json } => commands::browse::run(query, year, json),
        Commands::Report {
            authors,
            books,
            reviews,
            year,
            years,
            limit,
        } => commands::report::run(authors, books, reviews, year, years, limit),
        Commands::Review { id } => commands::review::run(id),
    };

    handle_result(result);
}
