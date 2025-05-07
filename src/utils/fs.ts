/**
 * 파일 시스템 관련 유틸리티 함수들
 */

/**
 * 디렉토리가 존재하는지 확인하고, 없으면 생성합니다
 */
export async function ensureDir(path: string): Promise<void> {
  try {
    await Deno.stat(path);
  } catch (error) {
    if (error instanceof Deno.errors.NotFound) {
      await Deno.mkdir(path, { recursive: true });
    } else {
      throw error;
    }
  }
}


/**
 * 디렉토리의 모든 파일 목록을 반환합니다
 */
export async function listFiles(
  dir: string,
  extension = ".md",
): Promise<string[]> {
  const files: string[] = [];

  try {
    for await (const entry of Deno.readDir(dir)) {
      if (entry.isFile && entry.name.endsWith(extension)) {
        files.push(`${dir}/${entry.name}`);
      }
    }
  } catch (error) {
    if (!(error instanceof Deno.errors.NotFound)) {
      throw error;
    }
  }

  return files;
}

/**
 * 파일에서 텍스트를 읽습니다
 */
export async function readTextFile(path: string): Promise<string> {
  return await Deno.readTextFile(path);
}

/**
 * 파일에 텍스트를 씁니다
 */
export async function writeTextFile(
  path: string,
  content: string,
): Promise<void> {
  await Deno.writeTextFile(path, content);
}

