import { useEffect, useState } from "react";
import { api, Note } from "../api";

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
      <div className="search-row">
        <input
          value={query}
          onChange={(e) => setQuery(e.target.value)}
          placeholder="Search notes…"
          onKeyDown={(e) => e.key === "Enter" && load()}
        />
        <button onClick={load}>Search</button>
      </div>
      <div className="list">
        {notes.map((n) => (
          <div className="card" key={n.id}>
            <h3>{n.title}</h3>
            {n.summary && <p>{n.summary}</p>}
            <p>{n.content}</p>
            <div className="meta">
              {n.isSmart && <span>smart</span>}
              <span>{n.contentType}</span>
              <span>{n.tags.join(", ")}</span>
              <span>{new Date(n.createdAt).toLocaleString()}</span>
            </div>
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
