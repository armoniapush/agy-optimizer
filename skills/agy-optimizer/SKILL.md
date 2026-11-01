---
name: agy-optimizer
description: "Diagnósticos, métricas de KV-cache y verificación del arnés mecánico determinista de Antigravity (ODD & Engram)."
---

# AGY Optimizer Skill

Proporciona comandos de auditoría y validación para el arnés determinista de alto rendimiento.

## Capacidades

- **Auditoría de Caché:** Verifica la inmutabilidad de los prefijos estáticos del sistema para garantizar hit-rates superiores al 90%.
- **Compuertas Mecánicas:** Inspecciona el estado del guardavías de ODD (`agy-hook.exe`).
- **Sincronización de Memoria:** Valida la conectividad con el almacén persistente de Engram.

## Comandos Operativos

- `agy-hook.exe pre-gate`: Evalúa la conformidad de la tarea activa en `odd/tasks/<feature>.md`.
- `agy-hook.exe session-init`: Dispara la sincronización en segundo plano con el historial arquitectónico de Engram.
