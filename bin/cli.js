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
  console.log('\x1b[36m%s\x1b[0m', '=== AGY Optimizer CLI Installer v2.1 ===');

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
  console.log('\x1b[36m%s\x1b[0m', '=== Updating Gentle-AI Shadow Vendor ===');
  if (fs.existsSync(VENDOR_TARGET)) {
    try {
      execSync('git pull origin main || git pull origin master', { cwd: VENDOR_TARGET, stdio: 'inherit' });
      console.log('\x1b[32m%s\x1b[0m', '[OK] Gentle-AI updated. JIT sync will reindex catalog on next SessionStart.');
    } catch(e) {
      console.error('Failed to pull gentle-ai:', e.message);
    }
  } else {
    console.log('\x1b[33m%s\x1b[0m', `Vendor directory not found at ${VENDOR_TARGET}. Cloning fresh...`);
    execSync(`git clone https://github.com/Gentleman-Programming/gentle-ai.git "${VENDOR_TARGET}"`, { stdio: 'inherit' });
  }
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
    updateGentle();
    break;
  case 'verify':
    verify();
    break;
  default:
    console.log(`Unknown action: ${action}. Use 'install', 'update-gentle', or 'verify'.`);
}