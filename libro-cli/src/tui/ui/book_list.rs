//! 도서 목록 화면 UI

use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
    Frame,
};

use crate::lib::models::ExtendedBook;
use crate::tui::state::AppState;

/// 도서 목록 화면을 렌더링합니다
pub fn render_book_list(f: &mut Frame, area: Rect, state: &AppState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // 헤더
            Constraint::Min(0),    // 도서 목록
            Constraint::Length(3), // 상태바
        ])
        .split(area);

    // 헤더
    render_header(f, chunks[0], state);

    // 도서 목록
    render_books(f, chunks[1], state);

    // 상태바
    render_status_bar(f, chunks[2], state);
}

/// 헤더를 렌더링합니다
fn render_header(f: &mut Frame, area: Rect, state: &AppState) {
    let title = format!("📚 Libro - 도서 목록 ({} 권)", state.books.len());
    let header = Paragraph::new(title)
        .block(Block::default().borders(Borders::ALL))
        .style(
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        );

    f.render_widget(header, area);
}

/// 도서 목록을 렌더링합니다
fn render_books(f: &mut Frame, area: Rect, state: &AppState) {
    if state.books.is_empty() {
        render_empty_list(f, area);
        return;
    }

    let items: Vec<ListItem> = state
        .books
        .iter()
        .enumerate()
        .map(|(i, book)| {
            let is_selected = i == state.selected_book_index;
            create_book_item(book, is_selected)
        })
        .collect();

    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title("도서 목록"))
        .highlight_style(
            Style::default()
                .bg(Color::DarkGray)
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        );

    let mut list_state = ListState::default();
    list_state.select(Some(state.selected_book_index));

    f.render_stateful_widget(list, area, &mut list_state);
}

/// 빈 목록을 렌더링합니다
fn render_empty_list(f: &mut Frame, area: Rect) {
    let empty_msg =
        Paragraph::new("📖 등록된 도서가 없습니다.\n\n'a' 키를 눌러 도서를 추가해보세요!")
            .block(Block::default().borders(Borders::ALL).title("도서 목록"))
            .style(Style::default().fg(Color::Gray));

    f.render_widget(empty_msg, area);
}

/// 개별 도서 아이템을 생성합니다
pub fn create_book_item(book: &ExtendedBook, is_selected: bool) -> ListItem {
    let authors = book
        .authors
        .iter()
        .map(|a| a.name.clone())
        .collect::<Vec<_>>()
        .join(", ");

    let translators = if !book.translators.is_empty() {
        format!(
            " (번역: {})",
            book.translators
                .iter()
                .map(|t| t.name.clone())
                .collect::<Vec<_>>()
                .join(", ")
        )
    } else {
        String::new()
    };

    let year_info = book
        .book
        .pub_year
        .map(|y| format!(" ({})", y))
        .unwrap_or_default();

    let pages_info = book
        .book
        .pages
        .map(|p| format!(" - {}p", p))
        .unwrap_or_default();

    let review_count = book.reviews.len();
    let review_info = if review_count > 0 {
        format!(" [리뷰 {}개]", review_count)
    } else {
        String::new()
    };

    let style = if is_selected {
        Style::default()
            .fg(Color::Yellow)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default()
    };

    let content = vec![
        Line::from(vec![
            Span::styled(&book.book.title, style.add_modifier(Modifier::BOLD)),
            Span::styled(year_info, Style::default().fg(Color::Gray)),
            Span::styled(pages_info, Style::default().fg(Color::Gray)),
            Span::styled(review_info, Style::default().fg(Color::Green)),
        ]),
        Line::from(vec![Span::styled(
            format!("  저자: {}{}", authors, translators),
            Style::default().fg(Color::Cyan),
        )]),
        Line::from(vec![Span::styled(
            format!("  장르: {}", book.book.genre),
            Style::default().fg(Color::Magenta),
        )]),
    ];

    ListItem::new(content)
}

/// 상태바를 렌더링합니다
fn render_status_bar(f: &mut Frame, area: Rect, state: &AppState) {
    let status_text = if let Some(error) = &state.error_message {
        format!("❌ 오류: {}", error)
    } else if state.books.is_empty() {
        "💡 도서를 추가하려면 'a' 키를 누르세요".to_string()
    } else {
        format!(
            "📍 {}/{} | j/k: 이동 | a: 추가 | e: 편집 | d: 삭제 | v: 리뷰 | /: 검색 | ?: 도움말",
            state.selected_book_index + 1,
            state.books.len()
        )
    };

    let status_style = if state.error_message.is_some() {
        Style::default().fg(Color::Red)
    } else {
        Style::default().fg(Color::Green)
    };

    let status = Paragraph::new(status_text)
        .block(Block::default().borders(Borders::ALL))
        .style(status_style);

    f.render_widget(status, area);
}
