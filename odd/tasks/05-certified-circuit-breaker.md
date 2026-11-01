# Spec ODD: 05-certified-circuit-breaker

## 1. Objetivo
Implementar las 3 salvaguardas obligatorias de certificación de la VM de Google:
1. **Circuit Breaker en Stop Gate**: Límite estricto de 20 iteraciones (`executionNum >= 20`) para prevenir bucles infinitos y drenaje de tokens ante fallos externos irresolubles.
2. **Zero-Trust en `odd/receipts/` (Anti-Forgery Guard)**: Veto estricto en `PreToolUse` ante cualquier intento del modelo de escribir directamente en la carpeta de recibos.
3. **Shell Write Guard en `run_command`**: Inspección léxica de `CommandLine` en `PreToolUse` para bloquear redirecciones (`>`, `>>`, `Out-File`, `Set-Content`, `tee`) dirigidas a `src/` si no hay spec ODD activa.

## 2. Archivos a tocar
- `src/main.rs`: Lógica en Rust compilada con las 3 salvaguardas.
- `bin/agy-hook.exe`: Binario distribuible.
- Copias en `~/.gemini/config/bin/agy-hook.exe` y plugins.

## 3. Test de verificación
- Test de veto de escritura directa en `odd/receipts/fake.json`.
- Test de veto de comando `run_command` con `Set-Content` hacia `src/` sin spec.
- Test de circuit breaker al llegar a `executionNum: 20`.