#!/usr/bin/env -S deno run --allow-read --allow-write --allow-ffi

import { Command } from "cliffy/command/mod.ts";
import { Database } from "./db.ts";
import {
  handleAddCommand,
  handleShowCommand,
  handleReviewCommand,
  handleReportCommand,
  handleImportMarkdownCommand,
} from "./commands/index.ts";

// Initialize database
const db = new Database("libro.db");

await new Command()
  .name("libro")
  .version("0.1.0")
  .description("A command-line book tracking tool with data stored in SQLite")

  // Original Libro commands
  .command("add", "Add a new book")
    .action(async () => {
      await handleAddCommand(db);
    })

  .command("show [id:number]", "Show book(s) by id or year")
    .option("--year <year:number>", "Show books read in specified year")
    .option("--json", "Output as JSON", { default: false })
    .action((options, id) => {
      handleShowCommand(db, { id, ...options });
    })

  .command("report", "Generate reading reports")
    .option("--author", "Show most read authors", { default: false })
    .option("--year <year:number>", "Filter report by year")
    .option("--years", "Show books read by year chart", { default: false })
    .action((options) => {
      handleReportCommand(db, options);
    })

  // Enhanced commands not in original Libro
  .command("review <id:number>", "Add or edit a review for a book")
    .action(async (_, id) => {
      await handleReviewCommand(db, id);
    })

  .command("import", "Import books and reviews from markdown files")
    .option("--path <path:string>", "Path to directory with markdown files", { default: "data/assets" })
    .option("--sync", "Sync database to markdown files", { default: false })
    .action(async (options) => {
      await handleImportMarkdownCommand(db, options);
    })

  .parse(Deno.args);

// Close the database connection when the program finishes
db.close();
