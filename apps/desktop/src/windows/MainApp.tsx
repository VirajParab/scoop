import { useEffect, useState } from "react";
import { convertFileSrc } from "@tauri-apps/api/core";
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

  useEffect(() => {
    document.documentElement.classList.add("main-window");
    api.ocrAvailable().then(setOcrOk).catch(() => setOcrOk(false));
  }, []);

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

            <div className="search-row home-search">
              <input
                value={query}
                onChange={(e) => setQuery(e.target.value)}
                placeholder="Search images + notes together…"
                onKeyDown={(e) => e.key === "Enter" && runSearch()}
              />
              <button disabled={searching} onClick={runSearch}>
                {searching ? "Searching…" : "Search"}
              </button>
            </div>

            {hits.length > 0 && (
              <div className="list home-hits">
                {hits.map((hit) => (
                  <div className="card" key={`${hit.kind}-${hit.id}`}>
                    {hit.screenshotPath && (
                      <HitThumb path={hit.screenshotPath} />
                    )}
                    <div className="meta">
                      <span className="pill-kind">{hit.kind}</span>
                      <span>{new Date(hit.createdAt).toLocaleString()}</span>
                    </div>
                    <h3>{hit.title}</h3>
                    <p>{hit.snippet || "(no text)"}</p>
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
                        Linked {hit.linkedKind}: {hit.linkedTitle || hit.linkedId}
                      </div>
                    )}
                    {hit.screenshotPath && (
                      <p className="path-line">{hit.screenshotPath}</p>
                    )}
                  </div>
                ))}
              </div>
            )}

            {!hits.length && (
              <>
                <ul>
                  <li>Global hotkey → marquee select</li>
                  <li>OCR + contextual actions</li>
                  <li>Images and notes stay linked for search</li>
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
              </>
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
