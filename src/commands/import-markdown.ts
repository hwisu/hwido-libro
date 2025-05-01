import { Database } from "../db.ts";
import {
  colors,
  parseMarkdownReview,
  listFiles,
  readTextFile,
  writeTextFile,
  bookReviewToMarkdown,
  ensureDir,
  BookReview
} from "../utils/index.ts";

export interface ImportMarkdownOptions {
  path?: string;
  sync?: boolean;
}

/**
 * 마크다운 파일에서 책 정보 및 리뷰를 임포트하는 명령어
 */
export async function handleImportMarkdownCommand(
  db: Database,
  options: ImportMarkdownOptions
): Promise<void> {
  const path = options.path || "data/assets";
  const sync = options.sync || false;

  console.log(colors.cyan(`마크다운 파일을 ${path} 디렉토리에서 임포트합니다...`));

  try {
    // 디렉토리에서 모든 마크다운 파일 목록 가져오기
    const files = await listFiles(path);

    if (files.length === 0) {
      console.log(colors.yellow(`${path} 디렉토리에 마크다운 파일이 없습니다.`));
      return;
    }

    console.log(colors.green(`${files.length}개의 마크다운 파일을 찾았습니다.`));

    // 각 파일을 처리
    let success = 0;
    let failed = 0;

    for (const file of files) {
      try {
        // 파일 내용 읽기
        const content = await readTextFile(file);

        // 마크다운 파싱
        const reviewData = parseMarkdownReview(content);

        if (!reviewData.title || !reviewData.author) {
          console.log(colors.red(`오류: ${file} - 필수 필드 누락 (제목 또는 작가)`));
          failed++;
          continue;
        }

        // 책 및 리뷰 추가
        const bookId = await addOrUpdateBook(db, reviewData);

        if (reviewData.rating && reviewData.date_read) {
          await addOrUpdateReview(db, bookId, reviewData);
        }

        console.log(colors.green(`성공: ${file} - ${reviewData.title} 임포트됨`));
        success++;
      } catch (error) {
        console.log(colors.red(`오류: ${file} - ${(error as Error).message}`));
        failed++;
      }
    }

    console.log(colors.cyan(`\n임포트 완료: ${success}개 성공, ${failed}개 실패\n`));

    // 동기화 모드인 경우 데이터베이스의 책을 마크다운 파일로 내보내기
    if (sync) {
      await syncDatabaseToMarkdown(db, path);
    }
  } catch (error) {
    console.error(colors.red(`오류 발생: ${(error as Error).message}`));
  }
}

/**
 * 책 정보를 데이터베이스에 추가하거나 업데이트합니다
 */
async function addOrUpdateBook(db: Database, data: BookReview): Promise<number> {
  // 이미 책이 있는지 확인 (제목과 작가로 검색)
  // 현재 getBooks()는 id와 year만 필터로 지원하므로 모든 책을 가져와서 필터링
  const allBooks = db.getBooks();
  const existingBooks = allBooks.filter(book =>
    book.title === data.title && book.author === data.author
  );

  if (existingBooks.length > 0) {
    const bookId = existingBooks[0].id;

    // 책 정보 업데이트
    db.updateBook(bookId, {
      genre: data.genre,
      pub_year: data.pub_year,
      pages: data.pages
    });

    return bookId;
  } else {
    // 새 책 추가
    return db.addBook({
      title: data.title!,
      author: data.author!,
      genre: data.genre,
      pub_year: data.pub_year,
      pages: data.pages
    });
  }
}

/**
 * 리뷰 정보를 데이터베이스에 추가하거나 업데이트합니다
 */
async function addOrUpdateReview(db: Database, bookId: number, data: BookReview): Promise<void> {
  // 리뷰 가져오기
  const existingReviews = db.getReviews(bookId);

  const reviewData = {
    book_id: bookId,
    rating: data.rating!,
    date_read: data.date_read!,
    review: data.review || ""
  };

  if (existingReviews.length > 0) {
    // 리뷰 업데이트
    const reviewId = existingReviews[0].id;
    db.updateReview(reviewId, reviewData);
  } else {
    // 새 리뷰 추가
    db.addReview(reviewData);
  }
}

/**
 * 데이터베이스의 책 정보를 마크다운 파일로 동기화합니다
 */
async function syncDatabaseToMarkdown(db: Database, path: string): Promise<void> {
  console.log(colors.cyan("\nDB → 마크다운 동기화를 시작합니다..."));

  // 디렉토리 확인 및 생성
  try {
    await ensureDir(path);
  } catch (error) {
    console.error(colors.red(`디렉토리 생성 오류: ${(error as Error).message}`));
    return;
  }

  // 모든 책 가져오기
  const books = db.getBooks();

  if (books.length === 0) {
    console.log(colors.yellow("데이터베이스에 책이 없습니다."));
    return;
  }

  console.log(colors.green(`총 ${books.length}개의 책을 동기화합니다...`));

  let success = 0;
  let skipped = 0;

  for (const book of books) {
    // 책에 대한 리뷰 가져오기
    const reviews = db.getReviews(book.id);

    // 리뷰가 없으면 건너뛰기
    if (reviews.length === 0) {
      skipped++;
      continue;
    }

    // 각 리뷰에 대해 마크다운 파일 생성
    for (const review of reviews) {
      try {
        // 마크다운 데이터 생성
        const reviewData: BookReview = {
          title: book.title,
          author: book.author,
          genre: book.genre,
          pub_year: book.pub_year,
          pages: book.pages,
          date_read: review.date_read,
          rating: review.rating,
          review: review.review
        };

        // 마크다운으로 변환
        const markdown = bookReviewToMarkdown(reviewData);

        // 파일명 생성 (ID)
        const fileName = `${book.id}.md`;
        const filePath = `${path}/${fileName}`;

        // 파일 저장
        await writeTextFile(filePath, markdown);

        console.log(colors.green(`저장 완료: ${filePath}`));
        success++;
      } catch (error) {
        console.error(colors.red(`오류: ${book.title} - ${(error as Error).message}`));
      }
    }
  }

  console.log(colors.cyan(`\n동기화 완료: ${success}개 저장, ${skipped}개 리뷰 없음`));
}
