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

/**
 * 책 리뷰 정보를 위한 인터페이스
 */
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
 * 마크다운 텍스트에서 도서 리뷰 데이터를 추출합니다
 *
 * @param markdown 파싱할 마크다운 텍스트
 * @returns 파싱된 책 리뷰 데이터
 */
export function parseMarkdownReview(markdown: string): BookReview {
  const result: BookReview = {};

  // 제목 추출 (# 으로 시작하는 첫 번째 줄)
  const titleMatch = markdown.match(/^#\s+(.+)$/m);
  if (titleMatch) {
    result.title = titleMatch[1].trim();
  }

  // 메타데이터 추출 (key: value 형식)
  const metadataMatches = {
    author: markdown.match(/작가:\s*(.+)$/m),
    genre: markdown.match(/장르:\s*(.+)$/m),
    pubYear: markdown.match(/출판년도:\s*(\d+)$/m),
    pages: markdown.match(/페이지:\s*(\d+)$/m),
    dateRead: markdown.match(/읽은 날짜:\s*(.+)$/m),
    rating: markdown.match(/평점:\s*(\d+(\.\d+)?)$/m)
  };

  if (metadataMatches.author) result.author = metadataMatches.author[1].trim();
  if (metadataMatches.genre) result.genre = metadataMatches.genre[1].trim();
  if (metadataMatches.pubYear) result.pub_year = parseInt(metadataMatches.pubYear[1]);
  if (metadataMatches.pages) result.pages = parseInt(metadataMatches.pages[1]);
  if (metadataMatches.dateRead) result.date_read = metadataMatches.dateRead[1].trim();
  if (metadataMatches.rating) result.rating = parseFloat(metadataMatches.rating[1]);

  // 리뷰 추출 (## 리뷰 이후의 모든 텍스트)
  const reviewMatch = markdown.match(/##\s+리뷰\s*\n\s*(.+(?:\n.+)*)/);
  if (reviewMatch) {
    result.review = reviewMatch[1].trim();
  }

  return result;
}

/**
 * 책 리뷰 데이터를 마크다운 형식으로 변환합니다
 *
 * @param review 마크다운으로 변환할 책 리뷰 데이터
 * @returns 생성된 마크다운 문자열
 */
export function bookReviewToMarkdown(review: BookReview): string {
  const sections = [];

  // 제목 섹션
  if (review.title) {
    sections.push(`# ${review.title}\n`);
  }

  // 메타데이터 섹션
  const metadata = [];
  if (review.author) metadata.push(`작가: ${review.author}`);
  if (review.genre) metadata.push(`장르: ${review.genre}`);
  if (review.pub_year) metadata.push(`출판년도: ${review.pub_year}`);
  if (review.pages) metadata.push(`페이지: ${review.pages}`);
  if (review.date_read) metadata.push(`읽은 날짜: ${review.date_read}`);
  if (review.rating) metadata.push(`평점: ${review.rating}`);

  if (metadata.length > 0) {
    sections.push(metadata.join('\n'));
  }

  // 리뷰 섹션
  if (review.review) {
    sections.push(`## 리뷰\n\n${review.review}`);
  }

  return sections.join('\n\n') + '\n';
}

/**
 * 파일 경로에서 책 ID를 추출합니다 (파일명이 ID.md 형식인 경우)
 *
 * @param path 파일 경로
 * @returns 추출된 책 ID 또는 null
 */
export function getBookIdFromPath(path: string): number | null {
  const match = path.match(/\/(\d+)\.md$/);
  return match ? parseInt(match[1]) : null;
}

/**
 * 책 제목에서 파일명에 사용할 수 있는 슬러그를 생성합니다
 *
 * @param title 원본 책 제목
 * @returns URL 및 파일명에 적합한 슬러그 문자열
 */
export function slugify(title: string): string {
  return title
    .toLowerCase()
    .replace(/[^\w\s-]/g, "") // 영숫자, 언더스코어, 하이픈 및 공백만 유지
    .replace(/\s+/g, "-")     // 공백을 하이픈으로 변환
    .replace(/-+/g, "-");     // 연속된 하이픈 단일화
}
