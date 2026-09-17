import { useCallback, useEffect, useRef, useState } from "react";
import { listen } from "@tauri-apps/api/event";
import { convertFileSrc } from "@tauri-apps/api/core";
import { api } from "../api";
import "./overlay.css";

type StartedPayload = { backdropPath: string };

function errMessage(err: unknown): string {
  if (typeof err === "string") return err;
  if (err && typeof err === "object" && "message" in err) {
    return String((err as { message: unknown }).message);
  }
  try {
    return JSON.stringify(err);
  } catch {
    return String(err);
  }
}

export default function Overlay() {
  const startClient = useRef<{ x: number; y: number } | null>(null);
  const confirming = useRef(false);
  const [dragging, setDragging] = useState(false);
  const [busy, setBusy] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [backdropUrl, setBackdropUrl] = useState<string | null>(null);
  const [rect, setRect] = useState<{
    x: number;
    y: number;
    w: number;
    h: number;
  } | null>(null);

  const cancel = useCallback(() => {
    confirming.current = false;
    setBusy(false);
    setError(null);
    api.cancelSelection().catch(console.error);
  }, []);

  const setBackdropFromPath = useCallback((path: string) => {
    // Asset protocol is much faster than base64 IPC for large screenshots.
    setBackdropUrl(convertFileSrc(path));
  }, []);

  useEffect(() => {
    const onKey = (e: KeyboardEvent) => {
      if (e.key === "Escape") cancel();
    };
    window.addEventListener("keydown", onKey);

    api.getOverlayBackdrop()
      .then((p) => {
        if (p) setBackdropFromPath(p);
      })
      .catch(console.error);

    const unlisten = listen<StartedPayload>("selection-started", (ev) => {
      setRect(null);
      setDragging(false);
      setBusy(false);
      setError(null);
      confirming.current = false;
      startClient.current = null;
      setBackdropFromPath(ev.payload.backdropPath);
    });

    return () => {
      window.removeEventListener("keydown", onKey);
      unlisten.then((u) => u());
    };
  }, [cancel, setBackdropFromPath]);

  const onPointerDown = (e: React.PointerEvent) => {
    if (busy || confirming.current) return;
    setError(null);
    startClient.current = { x: e.clientX, y: e.clientY };
    setDragging(true);
    setRect({ x: e.clientX, y: e.clientY, w: 0, h: 0 });
    (e.target as HTMLElement).setPointerCapture(e.pointerId);
  };

  const onPointerMove = (e: React.PointerEvent) => {
    if (!dragging || !startClient.current) return;
    const x1 = Math.min(startClient.current.x, e.clientX);
    const y1 = Math.min(startClient.current.y, e.clientY);
    const w = Math.abs(e.clientX - startClient.current.x);
    const h = Math.abs(e.clientY - startClient.current.y);
    setRect({ x: x1, y: y1, w, h });
  };

  const onPointerUp = async (e: React.PointerEvent) => {
    if (!startClient.current || confirming.current) return;
    setDragging(false);
    const x = Math.min(startClient.current.x, e.clientX);
    const y = Math.min(startClient.current.y, e.clientY);
    const width = Math.abs(e.clientX - startClient.current.x);
    const height = Math.abs(e.clientY - startClient.current.y);
    startClient.current = null;
    if (width < 4 || height < 4) {
      setRect(null);
      return;
    }

    confirming.current = true;
    setBusy(true);
    setError(null);
    try {
      await api.confirmSelection({
        x: Math.round(x),
        y: Math.round(y),
        width: Math.round(width),
        height: Math.round(height),
      });
      // Overlay is hidden by the backend on success.
    } catch (err) {
      console.error(err);
      confirming.current = false;
      setBusy(false);
      setError(errMessage(err));
      // Keep overlay open so the user can retry or press Esc.
    }
  };

  return (
    <div
      className="overlay-root"
      onPointerDown={onPointerDown}
      onPointerMove={onPointerMove}
      onPointerUp={onPointerUp}
    >
      {backdropUrl ? (
        <img
          className="overlay-backdrop"
          src={backdropUrl}
          alt=""
          draggable={false}
        />
      ) : (
        <div className="overlay-fallback" />
      )}
      <div className={rect ? "overlay-dim selecting" : "overlay-dim"} />
      <div className="overlay-hint">
        {busy
          ? "Working…"
          : error
            ? "Selection failed — drag again or press Esc"
            : "Drag to select · Esc to cancel"}
      </div>
      {error && <div className="overlay-error">{error}</div>}
      {rect && (
        <div
          className="selection-rect"
          style={{
            left: rect.x,
            top: rect.y,
            width: rect.w,
            height: rect.h,
          }}
        >
          <span className="selection-size">
            {Math.round(rect.w)}×{Math.round(rect.h)}
          </span>
        </div>
      )}
    </div>
  );
}
