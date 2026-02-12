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

const launchDriver = (iconPath: string) => {
  const driverPath = resolveDriverPath();
  return spawn(driverPath, [], {
    env: { ...process.env, FLOAT_TEST_PATH: iconPath },
    stdio: 'inherit',
  });
};

const getSettings = (page: import('@playwright/test').Page) =>
  page.evaluate(async () => {
    const tauri = (window as any).__TAURI__ || {};
    if (tauri?.core?.invoke) return tauri.core.invoke('get_settings');
    if (tauri.invoke) return tauri.invoke('get_settings');
    return null;
  });

/**
 * Boot the Tauri app via tauri-driver and connect Playwright to it.
 * The driver listens on 5544 by default; we wait for readiness before connecting.
 */
test('opens app and shows toolbar', async ({ page }) => {
  const iconPath = path.resolve(__dirname, '..', 'src-tauri', 'icons', 'icon.png');
  if (!fs.existsSync(iconPath)) {
    throw new Error(`icon not found at ${iconPath}`);
  }

  const driver = launchDriver(iconPath);

  // Give the driver time to start
  await new Promise((resolve) => setTimeout(resolve, 3000));

  await page.goto('http://localhost:5544/');

  await expect(page).toHaveTitle('Float');
  await page.waitForSelector('text=No file selected');

  driver.kill();
});

test('toggles click-through with shortcut', async ({ page }) => {
  const iconPath = path.resolve(__dirname, '..', 'src-tauri', 'icons', 'icon.png');
  if (!fs.existsSync(iconPath)) {
    throw new Error(`icon not found at ${iconPath}`);
  }

  const driver = launchDriver(iconPath);

  await new Promise((resolve) => setTimeout(resolve, 3000));

  await page.goto('http://localhost:5544/');
  await expect(page).toHaveTitle('Float');
  await page.waitForSelector('text=No file selected');

  await page.click('body');

  const shortcut = process.platform === 'darwin' ? 'Meta+Shift+C' : 'Control+Shift+C';

  let settings = (await getSettings(page)) as any;
  expect(settings?.click_through).toBeFalsy();

  await page.keyboard.press(shortcut);
  await page.waitForTimeout(200);

  settings = (await getSettings(page)) as any;
  expect(settings?.click_through).toBe(true);

  await page.keyboard.press(shortcut);
  await page.waitForTimeout(200);

  settings = (await getSettings(page)) as any;
  expect(settings?.click_through).toBe(false);

  driver.kill();
});
