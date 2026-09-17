import { useEffect, useState } from "react";
import { convertFileSrc } from "@tauri-apps/api/core";
import { api, Collection, LibraryItem } from "../api";

function LibraryThumb({ path }: { path: string }) {
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

export default function LibraryView() {
  const [collections, setCollections] = useState<Collection[]>([]);
  const [items, setItems] = useState<LibraryItem[]>([]);
  const [active, setActive] = useState<string | "all">("all");
  const [query, setQuery] = useState("");
  const [newCol, setNewCol] = useState("");

  const load = async () => {
    setCollections(await api.listCollections());
    if (query.trim()) {
      setItems(await api.searchLibrary(query));
    } else if (active === "all") {
      setItems(await api.listLibrary());
    } else {
      setItems(await api.listLibrary(active));
    }
  };

  useEffect(() => {
    load().catch(console.error);
  }, [active]);

  return (
    <div>
      <div className="panel-title">
        <h2>Library</h2>
      </div>
      <p className="panel-lead">
        Screenshots linked to notes — search by text, tags, or filename.
      </p>
      <div className="search-row">
        <input
          value={query}
          onChange={(e) => setQuery(e.target.value)}
          placeholder="Search library (text, tags, image)…"
          onKeyDown={(e) => e.key === "Enter" && load()}
        />
        <button onClick={load}>Search</button>
      </div>
      <div className="collections">
        <button
          className={active === "all" ? "active" : ""}
          onClick={() => setActive("all")}
        >
          All
        </button>
        {collections.map((c) => (
          <button
            key={c.id}
            className={active === c.id ? "active" : ""}
            onClick={() => setActive(c.id)}
          >
            {c.name}
          </button>
        ))}
      </div>
      <div className="search-row">
        <input
          value={newCol}
          onChange={(e) => setNewCol(e.target.value)}
          placeholder="New collection name"
        />
        <button
          onClick={async () => {
            if (!newCol.trim()) return;
            await api.createCollection(newCol.trim());
            setNewCol("");
            await load();
          }}
        >
          Add collection
        </button>
      </div>
      <div className="list">
        {items.map((item) => (
          <div className="card" key={item.id}>
            {item.screenshotPath && (
              <LibraryThumb path={item.screenshotPath} />
            )}
            <h3>{item.title}</h3>
            <p>{item.clipText || item.ocrText || "(no text)"}</p>
            {item.tags?.length > 0 && (
              <div className="tag-row">
                {item.tags.map((t) => (
                  <span className="tag" key={t}>
                    {t}
                  </span>
                ))}
              </div>
            )}
            {item.noteId && (
              <div className="lineage">
                Linked note: {item.linkedNoteTitle || item.noteId}
              </div>
            )}
            <div className="meta">
              <span>{item.collectionName}</span>
              <span>{item.itemType}</span>
              <span>{item.contentType}</span>
              <span>{new Date(item.createdAt).toLocaleString()}</span>
            </div>
            {item.screenshotPath && (
              <p className="path-line" title={item.screenshotPath}>
                {item.screenshotPath}
              </p>
            )}
            <button
              onClick={async () => {
                await api.deleteLibraryItem(item.id);
                await load();
              }}
            >
              Delete
            </button>
            {(item.clipText || item.ocrText) && (
              <button
                onClick={() =>
                  api.actionCopy(item.clipText || item.ocrText || "")
                }
              >
                Copy text
              </button>
            )}
          </div>
        ))}
        {!items.length && <p className="msg">No library items yet.</p>}
      </div>
    </div>
  );
}
