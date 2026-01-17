# Documentation Governance

Keep documentation minimal, precise, and LLM-first.

## Placement

- Specs live in `docs/spec/` and define required behavior.
- Guides live in `docs/guide/` and define workflows and validation.
- Plans live in `docs/plans/` and are non-normative.

Do not duplicate content. Link to the source of truth.

## Updates

- API or behavior changes: update `docs/spec/index.md`.
- Workflow or tooling changes: update `docs/guide/index.md`.
- Data or code generation changes: update both specs and guides.

## Canonical entry points

- Repository overview: `README.md`.
- Specs: `docs/spec/index.md`.
- Guides: `docs/guide/index.md`.
- Unified index: `docs/index.md`.
