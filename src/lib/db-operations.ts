import { Database } from "../db.ts";
import { createError } from "../utils/errors.ts";
import { tap } from "../utils/fp.ts";

// 모델 타입 정의
export interface Book {
  id?: number;
  title: string;
  author: string;
  pages?: number;
  pub_year?: number;
  genre?: string;
  reviews?: Review[];
}

export interface Review {
  id?: number;
  book_id: number;
  date_read: string;
  rating: number;
  review: string;
}

// 책 추가 함수
export const addBook = (db: Database) => async (book: Book): Promise<number> => {
  try {
    const id = db.addBook(book);
    return id;
  } catch (error) {
    throw createError(`Failed to add book: ${error instanceof Error ? error.message : String(error)}`, 'DATABASE');
  }
};

// 리뷰 추가 함수
export const addReview = (db: Database) => async (review: Review): Promise<number> => {
  try {
    const id = db.addReview(review);
    return id;
  } catch (error) {
    throw createError(`Failed to add review: ${error instanceof Error ? error.message : String(error)}`, 'DATABASE');
  }
};

// 트랜잭션으로 책과 리뷰 한 번에 추가
export const addBookWithReview = (db: Database) => async ({ book, review }: { book: Book; review?: Omit<Review, 'book_id'> }): Promise<{ bookId: number; reviewId?: number }> => {
  try {
    // 책 추가
    const bookId = db.addBook(book);

    let reviewId;
    // 리뷰가 있으면 추가
    if (review) {
      reviewId = db.addReview({
        ...review,
        book_id: bookId
      });
    }

    return { bookId, reviewId };
  } catch (error) {
    throw createError(`Transaction failed: ${error instanceof Error ? error.message : String(error)}`, 'DATABASE');
  }
};

// 책 조회 함수
export const getBooks = (db: Database) => (filter?: { id?: number; year?: number }): Book[] => {
  try {
    return db.getBooks(filter);
  } catch (error) {
    throw createError(`Failed to get books: ${error instanceof Error ? error.message : String(error)}`, 'DATABASE');
  }
};

// 리뷰 가져오기
export const getReviews = (db: Database) => (bookId: number): Review[] => {
  try {
    return db.getReviews(bookId);
  } catch (error) {
    throw createError(`Failed to get reviews: ${error instanceof Error ? error.message : String(error)}`, 'DATABASE');
  }
};
