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

const DENY_MSG: &str = "[ERROR MECÁNICO ODD]: Modificación de código denegada.\n\
No existe un slice de tarea activo en 'odd/tasks/<feature>.md'.\n\
Requisito previo obligatorio: Redacta primero la especificación de la tarea con:\n\
1) Objetivo, 2) Archivos a tocar, 3) Test de verificación.";

const CORRUPT_DENY_MSG: &str = "[ERROR MECÁNICO ODD]: Entrada inválida o corrupta en pre-gate hook.\n\
Por política de seguridad fail-closed, la modificación se bloquea.";

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

/// Normalizes a path by resolving '.' and '..' components lexically.
fn normalize_path(path: &Path) -> PathBuf {
    let mut normalized = PathBuf::new();
    for component in path.components() {
        match component {
            Component::CurDir => {}
            Component::ParentDir => {
                normalized.pop();
            }
            c => {
                normalized.push(c.as_os_str());
            }
        }
    }
    normalized
}

/// Checks whether a normalized relative or absolute path represents an odd task spec:
/// odd/tasks/<spec>.md (extension case-insensitive: .md, .MD, etc.)
fn is_odd_task_spec(target: &Path) -> bool {
    let normalized = normalize_path(target);
    let components: Vec<String> = normalized
        .components()
        .map(|c| c.as_os_str().to_string_lossy().to_string())
        .collect();

    if components.is_empty() {
        return false;
    }

    // Check if the path targets the odd/tasks directory directly
    if components.len() >= 2 {
        let last = &components[components.len() - 1];
        let second_last = &components[components.len() - 2];
        if second_last.eq_ignore_ascii_case("odd") && last.eq_ignore_ascii_case("tasks") {
            return true;
        }
    }

    // Check if the path targets odd/tasks/<spec>.md
    if components.len() >= 3 {
        let file_name = &components[components.len() - 1];
        let tasks_dir = &components[components.len() - 2];
        let odd_dir = &components[components.len() - 3];

        if odd_dir.eq_ignore_ascii_case("odd") && tasks_dir.eq_ignore_ascii_case("tasks") {
            let path_obj = Path::new(file_name);
            if let Some(ext) = path_obj.extension() {
                if ext.to_string_lossy().eq_ignore_ascii_case("md") {
                    return true;
                }
            }
        }
    }

    false
}

/// Extracts target path string across known parameter key aliases:
/// TargetFile, targetFile, Path, path, filePath, filepath.
fn extract_target_path(args: &serde_json::Value) -> Option<&str> {
    const KEYS: &[&str] = &[
        "TargetFile",
        "targetFile",
        "Path",
        "path",
        "filePath",
        "filepath",
    ];

    for key in KEYS {
        if let Some(val) = args.get(key).and_then(|v| v.as_str()) {
            return Some(val);
        }
    }
    None
}

/// Checks if any workspace path contains an active task in odd/tasks/<spec>.md
fn has_active_task_in_workspaces(workspaces: &[PathBuf]) -> bool {
    for ws in workspaces {
        let tasks_dir = ws.join("odd").join("tasks");
        if tasks_dir.exists() {
            let has_task = fs::read_dir(&tasks_dir)
                .map(|entries| {
                    entries.filter_map(Result::ok).any(|e| {
                        e.path()
                            .extension()
                            .map_or(false, |ext| ext.to_string_lossy().eq_ignore_ascii_case("md"))
                    })
                })
                .unwrap_or(false);
            if has_task {
                return true;
            }
        }
    }
    false
}

fn evaluate_pre_gate(input_str: &str) -> Result<bool, &'static str> {
    if input_str.is_empty() {
        return Err(CORRUPT_DENY_MSG);
    }

    let input: HookInput = match serde_json::from_str(input_str) {
        Ok(v) => v,
        Err(_) => return Err(CORRUPT_DENY_MSG),
    };

    // Check if the current call is creating or writing to odd/tasks/<spec>.md
    if let Some(tool) = &input.toolCall {
        if let Some(args) = &tool.args {
            if let Some(target_path_str) = extract_target_path(args) {
                let target_path = Path::new(target_path_str);
                if is_odd_task_spec(target_path) {
                    return Ok(true);
                }
            }
        }
    }

    // Resolve workspaces
    let workspaces: Vec<PathBuf> = match &input.workspacePaths {
        Some(paths) if !paths.is_empty() => paths.iter().map(PathBuf::from).collect(),
        _ => vec![PathBuf::from(".")],
    };

    if has_active_task_in_workspaces(&workspaces) {
        Ok(true)
    } else {
        Err(DENY_MSG)
    }
}

