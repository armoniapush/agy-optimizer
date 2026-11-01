use std::fs;
use std::io::{self, Read};
use std::path::{Component, Path, PathBuf};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
struct ToolUsePayload {
    tool_name: Option<String>,
    tool_args: Option<serde_json::Value>,
    #[serde(rename = "toolCall")]
    tool_call: Option<ToolCallInternal>,
}

#[derive(Deserialize, Debug)]
struct ToolCallInternal {
    name: Option<String>,
    args: Option<serde_json::Value>,
}

#[derive(Deserialize, Debug)]
struct StopPayload {
    #[serde(rename = "terminationReason")]
    termination_reason: Option<String>,
    error: Option<String>,
    #[serde(rename = "fullyIdle")]
    fully_idle: Option<bool>,
}

#[derive(Serialize)]
struct HookDecision {
    decision: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    reason: Option<String>,
}

#[derive(Serialize)]
struct InjectedStep {
    #[serde(skip_serializing_if = "Option::is_none")]
    system_message: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ephemeral_message: Option<String>,
}

#[derive(Serialize)]
struct PreInvocationResponse {
    #[serde(rename = "injectSteps")]
    inject_steps: Vec<serde_json::Value>,
}

fn normalize_lexical_path(p: &Path) -> PathBuf {
    let mut normalized = PathBuf::new();
    for component in p.components() {
        match component {
            Component::ParentDir => {
                normalized.pop();
            }
            Component::CurDir => {}
            _ => {
                normalized.push(component);
            }
        }
    }
    normalized
}

fn is_scaffolding_or_non_production(path_str: &str) -> bool {
    let lower = path_str.to_lowercase().replace('\\', "/");
    
    // ODD tasks and specs are always permissive
    if lower.contains("odd/tasks") || lower.contains("odd/receipts") || lower.contains(".odd") {
        return true;
    }
    // Documentation, markdown, notes
    if lower.ends_with(".md") || lower.ends_with(".txt") || lower.ends_with(".json") {
        return true;
    }
    // Configuration, dependencies, and types are scaffolding (OpenHands pattern)
    if lower.ends_with("package.json") || lower.ends_with("cargo.toml") || lower.ends_with("tsconfig.json")
       || lower.ends_with(".gitignore") || lower.ends_with("go.mod") || lower.ends_with("requirements.txt") {
        return true;
    }
    // Tests are permissive for writing RED reproduction tests
    if lower.contains("/tests/") || lower.contains("/test/") || lower.ends_with("_test.go")
       || lower.ends_with(".spec.ts") || lower.ends_with(".test.ts") || lower.ends_with(".test.js") {
        return true;
    }

    false
}

fn resolve_workspace_root(target: &Path) -> PathBuf {
    let mut curr = if target.is_dir() {
        target.to_path_buf()
    } else {
        target.parent().unwrap_or(target).to_path_buf()
    };

    while let Some(parent) = curr.parent() {
        if curr.join("odd").exists() || curr.join(".git").exists() || curr.join("Cargo.toml").exists() || curr.join("package.json").exists() {
            return curr;
        }
        curr = parent.to_path_buf();
    }

    std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."))
}

fn has_active_odd_task(workspace_root: &Path) -> bool {
    let tasks_dir = workspace_root.join("odd").join("tasks");
    if let Ok(entries) = fs::read_dir(tasks_dir) {
        for entry in entries.flatten() {
            let p = entry.path();
            if p.is_file() {
                if let Some(ext) = p.extension() {
                    if ext == "md" || ext == "json" {
                        if let Ok(metadata) = fs::metadata(&p) {
                            if metadata.len() > 20 {
                                return true;
                            }
                        }
                    }
                }
            }
        }
    }
    false
}

