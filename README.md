# AGY Optimizer Package v2.0 (Certified Autonomous Harness)

Deterministic native railway guard, KV-cache prefix protection, invisible Engram memory lifecycle, and FSM Coercitive Architecture for Google Antigravity (AGY).

---

## Architecture Overview

```
[Antigravity Execution Loop]
         �
         +-? SessionStart ----? agy-hook (Engram L2 Sync)
         �                      - Invisible memory lifecycle
         �
         +-? PreInvocation ---? agy-hook (ReFold Context Engine - arXiv:2610.07863)
         �                      - Invariant ephemeral message projection
         �                      - 100% KV-Cache prefix hit rate preservation
         �
         +-? PreToolUse ------? agy-hook (Rust Policy Evaluator - sub-2ms)
         �                      - Scaffolding & tests: Permissive (no compilation deadlocks)
         �                      - Zero-Trust odd/receipts/ guard (anti-forgery)
         �                      - Shell Write Guard: Blocks terminal redirects to src/
         �                      - Production code: Requires active ODD task in odd/tasks/
         �
         +-? Stop ------------? agy-hook (The Gatekeeper / Autonomous Closure)
                                - Tests passing & receipt verified?
                                  +- NO (cycle < 20) -? "decision": "continue" (Forced Closed Loop)
                                  +- NO (cycle >= 20) -? "decision": "allow" (Circuit Breaker Abort)
                                  +- YES -------------? "decision": "allow" (Task Verified & Closed)
```

---

## Certified Industrial Safeguards

1. **Deterministic ODD Guard (`PreToolUse`)**: Blocks unauthorized production source mutations (`src/`, `app/`) with exit code 0 and structured JSON guidance if an active task specification does not exist in `odd/tasks/<feature>.md`.
2. **Anti-Forgery Zero-Trust (`odd/receipts/`)**: Prohibits models from synthesizing fake receipts to bypass verification.
3. **Shell Write Guard (`run_command`)**: Intercepts shell commands containing redirection operators (`>`, `>>`, `Set-Content`, `Out-File`, `tee`) targeting production files.
4. **Autonomous Closure (`Stop` Hook)**: Eliminates "confirmitis" by refusing premature model stops (`decision: continue`) until the solution is proven with tests.
5. **Hard Circuit Breaker (`Stop` limit = 20 cycles)**: Automatically halts execution if an external environment issue prevents task completion after 20 cycles, preventing runaway token drains.
6. **Sub-2ms Native Rust Execution**: Static 300 KB binary (`agy-hook.exe`) with zero GC pauses.

---

## Installation & Distribution Modes

### Mode 1: NPX CLI Runner (Recommended for Any Machine)

```bash
# 1-step installer
npx github:armoniapush/agy-optimizer install

# Check status and components
npx github:armoniapush/agy-optimizer verify
```

### Mode 2: Native Antigravity Plugin

Place or clone the folder directly into your Antigravity plugins directory:

```bash
git clone https://github.com/armoniapush/agy-optimizer.git %USERPROFILE%\.gemini\config\plugins\agy-optimizer
```

### Mode 3: Local 1-Click PowerShell Script

```powershell
.\install.ps1
```

---

## Acknowledgements & Lineage

* **[Gentle AI / Gentleman Programming](https://github.com/Gentleman-Programming/gentle-ai)**: Organic Driven Development (ODD), the 4Rs Review Protocol, Receipt-Driven Development (RDD), and the Thin Working Thread philosophy.
* **[Engram](https://github.com/Gentleman-Programming/engram)**: Persistent memory architecture for AI coding agents.
* **Google VM Task Deep Investigation**: Formal verification of ReFold context mechanics (arXiv:2610.07863) and Stop hook autonomous continuation.
