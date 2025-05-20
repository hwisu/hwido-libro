#!/usr/bin/env -S deno run --allow-read --allow-write --allow-ffi

import { Command } from "@cliffy/command";
import { Database } from "./db.ts";
import {
  handleAddCommand,
  handleEditReviewCommand,
  handleReportCommand,
  handleReviewCommand,
  handleShowCommand,
} from "./commands/index.ts";
import { colors, getHwidoBanner } from "./utils/index.ts";

const db = new Database("libro.db");

if (
  Deno.args.includes("--help") || Deno.args.includes("-h") ||
  Deno.args.length === 0
) {
  console.log(colors.cyan(getHwidoBanner()));
}

const command = new Command()
  .name("libro")
  .version("0.1.7")
  .description("A command-line book tracking tool with data stored in SQLite")
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
  .command("review <id:number>", "Add or edit a review for a book")
  .action(async (_, id) => {
    await handleReviewCommand(db, id);
  })
  .command("import", "Import books and reviews from markdown files")
  .option("--path <path:string>", "Path to directory with markdown files", {
    default: "data/assets",
  })
  .command("edit-review <id:number>", "Edit a book review using system editor")
  .action(async (_, id) => {
    await handleEditReviewCommand(db, id);
  });

await command.parse(Deno.args);

db.close();
