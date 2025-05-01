import { Input, Confirm } from "cliffy/prompt/mod.ts";
import { Database } from "../db.ts";
import { colors } from "../utils/colors.ts";

/**
 * Handles the 'review' command to add a review to an existing book
 */
export async function handleReviewCommand(db: Database, bookId: number): Promise<void> {
  // First check if the book exists
  const books = db.getBooks({ id: bookId });

  if (books.length === 0) {
    console.log(colors.red(`Error: No book found with ID ${bookId}`));
    return;
  }

  const book = books[0];
  console.log(colors.green(`Adding review for: ${book.title} by ${book.author}`));

  // If book already has reviews, show them first
  if (book.reviews && book.reviews.length > 0) {
    console.log(colors.yellow("\nExisting reviews:"));
    for (const review of book.reviews) {
      console.log(`Date: ${review.date_read}`);
      console.log(`Rating: ${"★".repeat(review.rating)}${"☆".repeat(5 - review.rating)}`);
      console.log(`${review.review}\n`);
    }

    const shouldContinue = await Confirm.prompt("Add another review?");
    if (!shouldContinue) {
      return;
    }
  }

  // Get review details
  const ratingStr = await Input.prompt({
    message: "Rating (1-5)",
    validate: (value: string) => {
      const num = parseInt(value);
      return num >= 1 && num <= 5 ? true : "Rating must be between 1 and 5";
    },
  });

  const review = await Input.prompt({
    message: "Review",
  });

  const dateRead = await Input.prompt({
    message: "Date read (YYYY-MM-DD, leave empty for today)",
    default: "",
    validate: (value: string) => {
      if (!value) return true; // Empty is allowed

      // Basic date format validation
      const dateRegex = /^\d{4}-\d{2}-\d{2}$/;
      if (!dateRegex.test(value)) {
        return "Date must be in format YYYY-MM-DD";
      }

      // Validate date is real
      const date = new Date(value);
      if (isNaN(date.getTime())) {
        return "Invalid date";
      }

      return true;
    },
  });

  // Add review to database
  const reviewId = db.addReview({
    book_id: bookId,
    rating: parseInt(ratingStr),
    review,
    date_read: dateRead || undefined,
  });

  console.log(colors.green(`Review added with ID: ${reviewId}`));
}
