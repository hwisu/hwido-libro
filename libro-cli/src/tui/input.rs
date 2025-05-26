//! 텍스트 입력 위젯
//! Phase 3에서 구현 예정

// TODO: 텍스트 입력 위젯 구현
// - 커서 위치 관리
// - 텍스트 삽입/삭제
// - 멀티라인 지원
// - 유니코드 문자 지원

use ratatui::{
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Paragraph, Wrap},
    Frame,
};
use unicode_width::{UnicodeWidthChar, UnicodeWidthStr};

#[derive(Debug, Clone)]
pub struct TextInput {
    /// 입력된 텍스트 (멀티라인)
    lines: Vec<String>,
    /// 현재 커서 위치 (line, column)
    cursor: (usize, usize),
    /// 스크롤 오프셋 (표시할 첫 번째 라인)
    scroll_offset: usize,
    /// 최대 표시 가능한 라인 수
    max_visible_lines: usize,
    /// 편집 가능 여부
    editable: bool,
}

impl Default for TextInput {
    fn default() -> Self {
        Self {
            lines: vec![String::new()],
            cursor: (0, 0),
            scroll_offset: 0,
            max_visible_lines: 10,
            editable: true,
        }
    }
}

impl TextInput {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_text(text: String) -> Self {
        let lines: Vec<String> = if text.is_empty() {
            vec![String::new()]
        } else {
            text.lines().map(|s| s.to_string()).collect()
        };

        let cursor_line = lines.len().saturating_sub(1);
        let cursor_col = lines.last().map(|l| l.chars().count()).unwrap_or(0);

        Self {
            lines,
            cursor: (cursor_line, cursor_col),
            scroll_offset: 0,
            max_visible_lines: 10,
            editable: true,
        }
    }

    pub fn set_editable(&mut self, editable: bool) {
        self.editable = editable;
    }

    pub fn get_text(&self) -> String {
        self.lines.join("\n")
    }

    pub fn clear(&mut self) {
        self.lines = vec![String::new()];
        self.cursor = (0, 0);
        self.scroll_offset = 0;
    }

    pub fn insert_char(&mut self, c: char) {
        if !self.editable {
            return;
        }

        let (line_idx, col_idx) = self.cursor;

        // 라인이 존재하지 않으면 생성
        while self.lines.len() <= line_idx {
            self.lines.push(String::new());
        }

        // 현재 라인 가져오기
        let line = &mut self.lines[line_idx];
        let chars: Vec<char> = line.chars().collect();

        // 문자 삽입
        let mut new_chars = chars;
        new_chars.insert(col_idx, c);
        *line = new_chars.into_iter().collect();

        // 커서 이동
        self.cursor.1 += 1;
        self.adjust_scroll();
    }

    pub fn insert_newline(&mut self) {
        if !self.editable {
            return;
        }

        let (line_idx, col_idx) = self.cursor;

        // 현재 라인 분할
        let current_line = self.lines.get(line_idx).cloned().unwrap_or_default();
        let chars: Vec<char> = current_line.chars().collect();

        let left: String = chars[..col_idx].iter().collect();
        let right: String = chars[col_idx..].iter().collect();

        self.lines[line_idx] = left;
        self.lines.insert(line_idx + 1, right);

        // 커서를 다음 라인 시작으로 이동
        self.cursor = (line_idx + 1, 0);
        self.adjust_scroll();
    }

    pub fn delete_char(&mut self) {
        if !self.editable {
            return;
        }

        let (line_idx, col_idx) = self.cursor;

        if line_idx < self.lines.len() {
            let mut chars: Vec<char> = self.lines[line_idx].chars().collect();

            if col_idx < chars.len() {
                // 현재 위치의 문자 삭제
                chars.remove(col_idx);
                self.lines[line_idx] = chars.into_iter().collect();
            } else if line_idx + 1 < self.lines.len() {
                // 라인 끝에서 다음 라인과 합치기
                let next_line = self.lines.remove(line_idx + 1);
                self.lines[line_idx].push_str(&next_line);
            }
        }
    }

    pub fn backspace(&mut self) {
        if !self.editable {
            return;
        }

        let (line_idx, col_idx) = self.cursor;

        if col_idx > 0 {
            // 현재 라인에서 이전 문자 삭제
            if let Some(line) = self.lines.get_mut(line_idx) {
                let mut chars: Vec<char> = line.chars().collect();
                chars.remove(col_idx - 1);
                *line = chars.into_iter().collect();
                self.cursor.1 -= 1;
            }
        } else if line_idx > 0 {
            // 라인 시작에서 이전 라인과 합치기
            let current_line = self.lines.remove(line_idx);
            let prev_line_len = self.lines[line_idx - 1].chars().count();
            self.lines[line_idx - 1].push_str(&current_line);
            self.cursor = (line_idx - 1, prev_line_len);
        }

        self.adjust_scroll();
    }

    pub fn move_cursor_left(&mut self) {
        if self.cursor.1 > 0 {
            self.cursor.1 -= 1;
        } else if self.cursor.0 > 0 {
            self.cursor.0 -= 1;
            self.cursor.1 = self
                .lines
                .get(self.cursor.0)
                .map(|l| l.chars().count())
                .unwrap_or(0);
        }
        self.adjust_scroll();
    }

