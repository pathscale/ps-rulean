# ps-rulean

A Rust library subset of the upstream rulean source at 33a5f1b72d2f9ceb13514ea0f8b911928ebf68d8. The workspace contains ps-eterned, its procedural macros, ps-lean-syn-expr, ps-lean-kernel and ps-lean-macro-expander. Original Rust library crate names are retained for path consumers.

The parser, elaborator, analyzer, command-line applications and editor integrations are outside this subset. The interner uses an RwLock-protected HashMap and rechecks under the write lock before creating an entry. Existing tests for retained libraries are kept; parser-dependent integration contracts require a deliberate future port if those capabilities are added.

PS_FORK_PROVENANCE.ron records origin and adaptation scope. Source preservation and syntax review do not establish build, Lean compatibility or runtime correctness.
