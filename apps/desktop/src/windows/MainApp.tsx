import { useEffect, useState } from "react";
import { convertFileSrc } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { api, SearchHit } from "../api";
import LibraryView from "../views/LibraryView";
import NotesView from "../views/NotesView";
import HistoryView from "../views/HistoryView";
import SettingsView from "../views/SettingsView";
import "./main.css";

type Tab = "home" | "library" | "notes" | "history" | "settings";

function HitThumb({ path }: { path: string }) {
  const [src, setSrc] = useState(() => convertFileSrc(path));
  useEffect(() => {
    setSrc(convertFileSrc(path));
  }, [path]);
  return <img className="hit-thumb" src={src} alt="" />;
}

export default function MainApp() {
  const [tab, setTab] = useState<Tab>("home");
  const [ocrOk, setOcrOk] = useState<boolean | null>(null);
  const [query, setQuery] = useState("");
  const [hits, setHits] = useState<SearchHit[]>([]);
  const [searching, setSearching] = useState(false);
  const [flash, setFlash] = useState("");

  useEffect(() => {
    document.documentElement.classList.add("main-window");
    api.ocrAvailable().then(setOcrOk).catch(() => setOcrOk(false));

    const unsubs = [
      listen("library-saved", () => {
        setFlash("Saved to Library — linked and searchable");
        setTab("library");
      }),
      listen("note-saved", () => {
        setFlash("Note saved — linked to its screenshot");
        setTab("notes");
      }),
    ];
    return () => unsubs.forEach((p) => p.then((u) => u()));
  }, []);

  useEffect(() => {
    if (!flash) return;
    const t = window.setTimeout(() => setFlash(""), 4200);
    return () => window.clearTimeout(t);
  }, [flash]);

  const runSearch = async () => {
    setSearching(true);
    try {
      setHits(await api.searchAll(query.trim()));
    } catch (e) {
      console.error(e);
      setHits([]);
    } finally {
      setSearching(false);
    }
  };

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
        <div className="sidebar-foot">
          {ocrOk === true && (
            <div className="ocr-chip scoop-status-ok">OCR ready</div>
          )}
          {ocrOk === false && (
            <div className="ocr-chip scoop-status-bad">OCR missing</div>
          )}
          <button className="select-cta" onClick={() => api.startSelection()}>
            Start selection
          </button>
          <p className="hotkey-hint">Super + Shift + Space</p>
        </div>
      </aside>

      <main className="content">
        {flash && <div className="clipboard-hint flash-banner">{flash}</div>}

        {tab === "home" && (
          <section className="home-hero">
            <p className="home-kicker">AI visual workspace for Linux</p>
            <h1 className="home-brand">Scoop</h1>
            <p className="home-lead">
              Select anything on screen. Understand it. Act on it. Save it —
              images and notes stay linked so you can find them later.
            </p>

            <div className="home-cta-row">
              <button
                className="scoop-btn scoop-btn-primary"
                onClick={() => api.startSelection()}
              >
                Start selection
              </button>
              <button
                className="scoop-btn scoop-btn-secondary"
                onClick={() => setTab("library")}
              >
                Open library
              </button>
            </div>

            <div className="home-search">
              <input
                className="scoop-input"
                value={query}
                onChange={(e) => setQuery(e.target.value)}
                placeholder="Search images, notes, tags…"
                onKeyDown={(e) => e.key === "Enter" && runSearch()}
              />
              <button
                className="scoop-btn scoop-btn-secondary"
                disabled={searching}
                onClick={runSearch}
              >
                {searching ? "Searching…" : "Search"}
              </button>
            </div>

            {hits.length > 0 && (
              <div className="home-hits">
                {hits.map((hit) => (
                  <article className="hit-row" key={`${hit.kind}-${hit.id}`}>
                    {hit.screenshotPath ? (
                      <HitThumb path={hit.screenshotPath} />
                    ) : (
                      <div className="hit-thumb" />
                    )}
                    <div className="hit-body">
                      <div className="hit-meta">
                        <span className="scoop-pill">{hit.kind}</span>
                        <span className="scoop-hint">
                          {new Date(hit.createdAt).toLocaleString()}
                        </span>
                      </div>
                      <h3>{hit.title}</h3>
                      <p>{hit.snippet || "No text"}</p>
                      {hit.tags?.length > 0 && (
                        <div className="tag-row">
                          {hit.tags.map((t) => (
                            <span className="tag" key={t}>
                              {t}
                            </span>
                          ))}
                        </div>
                      )}
                      {hit.linkedId && (
                        <div className="lineage">
                          Linked {hit.linkedKind}:{" "}
                          {hit.linkedTitle || hit.linkedId}
                        </div>
                      )}
                    </div>
                  </article>
                ))}
              </div>
            )}

            {!hits.length && ocrOk === false && (
              <p className="warn-box">
                Install Tesseract for OCR, then restart Scoop.
                <br />
                <code>make install-ocr</code>
              </p>
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
