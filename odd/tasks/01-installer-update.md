# Spec 01: Actualización del instalador multiplataforma para agy-optimizer

## 1. Objetivo
Actualizar `install.ps1` para que registre automáticamente el plugin en `config.json` durante instalaciones en PCs nuevas, y crear `install.sh` para entornos Linux/macOS.

## 2. Archivos a tocar
- `C:\Users\armon\.gemini\antigravity\scratch\agy-optimizer\install.ps1`
- `C:\Users\armon\.gemini\antigravity\scratch\agy-optimizer\install.sh`

## 3. Test de verificación
- Ejecutar `install.ps1` en seco o validar sintaxis PowerShell con `powershell -Command "Get-Content install.ps1 | Out-Null"`.
- Validar commit y sincronización en GitHub.
