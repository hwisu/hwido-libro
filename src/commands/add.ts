import { Database } from "../db.ts";
import { colors } from "../utils/index.ts";
import { Input, Confirm, Select } from "cliffy/prompt/mod.ts";
import { addBook, addBookWithReview } from "../lib/db-operations.ts";
import { createError, errorHandler, validateNumber, validatePattern, validateRequired } from "../utils/errors.ts";
import { toInt } from "../utils/fp.ts";

/**
 * Handles the 'add' command to add a new book
 */

// 프롬프트 도우미 함수들 - 화살표 함수와 ES 기능 최대 활용
const promptBook = async () => {
  try {
    // 필수 입력
    const title = await Input.prompt({ message: "Title", minLength: 1 });
    validateRequired(title, "Title");

    const author = await Input.prompt({ message: "Author", minLength: 1 });
    validateRequired(author, "Author");

    // 선택 입력
    const pagesStr = await Input.prompt({ message: "Pages (optional)", default: "" });
    const pages = toInt(pagesStr);
    if (pagesStr && pages === undefined) {
      throw createError("Pages must be a valid number", "VALIDATION", { field: "pages" });
    }

    const yearStr = await Input.prompt({ message: "Publication year (optional)", default: "" });
    const pub_year = toInt(yearStr);
    if (yearStr && pub_year === undefined) {
      throw createError("Publication year must be a valid number", "VALIDATION", { field: "year" });
    }

    const genre = await Select.prompt({
      message: "Genre",
      options: [
        { name: "Fiction", value: "Fiction" },
        { name: "Nonfiction", value: "Nonfiction" },
        { name: "Science Fiction", value: "Sci-Fi" },
        { name: "Fantasy", value: "Fantasy" },
        { name: "Mystery", value: "Mystery" },
        { name: "Biography", value: "Biography" },
        { name: "Other", value: "" },
      ],
      default: "Fiction"
    });

    return {
      title,
      author,
      pages,
      pub_year,
      genre: genre || undefined
    };
  } catch (e) {
    if (e instanceof Error && e.name === "AbortError") {
      throw createError("Book input cancelled", "USER_INPUT");
    }
    throw e;
  }
};

// 리뷰 정보 수집
const promptReview = async () => {
  try {
    // 리뷰 추가 여부 확인
    const shouldAddReview = await Confirm.prompt({ message: "Add a review?", default: false });

    if (!shouldAddReview) {
      return null;
    }

    // 날짜 입력
    const dateRead = await Input.prompt({
      message: "Date read (YYYY-MM-DD)",
      default: new Date().toISOString().split("T")[0],
    });
    validatePattern(dateRead, /^\d{4}-\d{2}-\d{2}$/, "Date", "Invalid date format. Please use YYYY-MM-DD");

    // 평점 입력
    const ratingStr = await Input.prompt({
      message: "Rating (1-5)",
      default: "3",
    });
    const rating = toInt(ratingStr);
    validateNumber(rating, "Rating", { min: 1, max: 5 });

    // 리뷰 텍스트 입력
    const reviewText = await Input.prompt({
      message: "Review (optional)",
      default: "",
    });

    return {
      date_read: dateRead,
      rating: rating as number, // 이미 검증했으므로 안전
      review: reviewText
    };
  } catch (e) {
    if (e instanceof Error && e.name === "AbortError") {
      throw createError("Review input cancelled", "USER_INPUT");
    }
    throw e;
  }
};

// 메인 핸들러 함수
export async function handleAddCommand(db: Database): Promise<void> {
  try {
    // 1. 책 정보 수집
    const book = await promptBook();

    // 2. 리뷰 정보 수집
    const review = await promptReview();

    // 3. DB에 저장
    if (review) {
      // 책과 리뷰 함께 저장
      const { bookId, reviewId } = await addBookWithReview(db)({ book, review });
      console.log(colors.green(`Book added with ID: ${bookId}`));
      if (reviewId) {
        console.log(colors.green(`Review added with ID: ${reviewId}`));
      }
    } else {
      // 책만 저장
      const bookId = await addBook(db)(book);
      console.log(colors.green(`Book added with ID: ${bookId}`));
    }
  } catch (error) {
    errorHandler(error);
  }
}
