import { useEffect, useMemo, useState } from "react";
import { listen } from "@tauri-apps/api/event";
import { getCurrentWindow, LogicalSize } from "@tauri-apps/api/window";
import { api, SelectionSession } from "../api";
import ImageEditor from "./ImageEditor";
import "./toolbar.css";

const LABELS: Record<string, string> = {
  calculate: "Calculate",
  ask_ai: "Ask AI",
  search: "Search",
  save_library: "Save image",
  save_note: "Note",
  copy: "Copy",
};

function parseTags(raw: string): string[] {
  const seen = new Set<string>();
  const out: string[] = [];
  for (const part of raw.split(/[,#]+/)) {
    const t = part.trim().toLowerCase();
    if (!t || seen.has(t)) continue;
    seen.add(t);
    out.push(t);
  }
  return out;
}

export default function Toolbar() {
  const [session, setSession] = useState<SelectionSession | null>(null);
  const [ocr, setOcr] = useState("");
  const [preview, setPreview] = useState<string | null>(null);
  const [busy, setBusy] = useState(false);
  const [result, setResult] = useState<string>("");
  const [askOpen, setAskOpen] = useState(false);
  const [saveOpen, setSaveOpen] = useState(false);
  const [editing, setEditing] = useState(false);
  const [saveTitle, setSaveTitle] = useState("");
  const [saveTags, setSaveTags] = useState("");
  const [question, setQuestion] = useState("Explain this.");
  const [error, setError] = useState("");

  const applySession = (s: SelectionSession) => {
    setSession(s);
    setOcr(s.ocrText ?? "");
    if (s.previewDataUrl) {
      setPreview(s.previewDataUrl);
    } else if (s.capturePath) {
      api
        .readCaptureDataUrl(s.capturePath)
        .then(setPreview)
        .catch((e) => {
          console.error(e);
          setPreview(null);
        });
    } else {
      setPreview(null);
    }
  };

  const refresh = async () => {
    applySession(await api.getSession());
  };

  useEffect(() => {
    refresh().catch(console.error);
    const unsubs = [
      listen<SelectionSession>("session-updated", (ev) => {
        applySession(ev.payload);
        setAskOpen(false);
      }),
      listen("selection-started", () => {
        setResult("");
        setError("");
        setAskOpen(false);
        setSaveOpen(false);
        setEditing(false);
        setSaveTitle("");
        setSaveTags("");
        setPreview(null);
      }),
    ];
    const onKey = (e: KeyboardEvent) => {
      if (e.key === "Escape") {
        if (editing) {
          setEditing(false);
          return;
        }
        api.dismissToolbar().catch(console.error);
      }
    };
    window.addEventListener("keydown", onKey);
    getCurrentWindow()
      .setSize(new LogicalSize(1280, 820))
      .catch(console.error);
    return () => {
      window.removeEventListener("keydown", onKey);
      unsubs.forEach((p) => p.then((u) => u()));
    };
  }, [editing]);

  useEffect(() => {
    const tall = saveOpen || askOpen || editing;
    getCurrentWindow()
      .setSize(new LogicalSize(editing ? 1380 : 1280, tall ? 920 : 820))
      .catch(console.error);
  }, [saveOpen, askOpen, editing]);

  const actions = useMemo(() => session?.actions ?? [], [session]);
  const hasImage = Boolean(preview || session?.capturePath);

  const onOcrBlur = async () => {
    try {
      applySession(await api.updateOcrText(ocr));
    } catch (e) {
      console.error(e);
    }
  };

  const openSave = () => {
    setAskOpen(false);
    setEditing(false);
    setSaveOpen(true);
    setError("");
    setResult("");
    if (!saveTitle.trim()) {
      const first = ocr.split("\n").find((l) => l.trim())?.trim() ?? "";
      setSaveTitle(first.slice(0, 60));
    }
  };

  const openEditor = () => {
    if (!preview) {
      setError("No screenshot to edit.");
      return;
    }
    setSaveOpen(false);
    setAskOpen(false);
    setEditing(true);
    setError("");
    setResult("");
  };

  const confirmSave = async () => {
    setBusy(true);
    setError("");
    try {
      if (!session?.capturePath && !preview) {
        throw new Error("No screenshot in this selection. Select again.");
      }
      const tags = parseTags(saveTags);
      const item = await api.actionSaveLibrary({
        title: saveTitle.trim() || undefined,
        clipText: ocr,
        tags,
        capturePath: session?.capturePath ?? undefined,
        includeScreenshot: true,
      });
      if (!item.screenshotPath) {
        throw new Error("Saved, but screenshot was not stored. Try selecting again.");
      }
      const tagNote = tags.length ? `\nTags: ${tags.join(", ")}` : "";
      setResult(
        `Saved “${item.title}” to Library.\nStored at:\n${item.screenshotPath}${tagNote}${
          item.noteId
            ? `\nLinked note: ${item.linkedNoteTitle || item.noteId}`
            : "\nTip: Save Note next to link this image for search."
        }`,
      );
      setSaveOpen(false);
      setSaveTags("");
    } catch (e) {
      setError(String(e));
    } finally {
      setBusy(false);
    }
  };

  const run = async (action: string) => {
    setBusy(true);
    setError("");
    try {
      switch (action) {
        case "copy":
          await api.actionCopy(ocr);
          setResult("Copied OCR text to clipboard");
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
        case "save_library":
          openSave();
          break;
        case "save_note": {
          const note = await api.actionSaveNote(
            {
              content: ocr,
              tags: parseTags(saveTags),
              libraryItemId: session?.libraryItemId ?? undefined,
            },
            false,
          );
          const link = note.libraryItemId
            ? `\nLinked image: ${note.linkedLibraryTitle || note.libraryItemId}`
            : "";
          const where = note.screenshotPath
            ? `\nStored at:\n${note.screenshotPath}`
            : "";
          setResult(`Saved note: ${note.title}${link}${where}`);
          applySession(await api.getSession());
          break;
        }
        case "ask_ai":
          setSaveOpen(false);
          setEditing(false);
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

  if (editing && preview) {
    return (
      <div className="toolbar-root editor-mode">
        <div className="toolbar-meta">
          <span className="pill">EDIT IMAGE</span>
          <button className="ghost" onClick={() => setEditing(false)}>
            Back
          </button>
        </div>
        <ImageEditor
          imageSrc={preview}
          onCancel={() => setEditing(false)}
          onApply={async (png) => {
            const s = await api.applyEditedCapture(png);
            applySession(s);
            setEditing(false);
            setResult(
              s.clipboardImageCopied
                ? "Edits applied. Screenshot updated and copied — paste with Ctrl+V, then Save image when ready."
                : "Edits applied. Screenshot updated — Save image when ready.",
            );
          }}
        />
        {error && <div className="err">{error}</div>}
      </div>
    );
  }

  return (
    <div className="toolbar-root">
      <div className="toolbar-meta">
        <span className="pill">{session?.contentType || "UNKNOWN"}</span>
        <button className="ghost" onClick={() => api.dismissToolbar()}>
          Close
        </button>
      </div>

      {session?.ocrError && <div className="err">{session.ocrError}</div>}
      {session?.clipboardImageCopied && (
        <div className="clipboard-hint">
          Screenshot copied to clipboard — paste with Ctrl+V
        </div>
      )}

      <div className="content-row">
        <div className="preview-pane">
          {preview ? (
            <img className="preview-img" src={preview} alt="Selection screenshot" />
          ) : (
            <div className="preview-empty">No screenshot</div>
          )}
        </div>
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
          rows={12}
        />
      </div>

      <div className="action-row">
        <button disabled={busy || !hasImage} onClick={openEditor}>
          Edit image
        </button>
        {actions.map((a) => (
          <button key={a} disabled={busy} onClick={() => run(a)}>
            {LABELS[a] ?? a}
          </button>
        ))}
      </div>

      {saveOpen && (
        <div className="save-panel">
          <div className="save-title">Save screenshot to Library</div>
          <div className="save-layout">
            <div className="save-preview">
              {preview ? (
                <img src={preview} alt="" />
              ) : (
                <div className="preview-empty">Image missing</div>
              )}
            </div>
            <div className="save-fields">
              <label className="save-field">
                <span>Title</span>
                <input
                  value={saveTitle}
                  onChange={(e) => setSaveTitle(e.target.value)}
                  placeholder="Optional title"
                />
              </label>
              <label className="save-field">
                <span>Tags</span>
                <input
                  autoFocus
                  value={saveTags}
                  onChange={(e) => setSaveTags(e.target.value)}
                  placeholder="work, bug, postgres"
                  onKeyDown={(e) => {
                    if (e.key === "Enter") {
                      e.preventDefault();
                      confirmSave().catch(console.error);
                    }
                  }}
                />
              </label>
              <div className="save-hint">
                {hasImage
                  ? "Edited screenshot + text will be saved to ~/Pictures/Screenshots"
                  : "Screenshot missing — select the region again."}
              </div>
            </div>
          </div>
          <div className="save-actions">
            <button className="ghost" disabled={busy} onClick={() => setSaveOpen(false)}>
              Cancel
            </button>
            <button disabled={busy || !hasImage} onClick={() => confirmSave()}>
              {busy ? "Saving…" : "Save image"}
            </button>
          </div>
        </div>
      )}

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
