use std::env;
use std::fs;
use std::io::{self, Read};
use std::path::{Component, Path, PathBuf};
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

fn extract_target_path(args: &serde_json::Value) -> Option<&str> {
    for key in &["TargetFile", "targetFile", "Path", "path", "filePath", "filepath"] {
        if let Some(val) = args.get(*key).and_then(|v| v.as_str()) {
            return Some(val);
        }
    }
    None
}

fn normalize_path(path: &Path) -> PathBuf {
    let mut normalized = PathBuf::new();
    for comp in path.components() {
        match comp {
            Component::CurDir => {},
            Component::ParentDir => {
                normalized.pop();
            },
            c => normalized.push(c.as_os_str()),
        }
    }
    normalized
}

fn is_spec_file(path_str: &str, workspaces: &[PathBuf]) -> bool {
    let raw_path = Path::new(path_str);
    let normalized = normalize_path(raw_path);
    let norm_str = normalized.to_string_lossy().replace('\\', "/").to_ascii_lowercase();

    let is_md = normalized.extension()
        .map_or(false, |ext| ext.eq_ignore_ascii_case("md"));
    if !is_md {
        return false;
    }

    let parts: Vec<&str> = norm_str.trim_start_matches("./").split('/').filter(|s| !s.is_empty()).collect();
    if parts.len() == 3 && parts[0] == "odd" && parts[1] == "tasks" {
        return true;
    }

    for ws in workspaces {
        let ws_str = ws.to_string_lossy().replace('\\', "/").to_ascii_lowercase();
        let expected_prefix = format!("{}/odd/tasks/", ws_str.trim_end_matches('/'));
        if norm_str.starts_with(&expected_prefix) {
            let remainder = &norm_str[expected_prefix.len()..];
            if !remainder.contains('/') {
                return true;
            }
        }
    }

    false
}

fn resolve_target_workspace(target_path_str: &str) -> Option<PathBuf> {
    let raw_path = Path::new(target_path_str);
    let mut current = if raw_path.is_file() || target_path_str.ends_with(".rs") || target_path_str.ends_with(".md") || target_path_str.ends_with(".json") || target_path_str.ends_with(".js") || target_path_str.ends_with(".ts") {
        raw_path.parent()?
    } else {
        raw_path
    };

    loop {
        let tasks = current.join("odd").join("tasks");
        if tasks.exists() && tasks.is_dir() {
            return Some(normalize_path(current));
        }
        match current.parent() {
            Some(parent) if parent != current => current = parent,
            _ => break,
        }
    }
    None
}

fn has_active_task_in_workspaces(workspaces: &[PathBuf]) -> bool {
    for ws in workspaces {
        let tasks_dir = ws.join("odd").join("tasks");
        if tasks_dir.exists() {
            if let Ok(entries) = fs::read_dir(&tasks_dir) {
                let found = entries.filter_map(Result::ok).any(|e| {
                    e.path()
                        .extension()
                        .map_or(false, |ext| ext.eq_ignore_ascii_case("md"))
                });
                if found {
                    return true;
                }
            }
        }
    }
    false
}

fn handle_pre_gate(input_str: &str) -> io::Result<()> {
    if input_str.is_empty() {
        return deny("[ERROR MECÁNICO ODD]: Entrada vacía recibida en pre-gate.");
    }

    let input: HookInput = match serde_json::from_str(input_str) {
        Ok(v) => v,
        Err(_) => return deny("[ERROR MECÁNICO ODD]: Payload corrupto o JSON inválido recibido en pre-gate."),
    };

    let mut workspaces: Vec<PathBuf> = Vec::new();
    if let Some(ws_list) = input.workspacePaths {
        for w in ws_list {
            if !w.is_empty() {
                workspaces.push(normalize_path(Path::new(&w)));
            }
        }
    }

    let target_path_opt = input.toolCall
        .as_ref()
        .and_then(|t| t.args.as_ref())
        .and_then(extract_target_path);

    // If target belongs to an existing workspace root with odd/tasks, add it
    if let Some(target) = target_path_opt {
        if let Some(inferred_ws) = resolve_target_workspace(target) {
            if !workspaces.contains(&inferred_ws) {
                workspaces.push(inferred_ws);
            }
        }
    }

    if workspaces.is_empty() {
        workspaces.push(normalize_path(Path::new(".")));
    }

    if let Some(target) = target_path_opt {
        if is_spec_file(target, &workspaces) {
            return allow();
        }
    }

    if !has_active_task_in_workspaces(&workspaces) {
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