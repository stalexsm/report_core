# Git workflow: ветки, PR, коммиты, gates, CHANGELOG

## Ветки и PR

- В `main` напрямую не коммитим и не пушим: работа всегда в новой ветке от свежего `main`:

  ```bash
  git switch main && git pull
  git switch -c <type>/<краткое-имя>   # например: feat/finder-regex
  ```

- Имя ветки — латиницей, kebab-case, с префиксом типа (`feat/`, `fix/`, `refactor/`).
- Один PR — одна задача. PR открывается в `main` через `gh pr create`; описание PR рассказывает, что сделано и зачем.
- Заголовок PR — в формате Conventional Commits с русским описанием (`docs: ...`, `fix: ...`): при squash-merge он становится коммитом в `main` и попадает в CHANGELOG. Если в PR несколько типов работ, тип берётся по преобладающей задаче.
- Merge делает человек: агент сам не мержит PR и не пушит в `main`.

## Gates

Гейт — проверка или согласование, без которых следующий шаг запрещён. Quality-воркфлоу (`.github/workflows/quality.yml`) гоняет fmt/clippy/test/deny/skylos/semgrep на PR; локальные гейты — быстрый фильтр перед коммитом, чтобы не ждать CI.

| Гейт | Когда | Что гонять | Готово, когда |
|---|---|---|---|
| Коммит | перед каждым коммитом | `cargo fmt --all -- --check`; `cargo clippy --workspace --all-targets -- -D warnings` | обе команды — код выхода 0 |
| Тесты и линт | перед `gh pr create` | гейт коммита + `cargo test --workspace` + `ruff check .` | всё зелёное |
| Согласование PR | перед `gh pr create` | покажи сводку изменений и результат гейтов, получи подтверждение | явное «да» человека |
| Согласование релиза | перед тегом и публикацией | остановка и подтверждение | явное «да» человека |

`cargo test --workspace` — зелёный: `extension-module` включается только maturin-сборкой, линковка тестов не ломается. Python-контур (wheel из `maturin build` → `pip install` → `pytest core_pyo3/tests`) гоняется в quality.yml; смок-тесты лежат в `core_pyo3/tests/`.

## Коммиты: Conventional Commits, описание на русском

Формат: `type(scope): Описание с заглавной буквы, точка в конце` — как в истории проекта (`fix: Для поиска по параметрам, вернул сортировку.`).

- Тип — стандартный набор: `feat`, `fix`, `refactor`, `perf`, `docs`, `test`, `build`, `ci`, `chore`.
- Описание — на русском (см. `docs/agents/code-style.md`).
- Ломающее изменение — `!` после типа (`feat!:`) и `BREAKING CHANGE:` в теле.
- `chore(release): prepare for X.Y.Z` — только для релизных коммитов, текст префикса воспроизводи точно: `cliff.toml` исключает такие коммиты из CHANGELOG (см. **Релиз**).

## CHANGELOG.md: только git-cliff

`CHANGELOG.md` генерируется `git-cliff` по `cliff.toml`. Вручную его не редактируй — правки затрёт следующая генерация.

- Посмотреть не вышедшие изменения: `git-cliff --unreleased`
- Релизная секция поверх существующего файла: `git-cliff --unreleased --tag X.Y.Z --prepend CHANGELOG.md` — заголовок встанет с тегом; выполняй до коммита самого тега
- Полный пересбор файла: `git-cliff -o CHANGELOG.md`
- Следующая версия по коммитам — считает сам git-cliff: `git-cliff --bumped-version` (печать) и `git-cliff --bump` (генерация с новой версией)

## Релиз

Релиз — точка согласования: без явного подтверждения человека тег не ставится.

1. Убедись, что гейты зелёные на актуальном `main`.
2. `git-cliff --bumped-version` — узнай следующую версию.
3. Подними версию в двух местах: `version` в `[workspace.package]` корневого `Cargo.toml` и `version` в корневом `pyproject.toml` (крейты наследуют через `version.workspace = true`, версия Python-пакета динамическая).
4. `git-cliff --unreleased --tag X.Y.Z --prepend CHANGELOG.md`.
5. Закоммить всё (CHANGELOG и манифесты) как `chore(release): prepare for X.Y.Z` — точный префикс обязателен, `cliff.toml` исключает его из будущих генераций.
6. Получи подтверждение, поставь тег `X.Y.Z` (без префикса `v`, как в истории) и запушь ветку с тегом — CI соберёт и опубликует релиз.
