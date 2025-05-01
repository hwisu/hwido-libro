## 1&nbsp;· Database Layer

```ts
// db.ts
import { DB } from "sqlite";

/** Runs migrations and exposes high-level CRUD helpers. */
export class Database {
  #db: DB;

  constructor(path = "libro.db") {
    this.#db = new DB(path);
    this.migrate();
  }

  /** Apply schema updates if version changed. */
  migrate() {
    // Get current migration version.
    const [[oldVersion]] = this.#db.query<[number]>("PRAGMA user_version");
    // Initial migration: version 1.
    if (oldVersion < 1) {
      this.#db.execute(`
        PRAGMA foreign_keys = ON;

        CREATE TABLE IF NOT EXISTS books (
          id        INTEGER PRIMARY KEY AUTOINCREMENT,
          title     TEXT    NOT NULL,
          author    TEXT    NOT NULL,
          pages     INTEGER,
          pub_year  INTEGER,
          genre     TEXT
        );

        CREATE TABLE IF NOT EXISTS reviews (
          id         INTEGER PRIMARY KEY AUTOINCREMENT,
          book_id    INTEGER NOT NULL,
          date_read  DATE,
          rating     INTEGER,
          review     TEXT,
          FOREIGN KEY(book_id) REFERENCES books(id)
        );
      `);
      this.#db.execute("PRAGMA user_version = 1;");
    }
  }

  addBook(book) { /* … */ }
  addReview(review) { /* … */ }
  getBooks(filter?) { /* … */ }
}
```

* Use `PRAGMA user_version` for forward-compatible migrations.
* Wrap low-level queries in small helper methods to keep CLI commands thin.

---

## 2&nbsp;· CLI Surface (Cliffy)

| Command | Options / Args | Purpose |
| --- | --- | --- |
| **`add`** | prompts \| `--stdin` JSON | Insert a new book; optional inline review |
| **`show [id]`** | `--year <yyyy>` · `--json` | List all or detailed view for one book |
| **`review <id>`** | rating + text prompts | Append a review to existing book |
| **`report`** | `--author` · `--year` | Aggregate stats with ASCII bar chart |

```ts
import { Command } from "cliffy/command/mod.ts";
import { Database } from "./db.ts";

const db = new Database();

await new Command()
  .name("libro").version("0.1.0")
  .command("add", "Add a new book", /* … */)
  .command("show", "Show books", /* … */)
  .command("review", "Add review", /* … */)
  .command("report", "Generate reports", /* … */)
  .parse(Deno.args);
```

---

## 3&nbsp;· Output Helpers

* **`Table.ts`** – auto-adjusts column width, uses `@std/fmt/colors` for headers and zebra stripes.
* **`barChart.ts`** – 10-line util that prints monospace bars like `█████ 12`.

Example bar chart:

```text
2024 ██████████ 18
2023 ████████ 14
2022 ██ 3
```

---

## 4&nbsp;· Tests

```
tests/
├── db_test.ts          // init & CRUD
├── cli_add_test.ts     // spawn binary, add book
├── cli_report_test.ts  // report aggregates
```

* Use `Deno.test()` plus `withTempDir()` to sandbox a throw-away SQLite file.
* CI matrix (Linux · macOS · Windows) runs: `deno fmt` → `deno lint` → `deno test`.

---

## 5&nbsp;· Docs & Packaging

* **README badges**: CI status, Deno version, license.
* **Install**:
  ```bash
  deno install -A -n libro https://raw.githubusercontent.com/<user>/libro-deno/main/src/main.ts
  ```
* **Usage Examples** with copy-paste blocks for every command.
* Explain required permissions (`--allow-read`, `--allow-write`, `--allow-ffi`) and how to override the DB path via `LIBRO_DB` env var.