    pub fn move_cursor_right(&mut self) {
        let (line_idx, col_idx) = self.cursor;
        let line_len = self
            .lines
            .get(line_idx)
            .map(|l| l.chars().count())
            .unwrap_or(0);

        if col_idx < line_len {
            self.cursor.1 += 1;
        } else if line_idx + 1 < self.lines.len() {
            self.cursor = (line_idx + 1, 0);
        }
        self.adjust_scroll();
    }

    pub fn move_cursor_up(&mut self) {
        if self.cursor.0 > 0 {
            self.cursor.0 -= 1;
            let line_len = self
                .lines
                .get(self.cursor.0)
                .map(|l| l.chars().count())
                .unwrap_or(0);
            self.cursor.1 = self.cursor.1.min(line_len);
        }
        self.adjust_scroll();
    }

    pub fn move_cursor_down(&mut self) {
        if self.cursor.0 + 1 < self.lines.len() {
            self.cursor.0 += 1;
            let line_len = self
                .lines
                .get(self.cursor.0)
                .map(|l| l.chars().count())
                .unwrap_or(0);
            self.cursor.1 = self.cursor.1.min(line_len);
        }
        self.adjust_scroll();
    }

    pub fn move_to_line_start(&mut self) {
        self.cursor.1 = 0;
    }

    pub fn move_to_line_end(&mut self) {
        let line_len = self
            .lines
            .get(self.cursor.0)
            .map(|l| l.chars().count())
            .unwrap_or(0);
        self.cursor.1 = line_len;
    }

    pub fn clear_current_line(&mut self) {
        if !self.editable {
            return;
        }

        if let Some(line) = self.lines.get_mut(self.cursor.0) {
            line.clear();
            self.cursor.1 = 0;
        }
    }

    pub fn delete_to_line_end(&mut self) {
        if !self.editable {
            return;
        }

        let (line_idx, col_idx) = self.cursor;
        if let Some(line) = self.lines.get_mut(line_idx) {
            let chars: Vec<char> = line.chars().collect();
            let new_line: String = chars[..col_idx].iter().collect();
            *line = new_line;
        }
    }

    pub fn delete_word_backward(&mut self) {
        if !self.editable {
            return;
        }

        let (line_idx, col_idx) = self.cursor;
        if let Some(line) = self.lines.get_mut(line_idx) {
            if col_idx == 0 {
                return;
            }

            let chars: Vec<char> = line.chars().collect();
            let mut new_col = col_idx;

            // 공백 건너뛰기
            while new_col > 0 && chars[new_col - 1].is_whitespace() {
                new_col -= 1;
            }

            // 단어 삭제
            while new_col > 0 && !chars[new_col - 1].is_whitespace() {
                new_col -= 1;
            }

            let new_line: String = chars[..new_col]
                .iter()
                .chain(chars[col_idx..].iter())
                .collect();
            *line = new_line;
            self.cursor.1 = new_col;
        }
    }

    pub fn render(&mut self, f: &mut Frame, area: Rect, block: Block, show_cursor: bool) {
        let inner = block.inner(area);
        self.max_visible_lines = inner.height as usize;

        // 표시할 라인들 계산
        let visible_lines: Vec<Line> = self
            .lines
            .iter()
            .skip(self.scroll_offset)
            .take(self.max_visible_lines)
            .enumerate()
            .map(|(i, line)| {
                let line_idx = self.scroll_offset + i;
                if show_cursor && line_idx == self.cursor.0 {
                    self.render_line_with_cursor(line)
                } else {
                    Line::from(line.clone())
                }
            })
            .collect();

        let paragraph = Paragraph::new(visible_lines)
            .block(block)
            .wrap(Wrap { trim: false });

        f.render_widget(paragraph, area);
    }

    fn render_line_with_cursor(&self, line: &str) -> Line {
        let chars: Vec<char> = line.chars().collect();
        let cursor_col = self.cursor.1;

        if cursor_col >= chars.len() {
            // 커서가 라인 끝에 있는 경우
            let mut spans = vec![Span::raw(line.to_string())];
            spans.push(Span::styled(
                " ",
                Style::default().bg(Color::White).fg(Color::Black),
            ));
            Line::from(spans)
        } else {
            // 커서가 문자 위에 있는 경우
            let before: String = chars[..cursor_col].iter().collect();
            let cursor_char = chars[cursor_col].to_string();
            let after: String = chars[cursor_col + 1..].iter().collect();

            let spans = vec![
                Span::raw(before),
                Span::styled(
                    cursor_char,
                    Style::default().bg(Color::White).fg(Color::Black),
                ),
                Span::raw(after),
            ];
            Line::from(spans)
        }
    }

    fn adjust_scroll(&mut self) {
        let cursor_line = self.cursor.0;

        // 커서가 화면 위쪽을 벗어난 경우
        if cursor_line < self.scroll_offset {
            self.scroll_offset = cursor_line;
        }

        // 커서가 화면 아래쪽을 벗어난 경우
        if cursor_line >= self.scroll_offset + self.max_visible_lines {
            self.scroll_offset = cursor_line.saturating_sub(self.max_visible_lines - 1);
        }
    }
}
