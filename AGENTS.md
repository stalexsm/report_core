# AGENTS.md

report_core — Rust-ядро + PyO3-биндинги Python-библиотеки
для построения XLSX-отчётов.

## Agent skills

### Issue tracker

Задачи живут в GitHub Issues репозитория (stalexsm/report_core), работа через `gh` CLI. See `docs/agents/issue-tracker.md`.

### Triage labels

Стандартные метки триажа: needs-triage, needs-info, ready-for-agent, ready-for-human, wontfix. See `docs/agents/triage-labels.md`.

### Domain docs

Single-context: один `GLOSSARY.md` + `docs/adr/` в корне. See `docs/agents/domain.md`.

### Git workflow

Работа всегда в новой ветке через PR (в `main` напрямую не коммитим); коммиты и заголовки PR — Conventional Commits с русским описанием; gates перед коммитом, PR и релизом; CHANGELOG генерирует git-cliff, вручную не редактировать. See `docs/agents/git-workflow.md`.

### Code style

Комментарии, docstring и коммиты — на русском; числа в бизнес-логике — только именованные константы. See `docs/agents/code-style.md`.
