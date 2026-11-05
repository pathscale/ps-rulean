# TODO: ps-rulean


## Cleanup ownership 2026-11-05

- [ ] Build and run only the retained-library checks after source preservation; no build/runtime evidence was produced during cleanup.
- [ ] If a parser is adopted, port the eleven meaningful original parser-dependent macro integration contracts from upstream commit33a5f1b72d2f9ceb13514ea0f8b911928ebf68d8 onto the actual parser API; do not substitute synthetic AST examples or preserve retired debug-print probes as coverage. Retained native macro unit cases are wired in src/lib.rs.
- [ ] Verify all downstream native consumers of the five package/library-name pairs before claiming integration readiness.
