import { colors } from "./index.ts";

export type ErrorType =
  | 'VALIDATION'
  | 'DATABASE'
  | 'USER_INPUT'
  | 'UNEXPECTED';

export class AppError extends Error {
  type: ErrorType;
  details?: Record<string, unknown>;

  constructor(message: string, type: ErrorType = 'UNEXPECTED', details?: Record<string, unknown>) {
    super(message);
    this.name = 'AppError';
    this.type = type;
    this.details = details;
  }
}

// 에러 생성 유틸리티
export const createError = (message: string, type: ErrorType, details?: Record<string, unknown>) =>
  new AppError(message, type, details);

// 전역 에러 핸들러
export const errorHandler = (err: unknown) => {
  if (err instanceof AppError) {
    switch (err.type) {
      case 'VALIDATION':
        console.error(colors.yellow(`Validation: ${err.message}`), err.details);
        break;
      case 'DATABASE':
        console.error(colors.red(`Database: ${err.message}`), err.details);
        break;
      case 'USER_INPUT':
        console.log(colors.blue(`${err.message}`));
        break;
      default:
        console.error(colors.red(`Error: ${err.message}`), err.details);
    }
  } else {
    console.error(colors.red('Unexpected error:'), err instanceof Error ? err.message : String(err));
  }

  return null; // 체이닝에서 사용할 때 null을 반환하여 체인 중단
};

// 검증 유틸리티
export const validate = (condition: boolean, message: string, field?: string) => {
  if (!condition) {
    throw createError(message, 'VALIDATION', { field });
  }
};

// 일반적인 유효성 검사 함수들
export const validateRequired = (value: unknown, fieldName: string): void => {
  if (value === null || value === undefined || (typeof value === 'string' && value.trim() === '')) {
    throw createError(`${fieldName} is required`, 'VALIDATION', { field: fieldName });
  }
};

export const validateNumber = (value: unknown, fieldName: string, options?: { min?: number; max?: number }): void => {
  if (value === null || value === undefined) return; // Optional number

  const num = typeof value === 'number' ? value : Number(value);
  if (isNaN(num)) {
    throw createError(`${fieldName} must be a valid number`, 'VALIDATION', { field: fieldName });
  }

  if (options?.min !== undefined && num < options.min) {
    throw createError(`${fieldName} must be at least ${options.min}`, 'VALIDATION', { field: fieldName });
  }

  if (options?.max !== undefined && num > options.max) {
    throw createError(`${fieldName} must be at most ${options.max}`, 'VALIDATION', { field: fieldName });
  }
};

export const validatePattern = (value: string, pattern: RegExp, fieldName: string, message?: string): void => {
  if (!pattern.test(value)) {
    throw createError(message || `${fieldName} has invalid format`, 'VALIDATION', { field: fieldName });
  }
};
