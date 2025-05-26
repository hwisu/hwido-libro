use clap::{Parser, Subcommand};

mod cli;
mod lib;
mod tui;
mod utils;

#[derive(Parser)]
#[command(
    name = "libro-cli",
    version,
    about = "A command-line book tracking tool with data stored in SQLite"
)]
struct Args {
    /// Use CLI mode instead of TUI
    #[arg(long)]
    cli: bool,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Add a new book
    Add,
    /// Browse and search books
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
    let args = Args::parse();

    let result = if args.cli || args.command.is_some() {
        // CLI 모드: --cli 플래그가 있거나 서브커맨드가 제공된 경우
        run_cli_mode(args.command)
    } else {
        // TUI 모드: 기본 모드
        tui::run_tui()
    };

    if let Err(e) = result {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

fn run_cli_mode(command: Option<Commands>) -> Result<(), Box<dyn std::error::Error>> {
    use utils::error_handler::handle_result;

    let command = command.unwrap_or(Commands::Browse {
        query: None,
        year: None,
        json: false,
    });

    let result = match command {
        Commands::Add => cli::commands::add::run(),
        Commands::Browse { query, year, json } => cli::commands::browse::run(query, year, json),
        Commands::Report {
            authors,
            books,
            reviews,
            year,
            years,
            limit,
        } => cli::commands::report::run(authors, books, reviews, year, years, limit),
        Commands::Review { id } => cli::commands::review::run(id),
    };

    handle_result(result);
    Ok(())
}
