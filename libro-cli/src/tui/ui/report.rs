//! 리포트 화면 UI

use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph, Wrap},
    Frame,
};
use std::collections::HashMap;

use crate::lib::models::ExtendedBook;
use crate::tui::state::AppState;
use chrono::Datelike;

#[derive(Debug, Clone, PartialEq)]
pub enum ReportView {
    Authors, // 작가별 통계 (1키)
    Years,   // 연도별 통계 (2키)
    Recent,  // 최근 도서 목록 (3키)
}

impl Default for ReportView {
    fn default() -> Self {
        ReportView::Authors
    }
}

/// 리포트 화면을 렌더링합니다
pub fn render_report(f: &mut Frame, area: Rect, state: &AppState, current_view: &ReportView) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // 헤더
            Constraint::Min(0),    // 리포트 내용
            Constraint::Length(3), // 상태바
        ])
        .split(area);

    // 헤더
    render_header(f, chunks[0], current_view);

    // 리포트 내용
    match current_view {
        ReportView::Authors => render_authors_report(f, chunks[1], &state.books),
        ReportView::Years => render_years_report(f, chunks[1], &state.books),
        ReportView::Recent => render_recent_books_report(f, chunks[1], &state.books),
    }

    // 상태바
    render_status_bar(f, chunks[2], current_view);
}

/// 헤더를 렌더링합니다
fn render_header(f: &mut Frame, area: Rect, current_view: &ReportView) {
    let title = match current_view {
        ReportView::Authors => "📊 리포트 - 작가별 통계",
        ReportView::Years => "📊 리포트 - 연도별 통계",
        ReportView::Recent => "📊 리포트 - 최근 도서",
    };

    let header = Paragraph::new(title)
        .block(Block::default().borders(Borders::ALL))
        .style(
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        );

    f.render_widget(header, area);
}

