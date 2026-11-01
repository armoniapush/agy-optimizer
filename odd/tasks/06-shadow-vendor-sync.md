# Spec ODD: 06-shadow-vendor-sync

## 1. Objetivo
Implementar la arquitectura de "JIT Virtual Projection sobre Vendor Desacoplado" (Shadow Vendor):
1. Ubicar el repositorio upstream de Alan en `~/.gemini/vendor/gentle-ai` (fuera del descubrimiento de Antigravity).
2. En `agy-hook.exe session-init`, ejecutar la función `sync_vendor_skills`:
   - Comparar el hash de Git (`.git/refs/heads/main` o master) contra `catalog.json`.
   - Si no hay cambios, salir en <0.2ms.
   - Si hubo cambios post `git pull`, escanear `vendor/gentle-ai/skills/*/SKILL.md`, extraer `name` y `description` y compilar `catalog.json` en `plugins/agy-optimizer/skills/gentle-gateway/references/catalog.json`.
3. Mantener **una sola skill visible** en Antigravity (`gentle-gateway`) protegiendo el 100% de la KV-cache.
4. Crear la carpeta vendor inicial y mover el contenido de Gentle AI allí.

## 2. Archivos a tocar
- `src/main.rs`: Función `sync_vendor_skills` y `parse_yaml_frontmatter`.
- `skills/gentle-gateway/SKILL.md`: Router descriptivo.
- `bin/cli.js`: Añadir comando `update-gentle` que ejecuta `git pull` en la carpeta vendor.
- `C:\Users\armon\.gemini\vendor\gentle-ai`: Directorio vendor desacoplado.

## 3. Test de verificación
- Probar `session-init` con `sync_vendor_skills` midiendo tiempo (<2ms).
- Verificar que `catalog.json` se genere con las skills disponibles.
- Constatar que Antigravity solo indexa `gentle-gateway`.