# AGY Optimizer Package

Ultra-lightweight deterministic railway guard, KV-cache preservation, invisible Engram memory lifecycle, and anti-rot architecture for Google Antigravity (AGY).

---

## What This Does

1. **Deterministic ODD Guard (`PreToolUse`)**: Blocks source code mutation (`write_file`, `edit_file`) with exit code 0 and `"decision": "deny"` if an active task specification does not exist in `odd/tasks/<feature>.md`. Exploratory inspection and reads remain 100% permissive.
2. **Sub-2ms Native Rust Execution**: Compiled into a static 300 KB binary (`agy-hook.exe`) with zero runtime overhead or garbage collection pauses.
3. **Dual Packaging**: Distributable both as a self-contained Antigravity Plugin and as an NPX CLI runner (`npx agy-optimizer install`).
4. **Adversarial Hardening**:
   - **Lexical traversal bypass:** Defeated via path component normalization.
   - **Windows casing tricks:** Defeated via case-insensitive matching.
   - **Fail-closed fallback:** Malformed inputs or stdin EOF immediately deny rather than allowing unauthorized edits.
   - **Cross-workspace tree resolution:** Crawls parent directories to locate workspace root.
5. **Invisible Memory Lifecycle (`SessionStart`)**: Automates Engram historical context loading on startup without requiring LLM tool invocations.
6. **Anti-Context Rot Architecture**: Replaces thousands of lines of passive markdown rules with <40 lines of invariant rules, enabling >90% API prompt cache hit rate.

---

## Installation & Distribution Modes

### Mode 1: NPX CLI Runner (Recommended for Any Machine)

Install or verify without cloning the repository:

```bash
# 1-step installer
npx agy-optimizer install

# Check status and components
npx agy-optimizer verify
```

### Mode 2: Native Antigravity Plugin (Drop-in Directory)

Place or clone the folder directly into your Antigravity plugins directory:

```bash
git clone https://github.com/armoniapush/agy-optimizer.git %USERPROFILE%\.gemini\config\plugins\agy-optimizer
```

Then ensure it is enabled in `%USERPROFILE%\.gemini\config\config.json`:

```json
{
  "plugins": {
    "agy-optimizer": {
      "enabled": true
    }
  }
}
```

### Mode 3: Local 1-Click PowerShell Script

From inside this cloned directory, run:

```powershell
.\install.ps1
```

---

## Mechanical Architecture

```
[Agent Tool Call: write_file / edit_file]
                   ¦
                   ?
     [hooks.json: PreToolUse Event]
                   ¦
                   ?
    [agy-hook.exe pre-gate (1.5ms)]
        ¦                     ¦
        +- Task Spec Exists --+-? Exit 0 {"decision": "allow"}
        ¦                     ¦
        +- No Task Spec --------? Exit 0 {"decision": "deny", "reason": "[ERROR MECANICO ODD]..."}
```

---

## Documentation & Benchmarks

Read the complete engineering report, Tier 1-5 compaction matrix, and Gemini 4 Argon analysis at:
`SPECIFICATION.md`

---

## Acknowledgements & Credits

This project builds directly upon the concepts, paradigms, and pioneering work of:

* **[Gentle AI / Gentleman Programming](https://github.com/Gentleman-Programming/gentle-ai)**: Creator of Organic Driven Development (ODD), the 4Rs Review Protocol, Receipt-Driven Development (RDD), and the Thin Working Thread philosophy. `agy-optimizer` exists as a native binary enforcement layer designed to make the Gentle AI paradigms mechanically inviolable in closed runtimes like Google Antigravity.
* **[Engram](https://github.com/Gentleman-Programming/engram)**: Persistent memory architecture for AI coding agents.
