Write-Host "Iniciando sincronizacion inteligente de Gentle-AI..." -ForegroundColor Cyan
node (Join-Path $PSScriptRoot "bin\cli.js") update-gentle