# Spec ODD: 04-fsm-stop-hook

## 1. Objetivo
Implementar la arquitectura completa validada por la VM de Google y la documentación oficial de Antigravity:
1. `PreToolUse`: Permitir libremente scaffolding (configs, dependencias, types, tests, notas) y bloquear mutaciones a `src/` o código de producción únicamente si no hay una especificación ODD activa. Retornar payload semántico estructurado.
2. `Stop` Hook Coercitivo: Si el modelo intenta finalizar (`NO_TOOL_CALL`) pero hay una tarea ODD activa sin completar o los tests no están en verde, el hook retorna `{"decision": "continue", "reason": "Tarea ODD en progreso. Continúa la ejecución hasta verificar la solución."}` impidiendo el confirmitis.
3. `PreInvocation`: Inyectar mensajes efímeros (`ephemeralMessage`) según la fase activa sin contaminar el transcript histórico permanente (preservando KV-cache).
4. `SessionStart`: Inyectar contexto silencioso y autorizar autonomía total.

## 2. Archivos a tocar
- `src/main.rs`: Recompilar con soporte para subcomandos `pre-gate`, `stop-gate`, `pre-invocation` y `session-init`.
- `hooks.json` y `plugin.json`: Mapear `PreToolUse`, `Stop`, `PreInvocation` y `SessionStart`.
- `C:\Users\armon\.gemini\config\bin\agy-hook.exe`: Binario actualizado.

## 3. Test de verificación
- Probar llamadas directas por stdin en Rust a cada subcomando.
- Verificar que `PreToolUse` apruebe `package.json` y bloquee `src/main.rs` si no hay spec.
- Verificar que `Stop` retorne `{"decision": "continue"}` cuando haya un spec pendiente.