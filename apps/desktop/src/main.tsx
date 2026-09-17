import React from "react";
import ReactDOM from "react-dom/client";
import { getCurrentWindow } from "@tauri-apps/api/window";
appBootstrap();

async function appBootstrap() {
  const label = getCurrentWindow().label;
  const mod =
    label === "overlay"
      ? await import("./windows/Overlay")
      : label === "toolbar"
        ? await import("./windows/Toolbar")
        : await import("./windows/MainApp");

  const App = mod.default;
  ReactDOM.createRoot(document.getElementById("root")!).render(
    <React.StrictMode>
      <App />
    </React.StrictMode>,
  );
}
