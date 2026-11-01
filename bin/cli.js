#!/usr/bin/env node

const fs = require('fs');
const path = require('path');
const os = require('os');
const { execSync } = require('child_process');

const HOME = os.homedir();
const CONFIG_ROOT = path.join(HOME, '.gemini', 'config');
const PLUGIN_TARGET = path.join(CONFIG_ROOT, 'plugins', 'agy-optimizer');
const BIN_TARGET = path.join(CONFIG_ROOT, 'bin');

const action = process.argv[2] || 'install';

function install() {
  console.log('\x1b[36m%s\x1b[0m', '=== AGY Optimizer CLI Installer ===');

  // 1. Ensure target directories
  fs.mkdirSync(PLUGIN_TARGET, { recursive: true });
  fs.mkdirSync(BIN_TARGET, { recursive: true });

  const rootDir = path.resolve(__dirname, '..');

  // 2. Copy binary
  const binarySource = path.join(rootDir, 'bin', 'agy-hook.exe');
  const binaryDest = path.join(BIN_TARGET, 'agy-hook.exe');
  if (fs.existsSync(binarySource)) {
    fs.copyFileSync(binarySource, binaryDest);
    console.log('\x1b[32m%s\x1b[0m', `[OK] Native binary installed: ${binaryDest}`);
  }

  // 3. Copy plugin contents into ~/.gemini/config/plugins/agy-optimizer
  const itemsToCopy = ['plugin.json', 'hooks.json', 'rules', 'skills', 'bin'];
  for (const item of itemsToCopy) {
    const src = path.join(rootDir, item);
    const dest = path.join(PLUGIN_TARGET, item);
    if (fs.existsSync(src)) {
      fs.cpSync(src, dest, { recursive: true, force: true });
    }
  }
  console.log('\x1b[32m%s\x1b[0m', `[OK] Plugin package installed: ${PLUGIN_TARGET}`);

  // 4. Also register global hooks.json fallback
  const globalHooksSrc = path.join(rootDir, 'hooks', 'hooks.json');
  const globalHooksDest = path.join(CONFIG_ROOT, 'hooks.json');
  if (fs.existsSync(globalHooksSrc)) {
    fs.copyFileSync(globalHooksSrc, globalHooksDest);
    console.log('\x1b[32m%s\x1b[0m', `[OK] Global hooks linked: ${globalHooksDest}`);
  }

  // 5. Update config.json to enable agy-optimizer
  const configJsonPath = path.join(CONFIG_ROOT, 'config.json');
  if (fs.existsSync(configJsonPath)) {
    try {
      const config = JSON.parse(fs.readFileSync(configJsonPath, 'utf8'));
      if (!config.plugins) config.plugins = {};
      if (!config.plugins['agy-optimizer']) {
        config.plugins['agy-optimizer'] = { enabled: true };
        fs.writeFileSync(configJsonPath, JSON.stringify(config, null, 2), 'utf8');
        console.log('\x1b[32m%s\x1b[0m', `[OK] Plugin enabled in config.json`);
      }
    } catch (e) {
      console.warn('Could not update config.json:', e.message);
    }
  }

  console.log('\x1b[32m%s\x1b[0m', '\nInstallation successfully completed! AGY is guarded and rot-immune.');
}

function verify() {
  console.log('\x1b[36m%s\x1b[0m', '=== AGY Optimizer Verification ===');
  const binary = path.join(BIN_TARGET, 'agy-hook.exe');
  const hooks = path.join(CONFIG_ROOT, 'hooks.json');
  const plugin = path.join(PLUGIN_TARGET, 'plugin.json');

  console.log('Binary exists:', fs.existsSync(binary));
  console.log('Hooks.json exists:', fs.existsSync(hooks));
  console.log('Plugin exists:', fs.existsSync(plugin));
}

switch (action) {
  case 'install':
    install();
    break;
  case 'verify':
    verify();
    break;
  default:
    console.log(`Unknown action: ${action}. Use 'install' or 'verify'.`);
}
