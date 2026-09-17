import { useEffect, useState } from "react";
import { api, HistoryItem } from "../api";

export default function HistoryView() {
  const [items, setItems] = useState<HistoryItem[]>([]);

  const load = async () => setItems(await api.listHistory());

  useEffect(() => {
    load().catch(console.error);
  }, []);

  return (
    <div>
      <div className="panel-title">
        <h2>History</h2>
        <button
          onClick={async () => {
            await api.clearHistory();
            await load();
          }}
        >
          Clear history
        </button>
      </div>
      <p className="panel-lead">
        Recent actions only. Clearing history does not delete Library or Notes.
      </p>
      <div className="list">
        {items.map((h) => (
          <div className="card" key={h.id}>
            <h3>{h.action}</h3>
            <p>{h.inputSummary}</p>
            {h.outputSummary && <p>{h.outputSummary}</p>}
            <div className="meta">
              <span>{h.contentType}</span>
              <span>{new Date(h.createdAt).toLocaleString()}</span>
            </div>
            <button
              onClick={async () => {
                await api.deleteHistory(h.id);
                await load();
              }}
            >
              Delete
            </button>
          </div>
        ))}
        {!items.length && <p className="msg">No history yet.</p>}
      </div>
    </div>
  );
}
