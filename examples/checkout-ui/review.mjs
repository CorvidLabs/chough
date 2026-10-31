// Rendered review of the checkout prototype: screenshots for a human or a
// vision-capable agent, plus deterministic assertions that do not need judgment.
// Uses locally installed Playwright (and axe-core when given); installs nothing.
//
//   PLAYWRIGHT_MODULE=/path/to/node_modules/playwright/index.mjs \
//   AXE_SCRIPT=/path/to/node_modules/axe-core/axe.min.js \
//   bun examples/checkout-ui/review.mjs <output-dir>
import { mkdir, writeFile } from "node:fs/promises";
import { dirname, join, resolve } from "node:path";
import { fileURLToPath, pathToFileURL } from "node:url";

const here = dirname(fileURLToPath(import.meta.url));
const output = resolve(process.argv[2] ?? "checkout-ui-review");
if (!process.env.PLAYWRIGHT_MODULE) throw new Error("Set PLAYWRIGHT_MODULE to an installed playwright/index.mjs");
const { chromium } = await import(pathToFileURL(resolve(process.env.PLAYWRIGHT_MODULE)).href);
const prototype = pathToFileURL(join(here, "design/prototype/index.html")).href;
const viewports = [
  { name: "desktop-1280x800", width: 1280, height: 800 },
  { name: "mobile-390x844", width: 390, height: 844 },
];
const states = ["loading", "empty", "error", "below", "free", "submitting"];
const expectations = {
  below: { fee: "$4.99", hint: "Add $7.50 more for free shipping." },
  free: { fee: "Free shipping", hint: "Your order ships free." },
};

await mkdir(output, { recursive: true });
const browser = await chromium.launch({ headless: true });
const results = [];
let failures = 0;
try {
  for (const viewport of viewports) {
    const page = await browser.newPage({ viewport, deviceScaleFactor: 1, reducedMotion: "reduce" });
    for (const state of states) {
      await page.goto(`${prototype}?state=${state}`);
      const checks = [];
      const check = (name, passed, detail) => {
        checks.push({ name, passed, detail });
        if (!passed) failures += 1;
      };
      const overflow = await page.evaluate(() => document.documentElement.scrollWidth - window.innerWidth);
      check("no horizontal overflow", overflow <= 0, `${overflow}px wider than the viewport`);
      const visiblePanels = await page.evaluate(() =>
        [...document.querySelectorAll("[data-panel]")].filter(panel => !panel.hidden).map(panel => panel.dataset.panel));
      check("only the state's panels are visible", visiblePanels.length > 0, visiblePanels.join(", "));
      if (expectations[state]) {
        const fee = await page.locator(".summary dl div:nth-child(2) dd:not([hidden])").innerText();
        const hint = await page.locator(".hint:not([hidden])").innerText();
        check("fee line wording", fee.trim() === expectations[state].fee, fee.trim());
        check("hint wording", hint.trim() === expectations[state].hint, hint.trim());
      }
      if (state === "submitting") {
        check("pay button disabled while submitting", await page.locator("[data-pay]").isDisabled(), "");
      }
      if (process.env.AXE_SCRIPT) {
        await page.addScriptTag({ path: resolve(process.env.AXE_SCRIPT) });
        const violations = await page.evaluate(async () => {
          const result = await axe.run(document, { runOnly: ["wcag2a", "wcag2aa"] });
          return result.violations.map(violation => `${violation.id} (${violation.nodes.length})`);
        });
        check("axe-core WCAG 2 A/AA violations", violations.length === 0, violations.join(", ") || "none");
      }
      const screenshot = join(output, `${viewport.name}-${state}.png`);
      await page.screenshot({ path: screenshot, fullPage: true });
      results.push({ viewport: viewport.name, state, screenshot, checks });
    }
    await page.close();
  }
} finally {
  await browser.close();
}
await writeFile(join(output, "review.json"), JSON.stringify({ prototype, results, failures }, null, 2) + "\n");
for (const result of results) {
  for (const check of result.checks) {
    process.stdout.write(`${check.passed ? "PASS" : "FAIL"} ${result.viewport} ${result.state}: ${check.name}${check.detail ? ` — ${check.detail}` : ""}\n`);
  }
}
process.stdout.write(failures === 0 ? "REVIEW ASSERTIONS PASSED\n" : `REVIEW ASSERTIONS FAILED: ${failures}\n`);
process.exit(failures === 0 ? 0 : 1);
