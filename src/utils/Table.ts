import { colors } from "./colors.ts";

interface TableOptions {
  header?: boolean;
  zebra?: boolean;
  minWidth?: number; // Minimum width for each column
  maxWidth?: number; // Maximum width for each column
  padding?: number; // Padding between columns
  border?: boolean; // Show table borders
  borderStyle?: "ascii" | "unicode"; // Border style (ascii or unicode)
  groupByColumn?: number; // Column index to group by
}

/**
 * A simple table formatter for CLI output.
 * Features:
 * - Auto-adjusts column width
 * - Colored headers
 * - Optional zebra striping
 * - Configurable padding and borders
 * - Support for unicode borders
 * - Support for grouping rows
 */
export class Table {
  private _rows: string[][] = [];
  private columnWidths: number[] = [];
  private options: TableOptions;
  private groups: Map<string, string[][]> = new Map();
  private hasGroups = false;

  constructor(options: TableOptions = {}) {
    this.options = {
      header: true,
      zebra: true,
      minWidth: 3,
      maxWidth: 40,
      padding: 1,
      border: false,
      borderStyle: "unicode",
      ...options,
    };
  }

  /**
   * Add a header row to the table
   */
  public header(cells: string[]): this {
    this.addRow(cells, true);
    return this;
  }

  /**
   * Add a data row to the table
   */
  public row(cells: string[]): this {
    this.addRow(cells);
    return this;
  }

  /**
   * Add multiple data rows to the table
   */
  public rows(rows: string[][]): this {
    for (const row of rows) {
      this.row(row);
    }
    return this;
  }

  /**
   * Add a group header row
   */
  public groupHeader(label: string): this {
    this.hasGroups = true;
    const row = ["", label, "", "", ""];
    this.addRow(row);
    return this;
  }

  /**
   * Render the table as a string
   */
  public toString(): string {
    if (this._rows.length === 0) {
      return "";
    }

    // Calculate column widths
    this.calculateColumnWidths();

    // Check if we need to group the rows
    if (this.options.groupByColumn !== undefined && !this.hasGroups) {
      this.groupRowsByColumn(this.options.groupByColumn);
    }

    // Generate rows with proper padding and formatting
    let output = "";

    // Add top border if needed
    if (this.options.border) {
      output += this.makeBorder("top") + "\n";
    }

    if (this.hasGroups) {
      // Render grouped rows
      let isFirstGroup = true;
      let headerRow: string[] | null = null;

      if (this.options.header && this._rows.length > 0) {
        headerRow = this._rows[0];
      }

      // For each group
      for (const [groupName, groupRows] of this.groups.entries()) {
        // Add group header
        if (!isFirstGroup && this.options.border) {
          output += this.makeBorder("middle") + "\n";
        }

        if (groupName !== "") {
          output += this.formatRow([groupName, "", "", "", ""], -1, true) + "\n";
        }

        // Add header for each group if needed
        if (headerRow && isFirstGroup) {
          output += this.formatRow(headerRow, 0) + "\n";
          if (this.options.header) {
            output += this.makeBorder("header") + "\n";
          }
        }

        // Add group rows
        groupRows.forEach((row, index) => {
          output += this.formatRow(row, index + 1) + "\n";
        });

        isFirstGroup = false;
      }
    } else {
      // Render normal rows
      this._rows.forEach((row, rowIndex) => {
        const formattedRow = this.formatRow(row, rowIndex);
        output += formattedRow + "\n";

        // Add separator after header
        if (rowIndex === 0 && this.options.header) {
          output += this.makeBorder("header") + "\n";
        }
      });
    }

    // Add bottom border if needed
    if (this.options.border) {
      output += this.makeBorder("bottom");
    }

    return output;
  }

  /**
   * Print the table to the console
   */
  public render(): void {
    console.log(this.toString());
  }

  /**
   * Group rows by a column value
   */
  private groupRowsByColumn(columnIndex: number): void {
    if (!this.options.header || this._rows.length <= 1) {
      return;
    }

    const headerRow = this._rows[0];
    const dataRows = this._rows.slice(1);

    // Group rows by the specified column
    for (const row of dataRows) {
      const groupValue = row[columnIndex] || "";
      if (!this.groups.has(groupValue)) {
        this.groups.set(groupValue, []);
      }
      this.groups.get(groupValue)?.push(row);
    }

    this.hasGroups = true;
  }

