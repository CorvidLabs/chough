// Incidental documentation tooling. The product remains Rust + Markdown.
// Use installed Mermaid, Playwright and Chromium; this does not install packages.
import { readFile, readdir, writeFile, mkdir } from "node:fs/promises";
import { dirname, join, resolve } from "node:path";
import { fileURLToPath, pathToFileURL } from "node:url";

const diagramDir = dirname(fileURLToPath(import.meta.url));
const mermaidScript = process.env.MERMAID_SCRIPT;
const playwrightModule = process.env.PLAYWRIGHT_MODULE;
if (!mermaidScript || !playwrightModule) {
    throw new Error("Set MERMAID_SCRIPT and PLAYWRIGHT_MODULE to installed local modules. See docs/verification.md.");
}
const { chromium } = await import(pathToFileURL(resolve(playwrightModule)).href);
const screenshotDir = process.env.CHOUGH_DIAGRAM_SCREENSHOTS;
if (screenshotDir) await mkdir(screenshotDir, { recursive: true });
const browser = await chromium.launch({
    headless: true,
    ...(process.env.CHROMIUM_EXECUTABLE ? { executablePath: process.env.CHROMIUM_EXECUTABLE } : {})
});
try {
    const page = await browser.newPage({ viewport: { width: 1600, height: 1600 }, deviceScaleFactor: 1 });
    await page.setContent('<!doctype html><html><head><meta charset="utf-8"></head><body style="margin:24px;background:white"><main id="diagram"></main></body></html>');
    await page.addScriptTag({ path: resolve(mermaidScript) });
    await page.evaluate(() => mermaid.initialize({
        startOnLoad: false,
        securityLevel: "strict",
        theme: "base",
        themeVariables: {
            fontFamily: "Arial, sans-serif",
            fontSize: "16px",
            primaryColor: "#f0f5ee",
            primaryTextColor: "#1d3024",
            primaryBorderColor: "#527252",
            lineColor: "#526458",
            secondaryColor: "#f8f5ed",
            tertiaryColor: "#f0f3f7"
        },
        flowchart: { htmlLabels: false, curve: "linear", nodeSpacing: 28, rankSpacing: 36, padding: 16 }
    }));
    const names = (await readdir(diagramDir)).filter(name => name.endsWith(".mmd")).sort();
    for (const name of names) {
        const source = await readFile(join(diagramDir, name), "utf8");
        const id = name.slice(0, -4);
        const svg = await page.evaluate(async ({ source, id }) => {
            const result = await mermaid.render(`chough-${id}`, source);
            document.querySelector("#diagram").innerHTML = result.svg;
            return result.svg;
        }, { source, id });
        await writeFile(join(diagramDir, `${id}.svg`), svg);
        const bounds = await page.locator("#diagram svg").boundingBox();
        if (!bounds || bounds.width < 100 || bounds.height < 100) throw new Error(`Invalid diagram bounds: ${name}`);
        if (screenshotDir) await page.locator("#diagram svg").screenshot({ path: join(screenshotDir, `${id}.png`) });
        process.stdout.write(`${name}: rendered ${Math.round(bounds.width)} x ${Math.round(bounds.height)}\n`);
    }
} finally {
    await browser.close();
}
