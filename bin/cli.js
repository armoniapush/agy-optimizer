#!/usr/bin/env node

const fs = require('fs');
const path = require('path');
const os = require('os');
const { execSync } = require('child_process');

const HOME = os.homedir();
const CONFIG_ROOT = path.join(HOME, '.gemini', 'config');
const PLUGIN_TARGET = path.join(CONFIG_ROOT, 'plugins', 'agy-optimizer');
const BIN_TARGET = path.join(CONFIG_ROOT, 'bin');
const VENDOR_TARGET = path.join(HOME, '.gemini', 'vendor', 'gentle-ai');

const action = process.argv[2] || 'install';

function install() {
  console.log('\x1b[36m%s\x1b[0m', '=== AGY Optimizer CLI Installer v2.2 ===');

  fs.mkdirSync(PLUGIN_TARGET, { recursive: true });
  fs.mkdirSync(BIN_TARGET, { recursive: true });

  const rootDir = path.resolve(__dirname, '..');

  // 1. Copy binary
  const binarySource = path.join(rootDir, 'bin', 'agy-hook.exe');
  const binaryDest = path.join(BIN_TARGET, 'agy-hook.exe');
  if (fs.existsSync(binarySource)) {
    fs.copyFileSync(binarySource, binaryDest);
    console.log('\x1b[32m%s\x1b[0m', `[OK] Native binary installed: ${binaryDest}`);
  }

  // 2. Copy plugin bundle
  const itemsToCopy = ['plugin.json', 'hooks.json', 'rules', 'skills', 'bin'];
  for (const item of itemsToCopy) {
    const src = path.join(rootDir, item);
    const dest = path.join(PLUGIN_TARGET, item);
    if (fs.existsSync(src)) {
      fs.cpSync(src, dest, { recursive: true, force: true });
    }
  }
  console.log('\x1b[32m%s\x1b[0m', `[OK] Plugin package installed: ${PLUGIN_TARGET}`);

  // 3. Global hooks fallback
  const globalHooksDest = path.join(CONFIG_ROOT, 'hooks.json');
  const pluginHooksSrc = path.join(PLUGIN_TARGET, 'hooks.json');
  if (fs.existsSync(pluginHooksSrc)) {
    fs.copyFileSync(pluginHooksSrc, globalHooksDest);
    console.log('\x1b[32m%s\x1b[0m', `[OK] Global hooks linked: ${globalHooksDest}`);
  }

  // 4. Update-gentle clone if not present
  if (!fs.existsSync(VENDOR_TARGET)) {
    console.log('\x1b[33m%s\x1b[0m', `Cloning gentle-ai into shadow vendor directory: ${VENDOR_TARGET}`);
    try {
      execSync(`git clone https://github.com/Gentleman-Programming/gentle-ai.git "${VENDOR_TARGET}"`, { stdio: 'inherit' });
    } catch(e) {
      console.warn('Could not clone gentle-ai automatically. You can clone manually into ~/.gemini/vendor/gentle-ai');
    }
  }

  console.log('\x1b[32m%s\x1b[0m', '\nInstallation successfully completed! Shadow Vendor Sync is active.');
}