  /**
   * Internal helper to add a row and update column widths
   */
  private addRow(cells: string[], isHeader = false): void {
    // Convert all cell values to strings
    const stringCells = cells.map((cell) => String(cell || ""));

    // Update column widths if necessary
    this._rows.push(stringCells);

    // Pre-calculate column widths when adding rows for efficiency
    stringCells.forEach((cell, index) => {
      // Measure the visual width by counting characters
      const cellWidth = this.getCellWidth(cell);

      if (!this.columnWidths[index] || cellWidth > this.columnWidths[index]) {
        this.columnWidths[index] = Math.min(
          cellWidth,
          this.options.maxWidth || 40
        );
      }
    });
  }

  /**
   * Calculate and update column widths based on content
   */
  private calculateColumnWidths(): void {
    // Make sure all columns meet minimum width
    this.columnWidths = this.columnWidths.map((width) =>
      Math.max(width, this.options.minWidth || 3)
    );
  }

  /**
   * Format a row with proper padding and colors
   */
  private formatRow(cells: string[], rowIndex: number, isGroupHeader = false): string {
    const isHeader = rowIndex === 0 && this.options.header;
    const isZebraRow = this.options.zebra && rowIndex % 2 === 1 && !isHeader && !isGroupHeader;

    const formattedCells = cells.map((cell, columnIndex) => {
      // For group headers, only format the first cell
      if (isGroupHeader && columnIndex > 0) {
        return "".padEnd(this.columnWidths[columnIndex] + (this.options.padding || 1) * 2);
      }

      // Truncate and pad the cell content
      const width = this.columnWidths[columnIndex] || 0;
      let cellContent = cell;

      // Truncate if longer than max width
      if (cell.length > width) {
        cellContent = cell.substring(0, width - 1) + "…";
      }

      // Pad to match column width
      const padding = " ".repeat(this.options.padding || 1);
      const paddedCell = `${padding}${cellContent}${padding}`.padEnd(
        width + (this.options.padding || 1) * 2
      );

      // Apply formatting based on row type
      if (isHeader) {
        return colors.bold(colors.cyan(paddedCell));
      } else if (isGroupHeader) {
        return colors.bold(paddedCell);
      } else if (isZebraRow) {
        return colors.dim(paddedCell);
      }

      return paddedCell;
    });

    // Combine cells into a row
    const rowChar = this.getBorderChar("vertical");
    return `${rowChar}${formattedCells.join(rowChar)}${rowChar}`;
  }

  /**
   * Create a border line for tables
   */
  private makeBorder(position: "top" | "bottom" | "header" | "middle"): string {
    const parts = this.columnWidths.map((width) =>
      this.getBorderChar("horizontal").repeat(width + (this.options.padding || 1) * 2)
    );

    // Get the right border characters based on position
    const left = this.getBorderChar(`${position}Left`);
    const mid = this.getBorderChar(`${position}Mid`);
    const right = this.getBorderChar(`${position}Right`);

    return `${left}${parts.join(mid)}${right}`;
  }

  /**
   * Get the border character based on style and position
   */
  private getBorderChar(position: string): string {
    if (!this.options.border) return "";

    const unicodeChars: Record<string, string> = {
      horizontal: "━",
      vertical: "┃",
      topLeft: "┏",
      topMid: "┳",
      topRight: "┓",
      headerLeft: "┣",
      headerMid: "╋",
      headerRight: "┫",
      middleLeft: "┣",
      middleMid: "╋",
      middleRight: "┫",
      bottomLeft: "┗",
      bottomMid: "┻",
      bottomRight: "┛",
    };

    const asciiChars: Record<string, string> = {
      horizontal: "-",
      vertical: "|",
      topLeft: "+",
      topMid: "+",
      topRight: "+",
      headerLeft: "+",
      headerMid: "+",
      headerRight: "+",
      middleLeft: "+",
      middleMid: "+",
      middleRight: "+",
      bottomLeft: "+",
      bottomMid: "+",
      bottomRight: "+",
    };

    const chars = this.options.borderStyle === "unicode" ? unicodeChars : asciiChars;
    return chars[position] || "";
  }

  /**
   * Get the visible width of a cell (accounting for CJK chars if needed)
   */
  private getCellWidth(str: string): number {
    // CJK 문자 및 전각 문자(full-width)를 감지하는 정규식
    const wideCharRegex = /[\u1100-\u11FF\u3000-\u303F\u3040-\u309F\u30A0-\u30FF\u3130-\u318F\u3400-\u4DBF\u4E00-\u9FFF\uAC00-\uD7AF\uF900-\uFAFF]/g;

    // 전각 문자를 두 칸으로 계산
    const wideCharCount = (str.match(wideCharRegex) || []).length;

    // 기본 문자열 길이 + 추가 너비
    return str.length + wideCharCount;
  }
}
