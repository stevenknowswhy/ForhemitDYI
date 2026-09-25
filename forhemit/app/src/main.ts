import { mount } from "svelte";
import App from "./App.svelte";

const target = document.getElementById("app");
if (!target) {
  throw new Error("#app mount point missing from index.html");
}

// Surfacing uncaught errors on the page itself — the dev harness has no
// devtools console, and a render error that silently tears the walk must
// be visible, never swallowed.
const errorBanner = document.createElement("pre");
errorBanner.style.cssText =
  "position:fixed;bottom:0;left:0;right:0;max-height:40vh;overflow:auto;" +
  "background:#b3261e;color:#fff;padding:8px 12px;font-size:11px;z-index:9999;" +
  "white-space:pre-wrap;display:none;margin:0";
document.body.appendChild(errorBanner);
const show = (kind: string, detail: unknown) => {
  errorBanner.style.display = "block";
  const stack = detail instanceof Error ? `\n${detail.stack}` : "";
  errorBanner.textContent += `[${kind}] ${String(detail)}${stack}\n`;
};
window.addEventListener("error", (event) => show("error", event.message ?? event.error));
window.addEventListener("unhandledrejection", (event) => show("rejection", event.reason));

export default mount(App, { target });
