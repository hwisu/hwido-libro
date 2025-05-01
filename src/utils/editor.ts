/**
 * 텍스트 에디터 관련 유틸리티
 */

/**
 * 임시 파일에 텍스트를 저장한 후 vim으로 편집하고 수정된 결과를 반환합니다
 *
 * @param text 편집할 초기 텍스트
 * @returns 편집 후 텍스트
 */
export async function editWithVim(text: string): Promise<string | null> {
  try {
    // 임시 파일 경로 생성
    const tempDir = await Deno.makeTempDir({ prefix: "libro_edit_" });
    const tempFilePath = `${tempDir}/edit.md`;

    // 임시 파일에 텍스트 저장
    await Deno.writeTextFile(tempFilePath, text);

    // vim 실행
    const cmd = ["vim", tempFilePath];

    // 현재 터미널에서 vim을 대화형으로 실행
    const process = new Deno.Command(cmd[0], {
      args: cmd.slice(1),
      stdin: "inherit",
      stdout: "inherit",
      stderr: "inherit",
    });

    // vim 프로세스가 종료될 때까지 대기
    const { code } = await process.output();

    if (code !== 0) {
      console.error(`vim 편집기 종료 오류: ${code}`);
      return null;
    }

    // 편집된 내용 읽기
    const editedText = await Deno.readTextFile(tempFilePath);

    // 임시 파일 및 디렉토리 삭제
    await Deno.remove(tempFilePath);
    await Deno.remove(tempDir);

    return editedText;
  } catch (error: unknown) {
    console.error(`에디터 실행 오류: ${(error as Error).message}`);
    return null;
  }
}

/**
 * 시스템에 설치된 기본 에디터로 텍스트를 편집합니다
 * 환경 변수 EDITOR를 확인하고, 기본값으로 vim 사용
 *
 * @param text 편집할 초기 텍스트
 * @returns 편집 후 텍스트
 */
export async function editWithSystemEditor(text: string): Promise<string | null> {
  try {
    // 환경 변수에서 기본 에디터 확인
    const editor = Deno.env.get("EDITOR") || "vim";

    // 임시 파일 생성
    const tempDir = await Deno.makeTempDir({ prefix: "libro_edit_" });
    const tempFilePath = `${tempDir}/edit.md`;

    // 임시 파일에 텍스트 저장
    await Deno.writeTextFile(tempFilePath, text);

    // 에디터 실행
    const cmdParts = editor.split(/\s+/);

    const process = new Deno.Command(cmdParts[0], {
      args: [...cmdParts.slice(1), tempFilePath],
      stdin: "inherit",
      stdout: "inherit",
      stderr: "inherit",
    });

    // 에디터 프로세스가 종료될 때까지 대기
    const { code } = await process.output();

    if (code !== 0) {
      console.error(`에디터 종료 오류: ${code}`);
      return null;
    }

    // 편집된 내용 읽기
    const editedText = await Deno.readTextFile(tempFilePath);

    // 임시 파일 및 디렉토리 삭제
    await Deno.remove(tempFilePath);
    await Deno.remove(tempDir);

    return editedText;
  } catch (error: unknown) {
    console.error(`에디터 실행 오류: ${(error as Error).message}`);
    return null;
  }
}