/// 작가별 통계를 렌더링합니다
fn render_authors_report(f: &mut Frame, area: Rect, books: &[ExtendedBook]) {
    if books.is_empty() {
        let content = Paragraph::new("📚 도서가 없습니다.")
            .block(Block::default().borders(Borders::ALL).title("작가별 통계"))
            .style(Style::default().fg(Color::Gray));
        f.render_widget(content, area);
        return;
    }

    // 작가별 통계 계산 (도서 수와 가장 최근 도서 ID 추적)
    let mut author_stats: HashMap<String, (usize, i64)> = HashMap::new(); // (도서 수, 최근 도서 ID)
    for book in books {
        if let Some(book_id) = book.book.id {
            for author in &book.authors {
                let entry = author_stats.entry(author.name.clone()).or_insert((0, 0));
                entry.0 += 1; // 도서 수 증가
                entry.1 = entry.1.max(book_id); // 가장 최근 도서 ID 업데이트
            }
        }
    }

    let mut sorted_authors: Vec<_> = author_stats.into_iter().collect();
    // 가장 최근 도서 ID 기준으로 내림차순 정렬 (최근 추가된 작가 우선)
    sorted_authors.sort_by(|a, b| b.1 .1.cmp(&a.1 .1));
    sorted_authors.truncate(10); // 상위 10명만 표시

    let mut lines = vec![
        Line::from(vec![Span::styled(
            format!("👥 상위 {} 작가", sorted_authors.len()),
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from(""),
    ];

    for (i, (author_name, (book_count, _latest_id))) in sorted_authors.iter().enumerate() {
        // 작가 이름과 도서 수
        lines.push(Line::from(vec![
            Span::styled(format!("{:2}. ", i + 1), Style::default().fg(Color::Gray)),
            Span::styled(
                author_name.clone(),
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            ),
        ]));

        lines.push(Line::from(vec![
            Span::raw("    "),
            Span::styled(
                format!("📚 {} 권", book_count),
                Style::default().fg(Color::Blue),
            ),
        ]));

        // 해당 작가의 도서 목록 (최대 3권)
        let author_books: Vec<_> = books
            .iter()
            .filter(|book| book.authors.iter().any(|a| a.name == *author_name))
            .take(3)
            .collect();

        for (j, book) in author_books.iter().enumerate() {
            let prefix = if j == author_books.len() - 1 && author_books.len() < *book_count {
                "    └─ "
            } else {
                "    ├─ "
            };
            lines.push(Line::from(vec![
                Span::styled(prefix, Style::default().fg(Color::Gray)),
                Span::styled(&book.book.title, Style::default().fg(Color::Gray)),
            ]));
        }

        if author_books.len() < *book_count {
            lines.push(Line::from(vec![
                Span::styled("    └─ ", Style::default().fg(Color::Gray)),
                Span::styled(
                    format!("외 {} 권...", book_count - author_books.len()),
                    Style::default().fg(Color::Gray),
                ),
            ]));
        }

        if i < sorted_authors.len() - 1 {
            lines.push(Line::from(""));
        }
    }

    let content = Paragraph::new(lines)
        .block(Block::default().borders(Borders::ALL).title("작가별 통계"))
        .wrap(Wrap { trim: true });

    f.render_widget(content, area);
}

/// 연도별 통계를 렌더링합니다
fn render_years_report(f: &mut Frame, area: Rect, books: &[ExtendedBook]) {
    if books.is_empty() {
        let content = Paragraph::new("📚 도서가 없습니다.")
            .block(Block::default().borders(Borders::ALL).title("연도별 통계"))
            .style(Style::default().fg(Color::Gray));
        f.render_widget(content, area);
        return;
    }

    // 기본 통계
    let total_books = books.len();
    let total_pages: i32 = books.iter().filter_map(|b| b.book.pages).sum();
    let total_reviews = books.iter().map(|b| b.reviews.len()).sum::<usize>();

    let avg_rating = if total_reviews > 0 {
        let total_rating: i32 = books
            .iter()
            .flat_map(|b| &b.reviews)
            .map(|r| r.rating)
            .sum();
        total_rating as f32 / total_reviews as f32
    } else {
        0.0
    };

    // 읽은 날짜 기준 연도별 통계
    let mut year_counts = HashMap::new();
    for book in books {
        for review in &book.reviews {
            if let Some(date_read) = review.date_read {
                let read_year = date_read.year();
                *year_counts.entry(read_year).or_insert(0) += 1;
            }
        }
    }

    let mut lines = vec![
        Line::from(vec![Span::styled(
            "📊 독서 통계",
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from(""),
        Line::from(vec![
            Span::raw("총 도서: "),
            Span::styled(
                format!("{} 권", total_books),
                Style::default()
                    .fg(Color::Blue)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::raw("총 페이지: "),
            Span::styled(
                format!("{} 페이지", total_pages),
                Style::default()
                    .fg(Color::Blue)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::raw("총 리뷰: "),
            Span::styled(
                format!("{} 개", total_reviews),
                Style::default()
                    .fg(Color::Blue)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
    ];

    if total_reviews > 0 {
        lines.push(Line::from(vec![
            Span::raw("평균 평점: "),
            Span::styled(
                format!("{:.1}/5", avg_rating),
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            ),
        ]));
    }

    if !year_counts.is_empty() {
        lines.push(Line::from(""));
        lines.push(Line::from(vec![Span::styled(
            "📅 연도별 독서 현황 (읽은 날짜 기준)",
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD),
        )]));
        lines.push(Line::from(""));

        let mut years: Vec<_> = year_counts.iter().collect();
        years.sort_by_key(|(year, _)| *year);

        for (year, count) in years {
            let bar = "█".repeat(*count);
            lines.push(Line::from(vec![
                Span::styled(format!("{}: ", year), Style::default().fg(Color::White)),
                Span::styled(bar, Style::default().fg(Color::Green)),
                Span::styled(format!(" ({} 권)", count), Style::default().fg(Color::Gray)),
            ]));
        }
    } else {
        lines.push(Line::from(""));
        lines.push(Line::from(vec![Span::styled(
            "📅 읽은 날짜가 기록된 도서가 없습니다.",
            Style::default().fg(Color::Gray),
        )]));
    }

    let content = Paragraph::new(lines)
        .block(Block::default().borders(Borders::ALL).title("연도별 통계"))
        .wrap(Wrap { trim: true });

    f.render_widget(content, area);
}

/// 최근 도서 목록을 렌더링합니다
fn render_recent_books_report(f: &mut Frame, area: Rect, books: &[ExtendedBook]) {
    if books.is_empty() {
        let content = Paragraph::new("📚 도서가 없습니다.")
            .block(Block::default().borders(Borders::ALL).title("최근 도서"))
            .style(Style::default().fg(Color::Gray));
        f.render_widget(content, area);
        return;
    }

    let mut sorted_books = books.to_vec();
    sorted_books.sort_by(|a, b| b.book.id.cmp(&a.book.id));
    sorted_books.truncate(10); // 최근 10권만 표시

    let mut lines = vec![
        Line::from(vec![Span::styled(
            format!("📚 최근 {} 권", sorted_books.len()),
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from(""),
    ];

    for (i, book) in sorted_books.iter().enumerate() {
        let authors: Vec<String> = book.authors.iter().map(|a| a.name.clone()).collect();

        // 도서 제목과 저자
        lines.push(Line::from(vec![
            Span::styled(format!("{:2}. ", i + 1), Style::default().fg(Color::Gray)),
            Span::styled(
                &book.book.title,
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            ),
        ]));

        lines.push(Line::from(vec![
            Span::raw("    "),
            Span::styled("저자: ", Style::default().fg(Color::Gray)),
            Span::styled(authors.join(", "), Style::default().fg(Color::Blue)),
        ]));

        // 추가 정보
        let mut info_parts = Vec::new();
        if let Some(year) = book.book.pub_year {
            info_parts.push(format!("📅 {}", year));
        }
        if let Some(pages) = book.book.pages {
            info_parts.push(format!("📄 {} 페이지", pages));
        }
        info_parts.push(format!("🏷️ {}", &book.book.genre));

        if !info_parts.is_empty() {
            lines.push(Line::from(vec![
                Span::raw("    "),
                Span::styled(info_parts.join(" • "), Style::default().fg(Color::Gray)),
            ]));
        }

        // 리뷰 정보
        if !book.reviews.is_empty() {
            let avg_rating = book.reviews.iter().map(|r| r.rating).sum::<i32>() as f32
                / book.reviews.len() as f32;
            let stars = "⭐".repeat(avg_rating.round() as usize);

            lines.push(Line::from(vec![
                Span::raw("    "),
                Span::styled(stars, Style::default().fg(Color::Yellow)),
                Span::styled(
                    format!(" {:.1}/5 ({} 리뷰)", avg_rating, book.reviews.len()),
                    Style::default().fg(Color::Gray),
                ),
            ]));
        }

        if i < sorted_books.len() - 1 {
            lines.push(Line::from(""));
        }
    }

    let content = Paragraph::new(lines)
        .block(Block::default().borders(Borders::ALL).title("최근 도서"))
        .wrap(Wrap { trim: true });

    f.render_widget(content, area);
}

/// 상태바를 렌더링합니다
fn render_status_bar(f: &mut Frame, area: Rect, current_view: &ReportView) {
    let status_text = match current_view {
        ReportView::Authors => "💡 1: 작가 통계 | 2: 연도 통계 | 3: 최근 도서 | Esc: 뒤로가기",
        ReportView::Years => "💡 1: 작가 통계 | 2: 연도 통계 | 3: 최근 도서 | Esc: 뒤로가기",
        ReportView::Recent => "💡 1: 작가 통계 | 2: 연도 통계 | 3: 최근 도서 | Esc: 뒤로가기",
    };

    let status_bar = Paragraph::new(status_text)
        .block(Block::default().borders(Borders::ALL))
        .style(Style::default().fg(Color::Gray));

    f.render_widget(status_bar, area);
}
