import { Input } from "cliffy/prompt/mod.ts";
import { Database } from "../db.ts";

/**
 * Handles the 'add' command to insert a new book into the database
 * with optional review.
 */
export async function handleAddCommand(db: Database): Promise<void> {
  // Get book info from user
  const title = await Input.prompt({
    message: "Title",
    minLength: 2,
  });

  const author = await Input.prompt({
    message: "Author",
    minLength: 2,
  });

  const pagesStr = await Input.prompt({
    message: "Pages (optional)",
    default: "",
  });
  const pages = pagesStr ? parseInt(pagesStr) : undefined;

  const yearStr = await Input.prompt({
    message: "Publication year (optional)",
    default: "",
  });
  const pubYear = yearStr ? parseInt(yearStr) : undefined;

  const genre = await Input.prompt({
    message: "Genre (optional)",
    default: "",
  });

  // Add book to database
  const bookId = db.addBook({
    title,
    author,
    pages,
    pub_year: pubYear,
    genre: genre || undefined,
  });

  console.log(`Book added with ID: ${bookId}`);

  // Ask if user wants to add a review
  const addReview = await Input.prompt({
    message: "Add a review? (y/n)",
    default: "n",
  });

  if (addReview.toLowerCase() === "y") {
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

    const reviewId = db.addReview({
      book_id: bookId,
      rating: parseInt(ratingStr),
      review,
    });

    console.log(`Review added with ID: ${reviewId}`);
  }
}
