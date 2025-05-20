import { DB } from "sqlite";

const dbPath = "libro.db";

try {
  const db = new DB(dbPath);

  console.log(`Database: ${dbPath}`);

  // Get user_version
  const [[userVersion]] = db.query<[number]>("PRAGMA user_version;");
  console.log(`\nUser Version: ${userVersion}`);

  // Get table info for books
  console.log("\nTable Info: books");
  const booksInfo = db.query("PRAGMA table_info(books);");
  if (booksInfo.length > 0) {
    console.log(booksInfo.map(row => `  - ${row[1]} (${row[2]})`).join("\n"));
  } else {
    console.log("  Table 'books' not found.");
  }

  // Get table info for writers
  console.log("\nTable Info: writers");
  const writersInfo = db.query("PRAGMA table_info(writers);");
    if (writersInfo.length > 0) {
    console.log(writersInfo.map(row => `  - ${row[1]} (${row[2]})`).join("\n"));
  } else {
    console.log("  Table 'writers' not found.");
  }

  // Get table info for book_writers
  console.log("\nTable Info: book_writers");
  const bookWritersInfo = db.query("PRAGMA table_info(book_writers);");
    if (bookWritersInfo.length > 0) {
    console.log(bookWritersInfo.map(row => `  - ${row[1]} (${row[2]})`).join("\n"));
  } else {
    console.log("  Table 'book_writers' not found.");
  }

  db.close();

} catch (error) {
  console.error(`Error inspecting database: ${error}`);
}
