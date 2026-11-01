use std::fs::{self, File};
use std::io::{self, BufReader, Read};
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
    #[serde(rename = "executionNum")]
    execution_num: Option<usize>,
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

#[derive(Serialize, Deserialize, Clone, Debug)]
struct SkillMetadata {
    name: String,
    description: String,
    playbook_path: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Catalog {
    version_hash: String,
    skills: Vec<SkillMetadata>,
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
    if lower.contains("odd/tasks") || lower.contains(".odd") {
        return true;
    }
    // Documentation, markdown, notes, and catalogs
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

// Extractor léxico zero-copy de YAML frontmatter
fn parse_yaml_frontmatter(content: &str) -> Option<(String, String)> {
    if !content.starts_with("---") { return None; }
    let end_idx = content[3..].find("---")?;
    let yaml_block = &content[3..3 + end_idx];
    
    let mut name = None;
    let mut desc = None;

    for line in yaml_block.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("name:") {
            name = Some(trimmed.trim_start_matches("name:").trim().trim_matches('"').trim_matches('\'').to_string());
        } else if trimmed.starts_with("description:") {
            desc = Some(trimmed.trim_start_matches("description:").trim().trim_matches('"').trim_matches('\'').to_string());
        }
    }

    Some((name?, desc.unwrap_or_default()))
}

// JIT Virtual Projection: Sincroniza vendor/gentle-ai con references/catalog.json en <1.5ms
fn sync_vendor_skills() {
    let home = std::env::var("USERPROFILE").or_else(|_| std::env::var("HOME")).unwrap_or_default();
    if home.is_empty() { return; }

    let vendor_dir = PathBuf::from(&home).join(".gemini").join("vendor").join("gentle-ai");
    if !vendor_dir.exists() { return; }

    let ref_dir = PathBuf::from(&home).join(".gemini").join("config").join("plugins").join("agy-optimizer").join("skills").join("gentle-gateway").join("references");
    let _ = fs::create_dir_all(&ref_dir);
    let catalog_path = ref_dir.join("catalog.json");

    let head_file = vendor_dir.join(".git").join("refs").join("heads").join("main");
    let current_hash = fs::read_to_string(&head_file).unwrap_or_else(|_| "v1.0.0".to_string());

    // Check rápido de caché: si el hash coincide, retornar en sub-100 microsegundos
    if let Ok(file) = File::open(&catalog_path) {
        let reader = BufReader::new(file);
        if let Ok(catalog) = serde_json::from_reader::<_, Catalog>(reader) {
            if catalog.version_hash == current_hash {
                return;
            }
        }
    }

    // Reindexación JIT silenciosa
    let mut skills = Vec::new();
    let skills_dir = vendor_dir.join("skills");
    if skills_dir.exists() {
        if let Ok(entries) = fs::read_dir(skills_dir) {
            for entry in entries.flatten() {
                let skill_file = entry.path().join("SKILL.md");
                if skill_file.exists() {
                    if let Ok(content) = fs::read_to_string(&skill_file) {
                        if let Some((name, desc)) = parse_yaml_frontmatter(&content) {
                            skills.push(SkillMetadata {
                                name,
                                description: desc,
                                playbook_path: skill_file.to_string_lossy().to_string(),
                            });
                        }
                    }
                }
            }
        }
    }

    let catalog = Catalog {
        version_hash: current_hash,
        skills,
    };

    if let Ok(json) = serde_json::to_string_pretty(&catalog) {
        let _ = fs::write(catalog_path, json);
    }
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

    // Shell Write Guard
    if tool_name == "run_command" {
        let cmd = args.get("CommandLine")
            .or_else(|| args.get("command"))
            .and_then(|v| v.as_str())
            .unwrap_or("");
        
        let cmd_lower = cmd.to_lowercase();
        let has_redirect = cmd_lower.contains('>') || cmd_lower.contains("out-file") 
            || cmd_lower.contains("set-content") || cmd_lower.contains("add-content")
            || cmd_lower.contains("copy-item") || cmd_lower.contains("move-item")
            || cmd_lower.contains("new-item") || cmd_lower.contains("writefilesync")
            || cmd_lower.contains("write_text") || cmd_lower.contains("| tee");

        let targets_prod = cmd_lower.contains("src/") || cmd_lower.contains("src\\") 
            || cmd_lower.contains("app/") || cmd_lower.contains("app\\")
            || cmd_lower.contains("odd/receipts");

        if has_redirect && targets_prod {
            let ws = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
            if !has_active_odd_task(&ws) || cmd_lower.contains("odd/receipts") {
                let resp = HookDecision {
                    decision: "deny".into(),
                    reason: Some("[FSM SHELL VETO]: Intento de redirección o mutación de código de producción vía terminal bloqueado. Redacta primero la especificación en odd/tasks/.".into()),
                };
                println!("{}", serde_json::to_string(&resp).unwrap());
                return;
            }
        }

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
        let path_lower = path_str.to_lowercase().replace('\\', "/");

        // Zero-Trust en odd/receipts/
        if path_lower.contains("odd/receipts") {
            let resp = HookDecision {
                decision: "deny".into(),
                reason: Some("[ZERO-TRUST VETO]: Prohibida la escritura directa en 'odd/receipts/'. Los recibos de verificación solo son emitidos por el ejecutor tras constatar el paso de la suite de pruebas en exit code 0.".into()),
            };
            println!("{}", serde_json::to_string(&resp).unwrap());
            return;
        }

        // Scaffolding permisivo
        if is_scaffolding_or_non_production(&path_str) {
            println!("{}", serde_json::to_string(&HookDecision { decision: "allow".into(), reason: None }).unwrap());
            return;
        }

        // Production code mutation requires active ODD task
        let ws = resolve_workspace_root(&target_path);
        if has_active_odd_task(&ws) {
            println!("{}", serde_json::to_string(&HookDecision { decision: "allow".into(), reason: None }).unwrap());
            return;
        }

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

    let clean_str = String::from_utf8_lossy(&buffer).trim_start_matches('\u{feff}').to_string();
    let payload: Option<StopPayload> = serde_json::from_str(&clean_str).ok();

    // Circuit Breaker (20 iteraciones)
    if let Some(ref p) = payload {
        if let Some(exec_num) = p.execution_num {
            if exec_num >= 20 {
                let resp = HookDecision {
                    decision: "allow".into(),
                    reason: Some("[CIRCUIT_BREAKER_ABORT]: Límite de 20 iteraciones agotado sin verificación ODD. Deteniendo bucle cerrado para requerir intervención del usuario.".into()),
                };
                println!("{}", serde_json::to_string(&resp).unwrap());
                return;
            }
        }
    }

    let cwd = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    
    if has_active_odd_task(&cwd) {
        let receipts_dir = cwd.join("odd").join("receipts");
        let has_receipt = fs::read_dir(receipts_dir).map(|mut d| d.next().is_some()).unwrap_or(false);

        if !has_receipt {
            let resp = HookDecision {
                decision: "continue".into(),
                reason: Some("[MANDATO DE CIERRE AUTÓNOMO]: La tarea ODD sigue activa sin recibo de verificación en 'odd/receipts/'. Continúa ejecutando hasta compilar o verificar con tests.".into()),
            };
            println!("{}", serde_json::to_string(&resp).unwrap());
            return;
        }
    }

    println!("{}", serde_json::to_string(&HookDecision { decision: "allow".into(), reason: None }).unwrap());
}

fn handle_pre_invocation() {
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
    // 1. Ejecutar JIT Virtual Projection sobre vendor/gentle-ai (<1.5ms)
    sync_vendor_skills();

    // 2. Inyectar estado
    let resp = serde_json::json!({
        "injectSteps": [
            {
                "systemMessage": {
                    "systemMessage": "[AGY OPTIMIZER CERTIFIED]: Arnés determinista en Rust v2.1 activo. JIT Vendor Sync (<2ms), FSM Coercitiva, Circuit Breaker y ReFold KV-Cache protection."
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