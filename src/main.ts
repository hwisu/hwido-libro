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
  .description("A simple reading tracker CLI built with Deno")

  .command("add", "Add a new book")
    .action(async () => {
      await handleAddCommand(db);
    })

  .command("show [id:number]", "Show book(s)")
    .option("--year <year:number>", "Show by year")
    .option("--json", "Output as JSON", { default: false })
    .action((options, id) => {
      handleShowCommand(db, { id, ...options });
    })

  .command("review <id:number>", "Add review to a book")
    .action(async (_, id) => {
      await handleReviewCommand(db, id);
    })

  .command("report", "Generate reports")
    .option("--author <author:string>", "Filter report by author")
    .option("--year <year:number>", "Filter report by publication year")
    .action((options) => {
      handleReportCommand(db, options);
    })

  .command("import-markdown", "Import books and reviews from markdown files")
    .option("--path <path:string>", "Path to directory with markdown files", { default: "data/assets" })
    .option("--sync", "Sync database to markdown files", { default: false })
    .action(async (options) => {
      await handleImportMarkdownCommand(db, options);
    })

  .parse(Deno.args);

// Close the database connection when the program finishes
db.close();