fn handle_pre_gate(input_str: &str) -> io::Result<()> {
    match evaluate_pre_gate(input_str) {
        Ok(true) => allow(),
        Ok(false) => deny(DENY_MSG),
        Err(reason) => deny(reason),
    }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_or_corrupt_input_fails_closed() {
        assert!(evaluate_pre_gate("").is_err());
        assert!(evaluate_pre_gate("   ").is_err());
        assert!(evaluate_pre_gate("not-a-json").is_err());
        assert!(evaluate_pre_gate("{\"toolCall\": invalid}").is_err());
    }

    #[test]
    fn test_multi_key_extraction_and_case_insensitivity() {
        // TargetFile with .MD uppercase
        let json = r#"{"toolCall":{"name":"write_file","args":{"TargetFile":"C:\\project\\odd\\tasks\\feature.MD"}}}"#;
        assert_eq!(evaluate_pre_gate(json), Ok(true));

        // filePath with lowercase .md
        let json2 = r#"{"toolCall":{"name":"write_file","args":{"filePath":"odd/tasks/fix.md"}}}"#;
        assert_eq!(evaluate_pre_gate(json2), Ok(true));

        // Path with mixed case
        let json3 = r#"{"toolCall":{"name":"write_file","args":{"Path":"./odd/tasks/refactor.Md"}}}"#;
        assert_eq!(evaluate_pre_gate(json3), Ok(true));

        // targetFile with directory
        let json4 = r#"{"toolCall":{"name":"create_directory","args":{"targetFile":"odd/tasks"}}}"#;
        assert_eq!(evaluate_pre_gate(json4), Ok(true));
    }

    #[test]
    fn test_substring_exploit_denied_without_active_task() {
        // Bypass attempt: exploit filename containing "odd/tasks/" in its path but not being in odd/tasks/<spec>.md
        // e.g. src/odd/tasks/exploit.rs
        let json = r#"{"toolCall":{"name":"write_file","args":{"Path":"src/odd/tasks/exploit.rs"}},"workspacePaths":["C:\\non_existent_workspace_12345"]}"#;
        assert!(evaluate_pre_gate(json).is_err());

        // e.g. odd/tasks/something.rs (wrong extension)
        let json2 = r#"{"toolCall":{"name":"write_file","args":{"Path":"odd/tasks/something.rs"}},"workspacePaths":["C:\\non_existent_workspace_12345"]}"#;
        assert!(evaluate_pre_gate(json2).is_err());
    }

    #[test]
    fn test_path_traversal_normalization() {
        // odd/tasks/../src/main.rs normalizes to odd/src/main.rs -> not an odd/tasks spec!
        let json = r#"{"toolCall":{"name":"write_file","args":{"Path":"odd/tasks/../src/main.rs"}},"workspacePaths":["C:\\non_existent_workspace_12345"]}"#;
        assert!(evaluate_pre_gate(json).is_err());

        // Traversal that legitimately resolves to odd/tasks/spec.md
        let json2 = r#"{"toolCall":{"name":"write_file","args":{"Path":"odd/other/../tasks/spec.md"}}}"#;
        assert_eq!(evaluate_pre_gate(json2), Ok(true));
    }

    #[test]
    fn test_multiple_workspace_paths() {
        // Temp dirs for testing multiple workspaces
        let temp_dir1 = env::temp_dir().join("agy_test_ws1");
        let temp_dir2 = env::temp_dir().join("agy_test_ws2");
        let _ = fs::remove_dir_all(&temp_dir1);
        let _ = fs::remove_dir_all(&temp_dir2);

        let ws2_tasks = temp_dir2.join("odd").join("tasks");
        fs::create_dir_all(&ws2_tasks).unwrap();
        fs::write(ws2_tasks.join("task1.md"), "# Task").unwrap();

        let json = format!(
            r#"{{"toolCall":{{"name":"write_file","args":{{"Path":"src/lib.rs"}}}},"workspacePaths":["{}","{}"]}}"#,
            temp_dir1.to_string_lossy().replace('\\', "\\\\"),
            temp_dir2.to_string_lossy().replace('\\', "\\\\")
        );

        assert_eq!(evaluate_pre_gate(&json), Ok(true));

        let _ = fs::remove_dir_all(&temp_dir1);
        let _ = fs::remove_dir_all(&temp_dir2);
    }
}