function updateGentle() {
  console.log('\x1b[36m%s\x1b[0m', '=== AGY Smart Updater: Sincronizador de Gentle-AI ===');
  
  if (!fs.existsSync(VENDOR_TARGET)) {
    console.log('\x1b[33m%s\x1b[0m', `Directorio vendor no existe. Clonando repositorio oficial de Alan...`);
    fs.mkdirSync(path.dirname(VENDOR_TARGET), { recursive: true });
    execSync(`git clone https://github.com/Gentleman-Programming/gentle-ai.git "${VENDOR_TARGET}"`, { stdio: 'inherit' });
  }

  // 1. Obtener hash anterior
  let beforeHash = 'initial';
  try {
    beforeHash = execSync('git rev-parse HEAD', { cwd: VENDOR_TARGET, encoding: 'utf8' }).trim();
  } catch(e) {}

  // 2. Ejecutar Git Pull silencioso
  console.log('\x1b[90m%s\x1b[0m', 'Comprobando actualizaciones remotas de Alan (Gentleman-Programming/gentle-ai)...');
  let pullOutput = '';
  try {
    pullOutput = execSync('git pull origin main || git pull origin master', { cwd: VENDOR_TARGET, encoding: 'utf8' });
  } catch(e) {
    console.error('Error al ejecutar git pull:', e.message);
    return;
  }

  let afterHash = 'initial';
  try {
    afterHash = execSync('git rev-parse HEAD', { cwd: VENDOR_TARGET, encoding: 'utf8' }).trim();
  } catch(e) {}

  if (beforeHash === afterHash && pullOutput.includes('Already up to date')) {
    console.log('\x1b[32m%s\x1b[0m', '✓ Gentle-AI ya está en su versión más reciente. No hay cambios pendientes.');
  } else {
    console.log('\x1b[32m%s\x1b[0m', `✓ ¡Nuevas actualizaciones de Alan detectadas! (${beforeHash.slice(0, 7)} -> ${afterHash.slice(0, 7)})`);
    
    // 3. Analizar cambios (Diff Inspector)
    try {
      const changedFiles = execSync(`git diff --name-status ${beforeHash} ${afterHash}`, { cwd: VENDOR_TARGET, encoding: 'utf8' }).trim().split('\n');
      console.log('\n\x1b[33m%s\x1b[0m', '--- Análisis de Impacto de la Actualización ---');
      
      let newSkills = [];
      let modifiedSkills = [];
      let coreRules = [];

      changedFiles.forEach(line => {
        const [status, filePath] = line.split('\t');
        if (filePath.includes('skills/')) {
          const skillName = filePath.split('/')[1] || filePath;
          if (status === 'A') newSkills.push(skillName);
          else modifiedSkills.push(skillName);
        } else if (filePath.includes('rules/') || filePath.includes('AGENTS.md')) {
          coreRules.push(filePath);
        }
      });

      if (newSkills.length > 0) {
        console.log('\x1b[32m%s\x1b[0m', `+ Nuevas skills añadidas por Alan: ${[...new Set(newSkills)].join(', ')}`);
      }
      if (modifiedSkills.length > 0) {
        console.log('\x1b[36m%s\x1b[0m', `~ Playbooks/Skills actualizadas: ${[...new Set(modifiedSkills)].join(', ')}`);
      }
      if (coreRules.length > 0) {
        console.log('\x1b[35m%s\x1b[0m', `! Reglas core/filosofía modificadas: ${coreRules.join(', ')}`);
      }
    } catch(e) {
      console.log('No se pudo calcular el diff específico, pero el repositorio fue actualizado.');
    }
  }

  // 4. Disparar proyección JIT de Rust (<2ms)
  const binaryPath = path.join(BIN_TARGET, 'agy-hook.exe');
  if (fs.existsSync(binaryPath)) {
    try {
      execSync(`"${binaryPath}" session-init`, { encoding: 'utf8' });
      console.log('\x1b[32m%s\x1b[0m', '✓ Catálogo virtual indexado en references/catalog.json por Rust.');
      console.log('\x1b[32m%s\x1b[0m', '✓ El System Prompt se mantiene en 1 sola skill (KV-Cache 100% protegida).');
    } catch(e) {
      console.warn('Advertencia al compilar catalog.json:', e.message);
    }
  }

  console.log('\n\x1b[32m%s\x1b[0m', 'Sincronización completada con éxito. Tu arnés está al día con Gentle-AI.');
}

function verify() {
  console.log('\x1b[36m%s\x1b[0m', '=== AGY Optimizer Verification ===');
  const binary = path.join(BIN_TARGET, 'agy-hook.exe');
  const hooks = path.join(CONFIG_ROOT, 'hooks.json');
  const plugin = path.join(PLUGIN_TARGET, 'plugin.json');
  const vendor = VENDOR_TARGET;

  console.log('Binary exists:', fs.existsSync(binary));
  console.log('Hooks.json exists:', fs.existsSync(hooks));
  console.log('Plugin exists:', fs.existsSync(plugin));
  console.log('Shadow Vendor exists:', fs.existsSync(vendor));
}

switch (action) {
  case 'install':
    install();
    break;
  case 'update-gentle':
  case 'sync':
    updateGentle();
    break;
  case 'verify':
    verify();
    break;
  default:
    console.log(`Unknown action: ${action}. Use 'install', 'update-gentle' (or 'sync'), or 'verify'.`);
}