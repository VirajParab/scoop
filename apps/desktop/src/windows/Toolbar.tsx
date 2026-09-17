import { useEffect, useMemo, useState } from "react";
import { listen } from "@tauri-apps/api/event";
import { api, SelectionSession } from "../api";
import "./toolbar.css";

const LABELS: Record<string, string> = {
  calculate: "Calculate",
  ask_ai: "Ask AI",
  search: "Search",
  save_library: "Library",
  save_note: "Note",
  copy: "Copy",
};

export default function Toolbar() {
  const [session, setSession] = useState<SelectionSession | null>(null);
  const [ocr, setOcr] = useState("");
  const [busy, setBusy] = useState(false);
  const [result, setResult] = useState<string>("");
  const [askOpen, setAskOpen] = useState(false);
  const [question, setQuestion] = useState("Explain this.");
  const [error, setError] = useState("");

  const refresh = async () => {
    const s = await api.getSession();
    setSession(s);
    setOcr(s.ocrText ?? "");
  };

  useEffect(() => {
    refresh().catch(console.error);
    const unsubs = [
      listen("session-updated", () => refresh()),
      listen("selection-started", () => {
        setResult("");
        setError("");
        setAskOpen(false);
      }),
    ];
    const onKey = (e: KeyboardEvent) => {
      if (e.key === "Escape") api.dismissToolbar().catch(console.error);
    };
    window.addEventListener("keydown", onKey);
    return () => {
      window.removeEventListener("keydown", onKey);
      unsubs.forEach((p) => p.then((u) => u()));
    };
  }, []);

  const actions = useMemo(() => session?.actions ?? [], [session]);

  const onOcrBlur = async () => {
    try {
      const s = await api.updateOcrText(ocr);
      setSession(s);
    } catch (e) {
      console.error(e);
    }
  };

  const run = async (action: string) => {
    setBusy(true);
    setError("");
    try {
      switch (action) {
        case "copy":
          await api.actionCopy(ocr);
          setResult("Copied to clipboard");
          break;
        case "calculate": {
          const r = await api.actionCalculate(ocr);
          setResult(`${r.expression} = ${r.answer}`);
          break;
        }
        case "search": {
          const q = await api.actionSearch(ocr, false);
          setResult(`Searched: ${q}`);
          break;
        }
        case "save_library": {
          const item = await api.actionSaveLibrary({ clipText: ocr });
          setResult(`Saved to Library: ${item.title}`);
          break;
        }
        case "save_note": {
          const note = await api.actionSaveNote({ content: ocr }, false);
          setResult(`Saved note: ${note.title}`);
          break;
        }
        case "ask_ai":
          setAskOpen(true);
          break;
        default:
          setError(`Unknown action: ${action}`);
      }
    } catch (e) {
      setError(String(e));
    } finally {
      setBusy(false);
    }
  };

  const sendAsk = async () => {
    setBusy(true);
    setError("");
    try {
      const r = await api.actionAskAi(question);
      setResult(r.answer);
    } catch (e) {
      setError(String(e));
    } finally {
      setBusy(false);
    }
  };

  return (
    <div className="toolbar-root">
      <div className="toolbar-meta">
        <span className="pill">{session?.contentType || "UNKNOWN"}</span>
        <button className="ghost" onClick={() => api.dismissToolbar()}>
          Close
        </button>
      </div>
      {session?.ocrError && <div className="err">{session.ocrError}</div>}
      <textarea
        className="ocr-box"
        value={ocr}
        onChange={(e) => setOcr(e.target.value)}
        onBlur={onOcrBlur}
        placeholder={
          session?.ocrError
            ? "Type or paste text manually, then use actions below"
            : "OCR text (editable)"
        }
        rows={3}
      />
      <div className="action-row">
        {actions.map((a) => (
          <button key={a} disabled={busy} onClick={() => run(a)}>
            {LABELS[a] ?? a}
          </button>
        ))}
      </div>
      {askOpen && (
        <div className="ask-row">
          <input
            value={question}
            onChange={(e) => setQuestion(e.target.value)}
            placeholder="Ask about the selection…"
          />
          <button disabled={busy} onClick={sendAsk}>
            Send
          </button>
        </div>
      )}
      {error && <div className="err">{error}</div>}
      {result && <div className="result">{result}</div>}
    </div>
  );
}
