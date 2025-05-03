// Import and re-export color functions from standard library
import * as stdColors from "@std/fmt/colors";

// Create a colors object with all the color functions
export const colors = {
  reset: stdColors.reset,
  bold: stdColors.bold,
  dim: stdColors.dim,
  italic: stdColors.italic,
  underline: stdColors.underline,
  inverse: stdColors.inverse,
  hidden: stdColors.hidden,
  strikethrough: stdColors.strikethrough,

  // Foreground colors
  black: stdColors.black,
  red: stdColors.red,
  green: stdColors.green,
  yellow: stdColors.yellow,
  blue: stdColors.blue,
  magenta: stdColors.magenta,
  cyan: stdColors.cyan,
  white: stdColors.white,

  // Background colors
  bgBlack: stdColors.bgBlack,
  bgRed: stdColors.bgRed,
  bgGreen: stdColors.bgGreen,
  bgYellow: stdColors.bgYellow,
  bgBlue: stdColors.bgBlue,
  bgMagenta: stdColors.bgMagenta,
  bgCyan: stdColors.bgCyan,
  bgWhite: stdColors.bgWhite,

  // Bright variants
  brightBlack: stdColors.brightBlack,
  brightRed: stdColors.brightRed,
  brightGreen: stdColors.brightGreen,
  brightYellow: stdColors.brightYellow,
  brightBlue: stdColors.brightBlue,
  brightMagenta: stdColors.brightMagenta,
  brightCyan: stdColors.brightCyan,
  brightWhite: stdColors.brightWhite,
};

// Re-export utility functions
export const {
  setColorEnabled,
  getColorEnabled,
} = stdColors;
