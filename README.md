# AGY Optimizer Package

Ultra-lightweight deterministic railway guard, KV-cache preservation, invisible Engram memory lifecycle, and anti-rot architecture for Google Antigravity (AGY).

## What This Does

1. **Deterministic ODD Guard (`PreToolUse`)**: Blocks source code modification (`write_file`, `edit_file`) with exit code 0 and `"decision": "deny"` if an active task specification does not exist in `odd/tasks/<feature>.md`. Reads and scouting remain 100% permissive.
2. **Sub-2ms Native Rust Execution**: Compiled into a static 300 KB binary (`agy-hook.exe`) with zero runtime overhead or garbage collection pauses.
3. **Invisible Memory Lifecycle (`SessionStart`)**: Automates Engram historical context loading on startup without requiring LLM tool invocations.
4. **Anti-Context Rot Architecture**: Replaces thousands of lines of passive markdown rules with <40 lines of invariant rules, enabling >90% API prompt cache hit rate.

## Installation

Run PowerShell inside this folder:

```powershell
.\install.ps1
```

## Architecture Document

Read the complete engineering report and benchmarks at:
`agy_harness_specification.md`
