# Rust 명령 프롤프트 스터디

Rust 기초 학습을 바탕으로 커멘드 라인 셔듀를 직접 구현하는 개인 프로젝트입니다.

---

## 파일 빌드 및 시행

```bash
cargo build
cargo run
```

---

## 하나번째 단계(기능)

- **내림 명령어 처리**
  - `cd <dir>`: 디렉터리 이동
  - `pwd`: 현재 경로 출력
  - `exit`: 셔듀 종료
- **외부 명령어 시행**
  - `ls`, `date` 등 시스템 기본 명령 시행
- **입력 파싱**
  - 문자열을 공백 기준으로 나누어 인자(`Vec<String>`) 배열 구성

---

## 프로젝트 구조

```bash
rusty_shell/
├── src/
│   ├── main.rs       # 메인 루프
│   ├── parser.rs     # 입력 파싱 모듈
│   └── executor.rs   # 명령어 시행 모듈
├── Cargo.toml        # Rust 패키지 설정 파일
└── README.md         # 프로젝트 소개
```

---

## 다음 단계 예정 기능

- 파이프 (`|`) 기능 지원
- 입출력 리디렉션 (`>`, `<`) 지원
- 백그라운드 시행 (`&`) 지원
- 명령어 히스토리 저장
- `.rustyrc` 설정 파일 지원
- 프로젝트 구조 리파틱(모듈화, 에러 핸들린 건설)

---

## 협업 방식 (호주 프로젝트이라도 읽기 효과 있음)

- 브랜치 관리: 기능별 브랜치(`feature/parser`, `feature/executor` 등)
- 컨미트 메시지: `feat:`, `fix:`, `docs:` 형식으로 작성 (Conventional Commits 스타일)
- `.gitignore` 설정: `target/`, `*.rs.bk` 파일 등 빌드 결과물 제외

---

## Rust 혹과된 기술

- Rust 표준 라이브러리
  - `std::env` (`cd`, `pwd` 처리)
  - `std::process::Command` (외부 명령 시행)
  - `std::io` (입력 처리)

---

