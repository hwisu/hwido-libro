//! 리뷰 화면 UI

use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
    Frame,
};

use crate::tui::input::TextInput;
use crate::tui::state::{AppMode, AppState};

/// 리뷰 화면을 렌더링합니다
pub fn render_review(f: &mut Frame, area: Rect, state: &AppState, text_input: &mut TextInput) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // 헤더
            Constraint::Min(0),    // 리뷰 내용
            Constraint::Length(3), // 상태바
        ])
        .split(area);

    // 헤더
    render_header(f, chunks[0], state);

    // 리뷰 내용 영역
    if state.mode == AppMode::Edit {
        render_review_edit(f, chunks[1], text_input);
    } else {
        render_review_display(f, chunks[1], state);
    }

    // 상태바
    render_status_bar(f, chunks[2], state);
}

/// 헤더를 렌더링합니다
fn render_header(f: &mut Frame, area: Rect, state: &AppState) {
    let title = if let Some(book) = state.books.get(state.selected_book_index) {
        format!("📝 리뷰 작성 - {}", book.book.title)
    } else {
        "📝 리뷰 작성".to_string()
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

/// 편집 모드에서 리뷰 입력을 렌더링합니다
fn render_review_edit(f: &mut Frame, area: Rect, text_input: &mut TextInput) {
    let block = Block::default()
        .borders(Borders::ALL)
        .title("📝 리뷰 편집 (Ctrl+S: 저장, Ctrl+X: 취소)")
        .style(Style::default().fg(Color::Green));

    text_input.render(f, area, block, true);
}

/// 일반 모드에서 기존 리뷰들을 표시합니다
fn render_review_display(f: &mut Frame, area: Rect, state: &AppState) {
    let selected_book = state.books.get(state.selected_book_index);

    if let Some(book) = selected_book {
        if book.reviews.is_empty() {
            // 리뷰가 없는 경우
            let content =
                Paragraph::new("📝 아직 리뷰가 없습니다.\n\n'v' 키를 눌러 리뷰를 작성해보세요!")
                    .block(Block::default().borders(Borders::ALL).title("리뷰"))
                    .style(Style::default().fg(Color::Gray));
            f.render_widget(content, area);
        } else {
            // 기존 리뷰들 표시
            let review_items: Vec<ListItem> = book
                .reviews
                .iter()
                .enumerate()
                .map(|(i, review)| {
                    let stars = "⭐".repeat(review.rating as usize);
                    let date_str = review
                        .date_read
                        .map(|d| format!(" ({})", d))
                        .unwrap_or_default();

                    let header = format!("{}. {} {}/5{}", i + 1, stars, review.rating, date_str);
                    let content = if review.review.chars().count() > 100 {
                        let truncated: String = review.review.chars().take(97).collect();
                        format!("{}...", truncated)
                    } else {
                        review.review.clone()
                    };

                    ListItem::new(vec![
                        Line::from(Span::styled(
                            header,
                            Style::default()
                                .fg(Color::Yellow)
                                .add_modifier(Modifier::BOLD),
                        )),
                        Line::from(Span::styled(format!("  {}", content), Style::default())),
                        Line::from(""),
                    ])
                })
                .collect();

            let reviews_list = List::new(review_items)
                .block(Block::default().borders(Borders::ALL).title(format!(
                    "📝 리뷰 ({} 개) - j/k로 선택, v로 편집, d로 삭제",
                    book.reviews.len()
                )))
                .style(Style::default())
                .highlight_style(
                    Style::default()
                        .bg(Color::Blue)
                        .add_modifier(Modifier::BOLD),
                );

            // 선택된 리뷰 인덱스 설정
            let mut list_state = ListState::default();
            if state.selected_review_index < book.reviews.len() {
                list_state.select(Some(state.selected_review_index));
            }

            f.render_stateful_widget(reviews_list, area, &mut list_state);
        }
    } else {
        // 선택된 도서가 없는 경우
        let content = Paragraph::new("도서를 선택해주세요.")
            .block(Block::default().borders(Borders::ALL).title("리뷰"))
            .style(Style::default().fg(Color::Red));
        f.render_widget(content, area);
    }
}

/// 상태바를 렌더링합니다
fn render_status_bar(f: &mut Frame, area: Rect, state: &AppState) {
    let status_text = match state.mode {
        AppMode::Edit => {
            if state.editing_review_index.is_some() {
                "💡 편집 모드: 기존 리뷰를 수정 중입니다. 모든 전역 키가 무시됩니다"
            } else {
                "💡 편집 모드: 새 리뷰를 작성 중입니다. 모든 전역 키가 무시됩니다"
            }
        }
        AppMode::Normal => "💡 'v' 키를 눌러 리뷰를 작성하거나 편집하세요",
        _ => "리뷰 화면",
    };

    let status_style = match state.mode {
        AppMode::Edit => Style::default().fg(Color::Yellow),
        _ => Style::default().fg(Color::Cyan),
    };

    let status = Paragraph::new(status_text)
        .block(Block::default().borders(Borders::ALL))
        .style(status_style);

    f.render_widget(status, area);
}
