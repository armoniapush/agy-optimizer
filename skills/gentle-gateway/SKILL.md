---
name: gentle-gateway
description: >-
  Orquestador de ingenieria Gentle-AI (ODD, RDD, Engram, TDD).
  Activalo cuando el usuario pida disenar specs, sincronizar memoria,
  crear pruebas o implementar funcionalidades de software.
---
# Gentle-AI Gateway

Tienes acceso a la biblioteca modular de playbooks de Gentle-AI en:
`references/catalog.json`

## Protocolo de Ejecucion:
1. No cargues todos los playbooks en memoria.
2. Consulta `references/catalog.json` con `read_file` para identificar el playbook exacto para tu fase actual (ej. `odd-spec`, `rdd-defect-workflow`, `chained-pr`).
3. Lee el playbook seleccionado desde su ruta en disco únicamente cuando vayas a ejecutar dicha fase.