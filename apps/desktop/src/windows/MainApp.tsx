import { useEffect, useState } from "react";
import { api } from "../api";
import LibraryView from "../views/LibraryView";
import NotesView from "../views/NotesView";
import HistoryView from "../views/HistoryView";
import SettingsView from "../views/SettingsView";
import "./main.css";

type Tab = "home" | "library" | "notes" | "history" | "settings";

export default function MainApp() {
  const [tab, setTab] = useState<Tab>("home");
  const [ocrOk, setOcrOk] = useState<boolean | null>(null);

  useEffect(() => {
    document.documentElement.classList.add("main-window");
    api.ocrAvailable().then(setOcrOk).catch(() => setOcrOk(false));
  }, []);

  return (
    <div className="app-shell">
      <aside className="sidebar">
        <div className="brand">Scoop</div>
        <p className="tagline">Select → Understand → Act → Save</p>
        <nav>
          {(
            [
              ["home", "Home"],
              ["library", "Library"],
              ["notes", "Notes"],
              ["history", "History"],
              ["settings", "Settings"],
            ] as const
          ).map(([id, label]) => (
            <button
              key={id}
              className={tab === id ? "nav active" : "nav"}
              onClick={() => setTab(id)}
            >
              {label}
            </button>
          ))}
        </nav>
        <button className="select-cta" onClick={() => api.startSelection()}>
          Start selection
        </button>
        <p className="hotkey-hint">Default hotkey: Super+Shift+Space</p>
      </aside>
      <main className="content">
        {tab === "home" && (
          <section className="home">
            <h1>AI visual workspace for Linux</h1>
            <p>
              Scoop sits above your desktop. Select anything on screen, then
              search, calculate, ask AI, or save it to your local library.
            </p>
            <ul>
              <li>Global hotkey → marquee select</li>
              <li>OCR + contextual actions</li>
              <li>Local library & notes with search</li>
              <li>Privacy-first: captures stay temporary until you save</li>
            </ul>
            {ocrOk === false && (
              <p className="warn-box">
                OCR is not available: install Tesseract, then restart Scoop.
                <br />
                <code>make install-ocr</code>
                {"  or  "}
                <code>sudo apt install tesseract-ocr tesseract-ocr-eng</code>
              </p>
            )}
            {ocrOk === true && (
              <p className="ok-box">OCR ready (Tesseract detected).</p>
            )}
          </section>
        )}
        {tab === "library" && <LibraryView />}
        {tab === "notes" && <NotesView />}
        {tab === "history" && <HistoryView />}
        {tab === "settings" && <SettingsView />}
      </main>
    </div>
  );
}
