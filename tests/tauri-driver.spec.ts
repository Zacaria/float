import { test, expect } from '@playwright/test';
import { spawn } from 'child_process';
import * as path from 'path';
import * as fs from 'fs';

const DRIVER_NAME = process.platform === 'win32' ? 'tauri-driver.exe' : 'tauri-driver';

const resolveDriverPath = (): string => {
  const override = process.env.TAURI_DRIVER_PATH;
  if (override) {
    const resolved = path.resolve(override);
    if (!fs.existsSync(resolved)) {
      throw new Error(`TAURI_DRIVER_PATH set to ${resolved}, but no file exists there.`);
    }
    return resolved;
  }

  const pathEntries = process.env.PATH?.split(path.delimiter) ?? [];
  const candidates: string[] = [];
  pathEntries
    .map((segment) => segment.trim())
    .filter(Boolean)
    .forEach((segment) => {
      if (process.platform === 'win32') {
        candidates.push(
          path.join(segment, 'tauri-driver.exe'),
          path.join(segment, 'tauri-driver.cmd'),
          path.join(segment, 'tauri-driver.bat'),
        );
      } else {
        candidates.push(path.join(segment, DRIVER_NAME));
      }
    });

  for (const candidate of candidates) {
    if (fs.existsSync(candidate)) {
      return candidate;
    }
  }

  throw new Error(
    'tauri-driver binary not found in PATH. Install it with `cargo install tauri-driver --locked` or set TAURI_DRIVER_PATH to its location.',
  );
};

/**
 * Boot the Tauri app via tauri-driver and connect Playwright to it.
 * The driver listens on 5544 by default; we wait for readiness before connecting.
 */
test('opens app and shows toolbar', async ({ page }) => {
  const iconPath = path.resolve(__dirname, '..', 'src-tauri', 'icons', 'icon.png');
  if (!fs.existsSync(iconPath)) {
    throw new Error(`icon not found at ${iconPath}`);
  }

  const driverPath = resolveDriverPath();
  const driver = spawn(driverPath, [], {
    env: { ...process.env, FLOAT_TEST_PATH: iconPath },
    stdio: 'inherit',
  });

  // Give the driver time to start
  await new Promise((resolve) => setTimeout(resolve, 3000));

  await page.goto('http://localhost:5544/');

  await expect(page).toHaveTitle('Float');
  await page.waitForSelector('text=No file selected');

  driver.kill();
});
