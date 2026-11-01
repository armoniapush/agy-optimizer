use std::env;
use std::fs;
use std::io::{self, Read};
use std::path::Path;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
#[allow(non_snake_case)]
struct HookInput {
    toolCall: Option<ToolCall>,
    workspacePaths: Option<Vec<String>>,
}

#[derive(Deserialize, Debug)]
struct ToolCall {
    #[allow(dead_code)]
    name: Option<String>,
    args: Option<serde_json::Value>,
}

#[derive(Serialize)]
struct PreToolOutput<'a> {
    decision: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    reason: Option<&'a str>,
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let mode = args.get(1).map(|s| s.as_str()).unwrap_or("pre-gate");

    let mut raw_bytes = Vec::new();
    let _ = io::stdin().read_to_end(&mut raw_bytes);

    let clean_str = String::from_utf8_lossy(&raw_bytes);
    let trimmed = clean_str.trim_start_matches('\u{feff}').trim();

    match mode {
        "pre-gate" => handle_pre_gate(trimmed),
        "session-init" => handle_session_init(),
        _ => allow(),
    }
}

fn handle_pre_gate(input_str: &str) -> io::Result<()> {
    if input_str.is_empty() {
        return allow();
    }

    let input: HookInput = match serde_json::from_str(input_str) {
        Ok(v) => v,
        Err(_) => return allow(),
    };

    let ws_root = input.workspacePaths
        .as_ref()
        .and_then(|w| w.first())
        .map(|s| s.as_str())
        .unwrap_or(".");

    // Check if the current call is writing directly into odd/tasks/
    if let Some(tool) = &input.toolCall {
        if let Some(args) = &tool.args {
            let target_path = args.get("Path")
                .and_then(|p| p.as_str())
                .unwrap_or("");
            
            let normalized = target_path.replace('\\', "/");
            if normalized.contains("odd/tasks/") || normalized.ends_with("odd/tasks") {
                return allow();
            }
        }
    }

    let tasks_dir = Path::new(ws_root).join("odd").join("tasks");

    let has_active_task = if tasks_dir.exists() {
        fs::read_dir(&tasks_dir)
            .map(|entries| {
                entries.filter_map(Result::ok).any(|e| {
                    e.path().extension().map_or(false, |ext| ext == "md")
                })
            })
            .unwrap_or(false)
    } else {
        false
    };

    if !has_active_task {
        return deny(
            "[ERROR MECÁNICO ODD]: Modificación de código denegada.\n\
             No existe un slice de tarea activo en 'odd/tasks/<feature>.md'.\n\
             Requisito previo obligatorio: Redacta primero la especificación de la tarea con:\n\
             1) Objetivo, 2) Archivos a tocar, 3) Test de verificación."
        );
    }

    allow()
}

fn handle_session_init() -> io::Result<()> {
    let _ = std::process::Command::new("engram")
        .arg("context")
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .spawn();

    println!("{{}}");
    Ok(())
}

fn allow() -> io::Result<()> {
    let res = PreToolOutput { decision: "allow", reason: None };
    println!("{}", serde_json::to_string(&res)?);
    Ok(())
}

fn deny(msg: &str) -> io::Result<()> {
    let res = PreToolOutput { decision: "deny", reason: Some(msg) };
    println!("{}", serde_json::to_string(&res)?);
    Ok(())
}
