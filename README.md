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
`SPECIFICATION.md`

## Acknowledgements & Credits

This project builds directly upon the concepts, paradigms, and pioneering work of:

* **[Gentle AI / Gentleman Programming](https://github.com/Gentleman-Programming/gentle-ai)**: Creator of Organic Driven Development (ODD), the 4Rs Review Protocol, Receipt-Driven Development (RDD), and the Thin Working Thread philosophy. `agy-optimizer` exists as a native binary enforcement layer designed to make the Gentle AI paradigms mechanically inviolable in closed runtimes like Google Antigravity.
* **[Engram](https://github.com/Gentleman-Programming/engram)**: Persistent memory architecture for AI coding agents.
