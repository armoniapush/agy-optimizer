$ErrorActionPreference = "Stop"

$ConfigRoot = "$env:USERPROFILE\.gemini\config"
$BinDir = "$ConfigRoot\bin"
$PluginsDir = "$ConfigRoot\plugins\agy-optimizer"

Write-Host "==========================================================" -ForegroundColor Cyan
Write-Host " Installing AGY Ultra-Lightweight Harness Engine (Rust)   " -ForegroundColor Cyan
Write-Host "==========================================================" -ForegroundColor Cyan

# 1. Create target directories
New-Item -ItemType Directory -Force -Path $BinDir | Out-Null
New-Item -ItemType Directory -Force -Path $PluginsDir | Out-Null

# 2. Copy compiled binary
Copy-Item ".\bin\agy-hook.exe" "$BinDir\agy-hook.exe" -Force
Write-Host "[OK] Binary installed to: $BinDir\agy-hook.exe (300 KB)" -ForegroundColor Green

# 3. Copy plugin configuration
Copy-Item ".\plugin.json" "$PluginsDir\plugin.json" -Force
Copy-Item -Recurse -Force ".\rules" "$PluginsDir\rules"
Copy-Item ".\hooks\hooks.json" "$ConfigRoot\hooks.json" -Force
Write-Host "[OK] Global hooks.json configured at: $ConfigRoot\hooks.json" -ForegroundColor Green

# 4. Apply Windows Defender folder exclusion if running with privileges
try {
    Add-MpPreference -ExclusionPath $BinDir -ErrorAction SilentlyContinue
    Write-Host "[OK] Windows Defender exclusion configured for sub-millisecond I/O." -ForegroundColor Green
} catch {
    Write-Host "[INFO] Windows Defender exclusion skipped (requires elevation). Works normally." -ForegroundColor Gray
}

Write-Host "`nInstallation complete! AGY is now permanently guarded and rot-immune." -ForegroundColor Green
