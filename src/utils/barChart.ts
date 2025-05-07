import { colors } from "./colors.ts";

/**
 * Options for generating a bar chart
 */
export interface BarChartOptions {
  /** Maximum width of the bars */
  maxBarWidth?: number;

  /** Character to use for the bar */
  barChar?: string;

  /** Whether to color the bars */
  colorize?: boolean;

  /** Whether to sort the data before rendering (default: no sorting) */
  sort?: "asc" | "desc" | "none";

  /** Column headers */
  headers?: string[];

  /** Add title to the chart */
  title?: string;
}

/**
 * A simple utility for creating ASCII bar charts.
 *
 * Examples:
 * ```
 * 2023 ██████ 12
 * 2022 ███ 6
 * 2021 █ 2
 * ```
 */
export function barChart(
  data: Record<string, number>,
  options: BarChartOptions = {},
): string {
  const {
    maxBarWidth = 30,
    barChar = "▄",
    colorize = true,
    sort = "none",
    headers = ["Label", "Count", "Bar"],
    title,
  } = options;

  // Convert data to array of [label, value] pairs
  let entries = Object.entries(data);

  if (entries.length === 0) {
    return "";
  }

  // Sort entries if needed
  if (sort === "asc") {
    entries = entries.sort((a, b) => a[1] - b[1]);
  } else if (sort === "desc") {
    entries = entries.sort((a, b) => b[1] - a[1]);
  }

  // Find the maximum value for scaling
  const maxValue = Math.max(...entries.map(([_, value]) => value));

  // Calculate the scaling factor
  const scale = maxValue > 0 ? maxBarWidth / maxValue : 0;

  // Find the longest label for proper spacing
  const labelWidth = Math.max(...entries.map(([label]) => label.length));
  const countWidth = String(maxValue).length;

  // Build the output with proper spacing
  let output = "";

  // Add title if provided
  if (title) {
    const totalWidth = labelWidth + countWidth + maxBarWidth + 10;
    const paddedTitle = title.padStart((totalWidth + title.length) / 2).padEnd(
      totalWidth,
    );
    output += "\n" + colors.bold(colors.cyan(paddedTitle)) + "\n\n";
  }

  // Add header row
  if (headers) {
    const labelHeader = headers[0].padEnd(labelWidth);
    const countHeader = headers[1].padEnd(countWidth);
    const barHeader = headers[2];

    output += "  " + colors.bold(labelHeader) + "   " +
      colors.bold(countHeader) + "   " +
      colors.bold(barHeader) + "\n";

    // Add separator line
    output += " " + "─".repeat(labelWidth + countWidth + maxBarWidth + 7) +
      "\n";
  }

  // Build the chart rows
  const rows = entries.map(([label, value]) => {
    // Calculate bar length proportionally
    const barLength = Math.max(1, Math.round(scale * value));
    const bar = barChar.repeat(barLength);

    // Format the bar with optional coloring
    const coloredBar = colorize ? colors.cyan(bar) : bar;

    // Pad label and count based on alignment
    const paddedLabel = label.padEnd(labelWidth);
    const paddedCount = String(value).padStart(countWidth);

    // Format the row with proper spacing
    return `  ${paddedLabel}   ${paddedCount}   ${coloredBar}`;
  });

  output += rows.join("\n");
  return output;
}

