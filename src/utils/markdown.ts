/**
 * 마크다운 파일에서 책 리뷰 데이터를 파싱하는 유틸리티
 *
 * 지원하는 마크다운 형식:
 * ```
 * # 책 제목
 *
 * 작가: 작가 이름
 * 장르: Fiction|Nonfiction
 * 출판년도: 2023
 * 페이지: 320
 * 읽은 날짜: 2023-04-15
 * 평점: 4.5
 *
 * ## 리뷰
 *
 * 책에 대한 리뷰 내용...
 * ```
 */

// 책 리뷰 인터페이스
export interface BookReview {
  title?: string;
  author?: string;
  genre?: string;
  pub_year?: number;
  pages?: number;
  date_read?: string;
  rating?: number;
  review?: string;
}

/**
 * 마크다운 파일에서 도서 리뷰 데이터를 추출합니다
 */
export function parseMarkdownReview(markdown: string): BookReview {
  const result: BookReview = {};

  // 제목 추출 (# 으로 시작하는 첫 번째 줄)
  const titleMatch = markdown.match(/^#\s+(.+)$/m);
  if (titleMatch) {
    result.title = titleMatch[1].trim();
  }

  // 메타데이터 추출 (key: value 형식)
  const authorMatch = markdown.match(/작가:\s*(.+)$/m);
  if (authorMatch) {
    result.author = authorMatch[1].trim();
  }

  const genreMatch = markdown.match(/장르:\s*(.+)$/m);
  if (genreMatch) {
    result.genre = genreMatch[1].trim();
  }

  const yearMatch = markdown.match(/출판년도:\s*(\d+)$/m);
  if (yearMatch) {
    result.pub_year = parseInt(yearMatch[1]);
  }

  const pagesMatch = markdown.match(/페이지:\s*(\d+)$/m);
  if (pagesMatch) {
    result.pages = parseInt(pagesMatch[1]);
  }

  const dateMatch = markdown.match(/읽은 날짜:\s*(.+)$/m);
  if (dateMatch) {
    result.date_read = dateMatch[1].trim();
  }

  const ratingMatch = markdown.match(/평점:\s*(\d+(\.\d+)?)$/m);
  if (ratingMatch) {
    result.rating = parseFloat(ratingMatch[1]);
  }

  // 리뷰 추출 (## 리뷰 이후의 모든 텍스트)
  const reviewMatch = markdown.match(/##\s+리뷰\s*\n\s*(.+(?:\n.+)*)/);
  if (reviewMatch) {
    result.review = reviewMatch[1].trim();
  }

  return result;
}

/**
 * 책 리뷰 데이터를 마크다운 형식으로 변환합니다
 */
export function bookReviewToMarkdown(review: BookReview): string {
  let md = "";

  // 제목
  if (review.title) {
    md += `# ${review.title}\n\n`;
  }

  // 메타데이터
  if (review.author) {
    md += `작가: ${review.author}\n`;
  }

  if (review.genre) {
    md += `장르: ${review.genre}\n`;
  }

  if (review.pub_year) {
    md += `출판년도: ${review.pub_year}\n`;
  }

  if (review.pages) {
    md += `페이지: ${review.pages}\n`;
  }

  if (review.date_read) {
    md += `읽은 날짜: ${review.date_read}\n`;
  }

  if (review.rating) {
    md += `평점: ${review.rating}\n`;
  }

  md += "\n";

  // 리뷰 내용
  if (review.review) {
    md += `## 리뷰\n\n${review.review}\n`;
  }

  return md;
}

/**
 * 파일 경로에서 책 ID를 추출합니다 (파일명이 ID.md 형식인 경우)
 */
export function getBookIdFromPath(path: string): number | null {
  const match = path.match(/\/(\d+)\.md$/);
  if (match) {
    return parseInt(match[1]);
  }
  return null;
}

/**
 * 책 제목에서 파일명에 사용할 수 있는 슬러그를 생성합니다
 */
export function slugify(title: string): string {
  return title
    .toLowerCase()
    .replace(/[^\w\s-]/g, "") // 영숫자, 언더스코어, 하이픈 및 공백만 유지
    .replace(/\s+/g, "-") // 공백을 하이픈으로 변환
    .replace(/-+/g, "-"); // 연속된 하이픈 단일화
}
