# Spec: Empaquetado Dual (Plugin Autocontenido + CLI NPX) y Auditoría de Integridad

## 1. Objetivo
1. Transformar `agy-optimizer` en un Plugin 100% autocontenido en `~/.gemini/config/plugins/agy-optimizer` con su propio `hooks.json` interno y binario sin depender de archivos dispersos en `config/bin`.
2. Crear un paquete npm para distribución vía `npx agy-optimizer install`.
3. Auditar duplicaciones, paths erróneos y malas señales en toda la configuración global.

## 2. Archivos a tocar
- `C:\Users\armon\.gemini\antigravity\scratch\agy-optimizer\package.json`
- `C:\Users\armon\.gemini\antigravity\scratch\agy-optimizer\bin\cli.js`
- `C:\Users\armon\.gemini\antigravity\scratch\agy-optimizer\hooks.json`
- `C:\Users\armon\.gemini\antigravity\scratch\agy-optimizer\skills\agy-optimizer\SKILL.md`
- `C:\Users\armon\.gemini\config\plugins\agy-optimizer\*`

## 3. Test de verificación
- Probar ejecución del CLI npx localmente (`node bin/cli.js install`).
- Verificar que `hooks.json` del plugin se cargue correctamente sin romper los demás plugins.
- Sincronizar en GitHub.