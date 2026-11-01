# Spec ODD: 07-auto-updater-analyzer

## 1. Objetivo
Crear un comando y script de automatización inteligente (`sync-gentle` / `update-gentle`):
1. Ejecuta `git pull` silencioso en el repositorio vendor `~/.gemini/vendor/gentle-ai`.
2. Si no hay cambios, reporta `Already up to date` en 0.2s.
3. Si hay cambios:
   - Extrae el diff del último commit (`git diff HEAD@{1} HEAD` o `git log -1 --stat`).
   - Analiza qué cambió (nuevas skills, cambios en prompts ODD, mejoras en Engram o linters).
   - Ejecuta `agy-hook.exe session-init` para compilar de inmediato el nuevo `catalog.json`.
   - Genera un resumen ejecutivo de alta señal: "Qué añadió Alan y cómo impacta en nuestro arnés".

## 2. Archivos a tocar
- `bin/cli.js`: Implementación de `sync-gentle` / `update-gentle`.
- `bin/agy-sync.ps1`: Script PowerShell 1-clic directo.
- `package.json`: Mapear script npm `npm run sync-gentle`.

## 3. Test de verificación
- Probar `node bin/cli.js update-gentle` y verificar la salida del analizador.
- Verificar tiempo de ejecución (<2 segundos).