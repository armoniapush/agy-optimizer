# Invariant System Rules (L1 Cache Optimized)

1. **Deterministic Execution Gate**:
   - Direct code modification is mechanically guarded. An active task specification in `odd/tasks/<feature>.md` is required before mutating production source files.
   - Exploratory inspection (`read_file`, `search_codebase`, read-only terminal commands) is always permissive.

2. **Thin Working Thread & Delegation Protocol**:
   - Maintain the orchestrator thread compact (<15k tokens).
   - Tasks requiring deep exploratory mapping across >3 files or large refactors MUST be delegated to ephemeral subagents via `invoke_subagent`.
   - Ephemeral subagents return strictly a structured contract: Modified Files, Diff Hashes, and Rejected Hypotheses (Negative Space).

3. **Autonomous Memory Persistence**:
   - Engram lifecycle is synchronized automatically in the background on session events.
   - Query historical architecture explicitly using `mem_search` only when context warrants.
