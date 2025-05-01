// 함수형 프로그래밍 유틸리티 - lodash/rxjs 스타일
export const pipe = <T>(...fns: Array<(arg: T) => T>) => (x: T): T =>
  fns.reduce((v, f) => f(v), x);

export const pipeAsync = <T>(...fns: Array<(arg: T) => Promise<T> | T>) => async (x: T): Promise<T> => {
  let result = x;
  for (const fn of fns) {
    result = await fn(result);
  }
  return result;
};

export const tap = <T>(fn: (value: T) => void) => (value: T): T => {
  fn(value);
  return value;
};

export const map = <T, R>(fn: (value: T) => R) => (array: T[]): R[] =>
  array.map(fn);

export const filter = <T>(predicate: (value: T) => boolean) => (array: T[]): T[] =>
  array.filter(predicate);

export const reduce = <T, R>(fn: (acc: R, value: T) => R, initial: R) => (array: T[]): R =>
  array.reduce(fn, initial);

export const flow = pipe; // lodash flow의 별칭

export const catchError = <T>(handler: (err: unknown) => T) => async (promise: Promise<T>): Promise<T> => {
  try {
    return await promise;
  } catch (err) {
    return handler(err);
  }
};

// 추가 유틸리티 함수들
export const isEmpty = (value: unknown): boolean => {
  if (value === null || value === undefined) return true;
  if (typeof value === 'string' || Array.isArray(value)) return value.length === 0;
  if (typeof value === 'object') return Object.keys(value as Record<string, unknown>).length === 0;
  return false;
};

export const isNotEmpty = (value: unknown): boolean => !isEmpty(value);

export const toInt = (value: string): number | undefined => {
  const parsed = parseInt(value);
  return isNaN(parsed) ? undefined : parsed;
};

export const toString = (value: unknown): string =>
  value === null || value === undefined ? '' : String(value);

export const pick = <T extends Record<string, unknown>, K extends keyof T>(keys: K[]) => (obj: T): Pick<T, K> => {
  const result = {} as Pick<T, K>;
  keys.forEach(key => {
    if (key in obj) result[key] = obj[key];
  });
  return result;
};

export const omit = <T extends Record<string, unknown>, K extends keyof T>(keys: K[]) => (obj: T): Omit<T, K> => {
  const result = { ...obj } as Omit<T, K>;
  keys.forEach(key => {
    delete result[key as unknown as keyof Omit<T, K>];
  });
  return result;
};
