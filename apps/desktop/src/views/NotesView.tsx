import { useEffect, useState } from "react";
import { convertFileSrc } from "@tauri-apps/api/core";
import { api, Note } from "../api";

function NoteThumb({ path }: { path: string }) {
  const [src, setSrc] = useState<string>(() => convertFileSrc(path));
  useEffect(() => {
    setSrc(convertFileSrc(path));
    const img = new Image();
    img.onerror = () => {
      api.readCaptureDataUrl(path).then(setSrc).catch(console.error);
    };
    img.src = convertFileSrc(path);
  }, [path]);
  return <img className="lib-thumb" src={src} alt="" />;
}

export default function NotesView() {
  const [notes, setNotes] = useState<Note[]>([]);
  const [query, setQuery] = useState("");

  const load = async () => {
    if (query.trim()) setNotes(await api.searchNotes(query));
    else setNotes(await api.listNotes());
  };

  useEffect(() => {
    load().catch(console.error);
  }, []);

  return (
    <div>
      <div className="panel-title">
        <h2>Notes</h2>
      </div>
      <p className="msg">
        Notes stay linked to their screenshots — search finds either side.
      </p>
      <div className="search-row">
        <input
          value={query}
          onChange={(e) => setQuery(e.target.value)}
          placeholder="Search notes (text, tags, linked image)…"
          onKeyDown={(e) => e.key === "Enter" && load()}
        />
        <button onClick={load}>Search</button>
      </div>
      <div className="list">
        {notes.map((n) => (
          <div className="card" key={n.id}>
            {n.screenshotPath && <NoteThumb path={n.screenshotPath} />}
            <h3>{n.title}</h3>
            {n.summary && <p>{n.summary}</p>}
            <p>{n.content}</p>
            {n.tags?.length > 0 && (
              <div className="tag-row">
                {n.tags.map((t) => (
                  <span className="tag" key={t}>
                    {t}
                  </span>
                ))}
              </div>
            )}
            {n.libraryItemId && (
              <div className="lineage">
                Linked image: {n.linkedLibraryTitle || n.libraryItemId}
              </div>
            )}
            <div className="meta">
              {n.isSmart && <span>smart</span>}
              <span>{n.contentType}</span>
              <span>{new Date(n.createdAt).toLocaleString()}</span>
            </div>
            {n.screenshotPath && (
              <p className="path-line" title={n.screenshotPath}>
                {n.screenshotPath}
              </p>
            )}
            <button
              onClick={async () => {
                await api.deleteNote(n.id);
                await load();
              }}
            >
              Delete
            </button>
          </div>
        ))}
        {!notes.length && <p className="msg">No notes yet.</p>}
      </div>
    </div>
  );
}
