import { test, expect } from '@playwright/test';
import { spawn } from 'child_process';
import * as path from 'path';
import * as fs from 'fs';

/**
 * Boot the Tauri app via tauri-driver and connect Playwright to it.
 * The driver listens on 5544 by default; we wait for readiness before connecting.
 */
test('opens app and shows toolbar', async ({ page }) => {
  const iconPath = path.resolve(__dirname, '..', 'src-tauri', 'icons', 'icon.png');
  if (!fs.existsSync(iconPath)) {
    throw new Error(`icon not found at ${iconPath}`);
  }

  const driver = spawn('tauri-driver', [], {
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
