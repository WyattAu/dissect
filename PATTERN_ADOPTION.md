# PATTERN_ADOPTION

Dissect's declared relationship to the WyattAu pattern canon
(`pattern-standards`, references/catalog.md). Machine-checked by
`pattern-standards/scripts/adoption-check.py`.

| Pattern | Status | Evidence |
|---|---|---|
| T1 | adopted | src/cliJsonInput.ts:30 (one parse boundary: inline JSON, stdin, or one local file becomes typed values via `Result`, no re-validation downstream), src/domain/webExecution.ts:25 (epistemic status as closed enums: `verified`/`unknown`, `confirmed`/`unconfirmed`, not booleans) |
| T13 | adopted | src/domain/result.ts:1 (pure domain core: `Result` channel, error taxonomy, projection), src/process/BoundedFileBytes.ts:1 (imperative shell: bounded file reads, control lines, process capture) |