fn handle_pre_tool() {
    let mut buffer = Vec::new();
    if io::stdin().read_to_end(&mut buffer).is_err() || buffer.is_empty() {
        let resp = HookDecision {
            decision: "deny".into(),
            reason: Some("[ERROR MECÁNICO ODD]: Entrada de hook vacía.".into()),
        };
        println!("{}", serde_json::to_string(&resp).unwrap());
        return;
    }

    let clean_str = String::from_utf8_lossy(&buffer).trim_start_matches('\u{feff}').to_string();
    let payload: ToolUsePayload = match serde_json::from_str(&clean_str) {
        Ok(p) => p,
        Err(_) => {
            let resp = HookDecision {
                decision: "deny".into(),
                reason: Some("[ERROR MECÁNICO ODD]: Payload corrupto o JSON inválido en PreToolUse.".into()),
            };
            println!("{}", serde_json::to_string(&resp).unwrap());
            return;
        }
    };

    let tool_name = payload.tool_name
        .or_else(|| payload.tool_call.as_ref().and_then(|tc| tc.name.clone()))
        .unwrap_or_default();

    let args = payload.tool_args
        .or_else(|| payload.tool_call.and_then(|tc| tc.args))
        .unwrap_or(serde_json::Value::Null);

    // Permissive tools (reads, exploration, non-mutating inspection)
    if tool_name == "read_file" || tool_name == "search_codebase" || tool_name == "list_dir" || tool_name == "ask_question" {
        println!("{}", serde_json::to_string(&HookDecision { decision: "allow".into(), reason: None }).unwrap());
        return;
    }

    if tool_name == "write_file" || tool_name == "edit_file" {
        let raw_path = args.get("Path")
            .or_else(|| args.get("path"))
            .or_else(|| args.get("filePath"))
            .or_else(|| args.get("TargetFile"))
            .and_then(|v| v.as_str())
            .unwrap_or("");

        let target_path = normalize_lexical_path(Path::new(raw_path));
        let path_str = target_path.to_string_lossy();

        // 1. Scaffolding, configs, types, tests, docs are PERMISSIVE (OpenHands/Aider pattern)
        if is_scaffolding_or_non_production(&path_str) {
            println!("{}", serde_json::to_string(&HookDecision { decision: "allow".into(), reason: None }).unwrap());
            return;
        }

        // 2. Production code mutation requires active ODD task
        let ws = resolve_workspace_root(&target_path);
        if has_active_odd_task(&ws) {
            println!("{}", serde_json::to_string(&HookDecision { decision: "allow".into(), reason: None }).unwrap());
            return;
        }

        // 3. Deny with Structured Guidance (Zero-Turn Recovery)
        let guidance = format!(
            "[FSM ODD VETO]: Mutación de código de producción bloqueada en '{}'.\n\
            Fase actual: SCOUT / SPEC PENDIENTE.\n\
            Para editar archivos de producción, redacta primero la especificación en 'odd/tasks/<feature>.md' con:\n\
            1) Objetivo\n2) Archivos a tocar\n3) Test de verificación.",
            path_str
        );

        let resp = HookDecision {
            decision: "deny".into(),
            reason: Some(guidance),
        };
        println!("{}", serde_json::to_string(&resp).unwrap());
        return;
    }

    println!("{}", serde_json::to_string(&HookDecision { decision: "allow".into(), reason: None }).unwrap());
}

fn handle_stop_gate() {
    let mut buffer = Vec::new();
    let _ = io::stdin().read_to_end(&mut buffer);

    let cwd = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    
    // Check if there is an active ODD task in progress
    if has_active_odd_task(&cwd) {
        // In the future, check if receipt exists. If spec exists without receipt:
        let receipts_dir = cwd.join("odd").join("receipts");
        let has_receipt = fs::read_dir(receipts_dir).map(|mut d| d.next().is_some()).unwrap_or(false);

        if !has_receipt {
            // Task is in progress but not closed! Force continuation (Claude Code autonomy pattern)
            let resp = HookDecision {
                decision: "continue".into(),
                reason: Some("[MANDATO DE CIERRE AUTÓNOMO]: La tarea ODD sigue activa sin recibo de verificación en 'odd/receipts/'. Continúa ejecutando hasta compilar o verificar con tests.".into()),
            };
            println!("{}", serde_json::to_string(&resp).unwrap());
            return;
        }
    }

    // Normal stop allowed
    println!("{}", serde_json::to_string(&HookDecision { decision: "allow".into(), reason: None }).unwrap());
}

fn handle_pre_invocation() {
    // ReFold Projection: Inject clean ephemeral guidance without dirtying long-term transcript
    let resp = serde_json::json!({
        "injectSteps": [
            {
                "ephemeralMessage": "[AUTONOMOUS EXECUTION HARNESS]: Mantén propiedad completa de la solución. Ejecuta herramientas sin pedir confirmación intermedia hasta verificar el resultado final."
            }
        ]
    });
    println!("{}", serde_json::to_string(&resp).unwrap());
}

fn handle_session_init() {
    let resp = serde_json::json!({
        "injectSteps": [
            {
                "systemMessage": {
                    "systemMessage": "[AGY OPTIMIZER ACTIVE]: Arnés cognitivo determinista en Rust activo. FSM Coercitiva, ciclo ODD guiado y ReFold KV-Cache protection."
                }
            }
        ]
    });
    println!("{}", serde_json::to_string(&resp).unwrap());
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mode = args.get(1).map(|s| s.as_str()).unwrap_or("pre-gate");

    match mode {
        "pre-gate" => handle_pre_tool(),
        "stop-gate" => handle_stop_gate(),
        "pre-invocation" => handle_pre_invocation(),
        "session-init" => handle_session_init(),
        _ => {
            println!("{}", serde_json::to_string(&HookDecision { decision: "allow".into(), reason: None }).unwrap());
        }
    }
}