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
    /// Show book(s) with reviews by id or year
    Brs {
        /// 조회할 책 ID
        id: Option<u32>,
        /// 연도로 조회
        #[arg(long)]
        year: Option<u32>,
        /// JSON 형식 출력
        #[arg(long)]
        json: bool,
    },
    /// Generate reading reports
    Report {
        /// 작가별 통계
        #[arg(long)]
        author: bool,
        /// 연도별 필터
        #[arg(long)]
        year: Option<u32>,
        /// 연도별 차트
        #[arg(long)]
        years: bool,
    },
    /// Add or edit a review for a book
    Review {
        /// 리뷰할 책 ID
        id: u32,
    },
    /// Edit a book review using system editor
    EditReview {
        /// 수정할 리뷰의 책 ID
        id: u32,
    },
    /// Show latest books summary
    Books {
        /// Number of books to show
        #[arg(short, long, default_value = "10")]
        limit: u32,
    },
    /// Show latest reviews summary
    Reviews {
        /// Number of reviews to show
        #[arg(short, long, default_value = "10")]
        limit: u32,
    },
    /// Show authors summary
    Authors {
        /// Number of authors to show
        #[arg(short, long, default_value = "10")]
        limit: u32,
    },
}

fn main() {
    let cli = Cli::parse();

    let result = match cli.command {
        Commands::Add => commands::add::run(),
        Commands::Brs { id, year, json } => commands::brs::run(id, year, json),
        Commands::Report {
            author,
            year,
            years,
        } => commands::report::run(author, year, years),
        Commands::Review { id } => commands::review::run(id),
        Commands::EditReview { id } => commands::edit_review::run(id),
        Commands::Books { limit } => commands::books::run(limit),
        Commands::Reviews { limit } => commands::reviews::run(limit),
        Commands::Authors { limit } => commands::authors::run(limit),
    };

    handle_result(result);
}
