import { Database } from "../db.ts";
import { colors, Table } from "../utils/index.ts";

export interface ShowOptions {
  id?: number;
  year?: number;
  json?: boolean;
}

/**
 * Handles the 'show' command to display books
 * Can show all books, books from a specific year, or details of a specific book
 */
export function handleShowCommand(db: Database, options: ShowOptions): void {
  const { id, year, json } = options;

  const books = db.getBooks({ id, year });

  if (books.length === 0) {
    console.log("No books found matching the criteria.");
    return;
  }

  if (json) {
    // Output as JSON
    console.log(JSON.stringify(books, null, 2));
    return;
  }

  // Detailed view for a single book
  if (id) {
    const book = books[0];
    console.log(colors.bold(colors.green(`${book.title} (${book.pub_year || "Unknown Year"})`)));
    console.log(colors.italic(`by ${book.author}`));

    if (book.pages) {
      console.log(`Pages: ${book.pages}`);
    }

    if (book.genre) {
      console.log(`Genre: ${book.genre}`);
    }

    if (book.reviews && book.reviews.length > 0) {
      console.log("\nReviews:");
      for (const review of book.reviews) {
        console.log(`Date: ${review.date_read}`);
        console.log(`Rating: ${"★".repeat(review.rating)}${"☆".repeat(5 - review.rating)}`);
        console.log(`${review.review}\n`);
      }
    } else {
      console.log("\nNo reviews yet.");
    }
    return;
  }

  // List view for multiple books using the Table utility
  console.log(colors.bold("Books:"));

  const yearFilter = year ? `(${year})` : "";
  console.log(colors.yellow(`Found ${books.length} books ${yearFilter}\n`));

  // Create a table for formatting
  const table = new Table({ border: false, maxWidth: 50 });

  // Add header
  table.header(["ID", "Title", "Author", "Year"]);

  // Add book rows
  books.forEach(book => {
    table.row([
      book.id.toString(),
      book.title,
      book.author,
      book.pub_year?.toString() || ""
    ]);
  });

  // Render the table
  table.render();
}
