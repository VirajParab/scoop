import { useCallback, useEffect, useRef, useState } from "react";
import "./image-editor.css";

export type EditorTool =
  | "pen"
  | "highlighter"
  | "eraser"
  | "line"
  | "rect"
  | "ellipse"
  | "arrow"
  | "text";

type Point = { x: number; y: number };

type Stroke =
  | {
      kind: "free";
      tool: "pen" | "highlighter" | "eraser";
      color: string;
      size: number;
      points: Point[];
    }
  | {
      kind: "shape";
      tool: "line" | "rect" | "ellipse" | "arrow";
      color: string;
      size: number;
      a: Point;
      b: Point;
    }
  | {
      kind: "text";
      color: string;
      size: number;
      at: Point;
      text: string;
    };

const COLORS = [
  "#ff3b30",
  "#ff9500",
  "#ffcc00",
  "#34c759",
  "#5ac8fa",
  "#007aff",
  "#af52de",
  "#ffffff",
  "#000000",
];

const TOOLS: { id: EditorTool; label: string }[] = [
  { id: "pen", label: "Pen" },
  { id: "highlighter", label: "Marker" },
  { id: "eraser", label: "Eraser" },
  { id: "line", label: "Line" },
  { id: "rect", label: "Rect" },
  { id: "ellipse", label: "Oval" },
  { id: "arrow", label: "Arrow" },
  { id: "text", label: "Text" },
];

type Props = {
  imageSrc: string;
  onCancel: () => void;
  onApply: (pngDataUrl: string) => Promise<void>;
};

function drawArrow(
  ctx: CanvasRenderingContext2D,
  a: Point,
  b: Point,
  size: number,
) {
  const angle = Math.atan2(b.y - a.y, b.x - a.x);
  const head = Math.max(10, size * 3);
  ctx.beginPath();
  ctx.moveTo(a.x, a.y);
  ctx.lineTo(b.x, b.y);
  ctx.stroke();
  ctx.beginPath();
  ctx.moveTo(b.x, b.y);
  ctx.lineTo(
    b.x - head * Math.cos(angle - Math.PI / 6),
    b.y - head * Math.sin(angle - Math.PI / 6),
  );
  ctx.lineTo(
    b.x - head * Math.cos(angle + Math.PI / 6),
    b.y - head * Math.sin(angle + Math.PI / 6),
  );
  ctx.closePath();
  ctx.fill();
}

function paintStroke(ctx: CanvasRenderingContext2D, stroke: Stroke) {
  if (stroke.kind === "free") {
    if (stroke.points.length < 2) return;
    ctx.save();
    if (stroke.tool === "eraser") {
      ctx.globalCompositeOperation = "destination-out";
      ctx.strokeStyle = "rgba(0,0,0,1)";
    } else if (stroke.tool === "highlighter") {
      ctx.globalCompositeOperation = "source-over";
      ctx.strokeStyle = stroke.color;
      ctx.globalAlpha = 0.35;
    } else {
      ctx.globalCompositeOperation = "source-over";
      ctx.strokeStyle = stroke.color;
      ctx.globalAlpha = 1;
    }
    ctx.lineWidth = stroke.size;
    ctx.lineCap = "round";
    ctx.lineJoin = "round";
    ctx.beginPath();
    ctx.moveTo(stroke.points[0].x, stroke.points[0].y);
    for (let i = 1; i < stroke.points.length; i++) {
      ctx.lineTo(stroke.points[i].x, stroke.points[i].y);
    }
    ctx.stroke();
    ctx.restore();
    return;
  }

  if (stroke.kind === "text") {
    ctx.save();
    ctx.fillStyle = stroke.color;
    ctx.font = `bold ${Math.max(12, stroke.size * 4)}px "IBM Plex Sans", sans-serif`;
    ctx.fillText(stroke.text, stroke.at.x, stroke.at.y);
    ctx.restore();
    return;
  }

  ctx.save();
  ctx.strokeStyle = stroke.color;
  ctx.fillStyle = stroke.color;
  ctx.lineWidth = stroke.size;
  ctx.lineCap = "round";
  ctx.globalAlpha = 1;
  const { a, b } = stroke;
  if (stroke.tool === "line") {
    ctx.beginPath();
    ctx.moveTo(a.x, a.y);
    ctx.lineTo(b.x, b.y);
    ctx.stroke();
  } else if (stroke.tool === "rect") {
    ctx.strokeRect(a.x, a.y, b.x - a.x, b.y - a.y);
  } else if (stroke.tool === "ellipse") {
    const cx = (a.x + b.x) / 2;
    const cy = (a.y + b.y) / 2;
    const rx = Math.abs(b.x - a.x) / 2;
    const ry = Math.abs(b.y - a.y) / 2;
    ctx.beginPath();
    ctx.ellipse(cx, cy, Math.max(rx, 0.5), Math.max(ry, 0.5), 0, 0, Math.PI * 2);
    ctx.stroke();
  } else if (stroke.tool === "arrow") {
    drawArrow(ctx, a, b, stroke.size);
  }
  ctx.restore();
}

export default function ImageEditor({ imageSrc, onCancel, onApply }: Props) {
  const baseRef = useRef<HTMLCanvasElement>(null);
  const overlayRef = useRef<HTMLCanvasElement>(null);
  const wrapRef = useRef<HTMLDivElement>(null);
  const [tool, setTool] = useState<EditorTool>("pen");
  const [color, setColor] = useState("#ff3b30");
  const [size, setSize] = useState(4);
  const [strokes, setStrokes] = useState<Stroke[]>([]);
  const [redo, setRedo] = useState<Stroke[]>([]);
  const [draft, setDraft] = useState<Stroke | null>(null);
  const [busy, setBusy] = useState(false);
  const [natural, setNatural] = useState({ w: 0, h: 0 });
  const drawing = useRef(false);

  const redrawOverlay = useCallback(
    (extra?: Stroke | null) => {
      const canvas = overlayRef.current;
      if (!canvas || !natural.w) return;
      const ctx = canvas.getContext("2d");
      if (!ctx) return;
      ctx.clearRect(0, 0, canvas.width, canvas.height);
      for (const s of strokes) paintStroke(ctx, s);
      if (extra) paintStroke(ctx, extra);
      if (draft && draft !== extra) paintStroke(ctx, draft);
    },
    [strokes, draft, natural.w],
  );

  useEffect(() => {
    const img = new Image();
    img.onload = () => {
      const w = img.naturalWidth || img.width;
      const h = img.naturalHeight || img.height;
      setNatural({ w, h });
      for (const ref of [baseRef, overlayRef]) {
        const c = ref.current;
        if (!c) continue;
        c.width = w;
        c.height = h;
      }
      const ctx = baseRef.current?.getContext("2d");
      if (ctx) {
        ctx.clearRect(0, 0, w, h);
        ctx.drawImage(img, 0, 0);
      }
      redrawOverlay(null);
    };
    img.src = imageSrc;
  }, [imageSrc]);

  useEffect(() => {
    redrawOverlay(null);
  }, [strokes, draft, redrawOverlay]);

  const canvasPoint = (e: React.PointerEvent<HTMLCanvasElement>): Point => {
    const canvas = overlayRef.current!;
    const rect = canvas.getBoundingClientRect();
    const x = ((e.clientX - rect.left) / rect.width) * canvas.width;
    const y = ((e.clientY - rect.top) / rect.height) * canvas.height;
    return { x, y };
  };

  const onPointerDown = (e: React.PointerEvent<HTMLCanvasElement>) => {
    if (!overlayRef.current) return;
    const p = canvasPoint(e);
    if (tool === "text") {
      const text = window.prompt("Annotation text:", "");
      if (text && text.trim()) {
        const stroke: Stroke = {
          kind: "text",
          color,
          size,
          at: p,
          text: text.trim(),
        };
        setStrokes((s) => [...s, stroke]);
        setRedo([]);
      }
      return;
    }
    drawing.current = true;
    (e.target as HTMLElement).setPointerCapture(e.pointerId);
    if (tool === "pen" || tool === "highlighter" || tool === "eraser") {
      setDraft({
        kind: "free",
        tool,
        color,
        size: tool === "highlighter" ? size * 3 : size,
        points: [p],
      });
    } else {
      setDraft({
        kind: "shape",
        tool,
        color,
        size,
        a: p,
        b: p,
      });
    }
  };

  const onPointerMove = (e: React.PointerEvent<HTMLCanvasElement>) => {
    if (!drawing.current || !draft) return;
    const p = canvasPoint(e);
    if (draft.kind === "free") {
      const next = { ...draft, points: [...draft.points, p] };
      setDraft(next);
      redrawOverlay(next);
    } else if (draft.kind === "shape") {
      const next = { ...draft, b: p };
      setDraft(next);
      redrawOverlay(next);
    }
  };

  const onPointerUp = () => {
    if (!drawing.current) return;
    drawing.current = false;
    if (draft) {
      setStrokes((s) => [...s, draft]);
      setRedo([]);
      setDraft(null);
    }
  };

  const undo = () => {
    setStrokes((s) => {
      if (!s.length) return s;
      const last = s[s.length - 1];
      setRedo((r) => [...r, last]);
      return s.slice(0, -1);
    });
  };

  const redoOne = () => {
    setRedo((r) => {
      if (!r.length) return r;
      const last = r[r.length - 1];
      setStrokes((s) => [...s, last]);
      return r.slice(0, -1);
    });
  };

  const clearAll = () => {
    setStrokes([]);
    setRedo([]);
    setDraft(null);
  };

  const apply = async () => {
    const base = baseRef.current;
    const overlay = overlayRef.current;
    if (!base || !overlay) return;
    setBusy(true);
    try {
      const out = document.createElement("canvas");
      out.width = base.width;
      out.height = base.height;
      const ctx = out.getContext("2d");
      if (!ctx) throw new Error("Canvas unsupported");
      ctx.drawImage(base, 0, 0);
      ctx.drawImage(overlay, 0, 0);
      const dataUrl = out.toDataURL("image/png");
      await onApply(dataUrl);
    } finally {
      setBusy(false);
    }
  };

  return (
    <div className="img-editor" ref={wrapRef}>
      <div className="img-editor-toolbar">
        <div className="img-editor-tools">
          {TOOLS.map((t) => (
            <button
              key={t.id}
              type="button"
              className={tool === t.id ? "active" : ""}
              onClick={() => setTool(t.id)}
            >
              {t.label}
            </button>
          ))}
        </div>
        <div className="img-editor-colors">
          {COLORS.map((c) => (
            <button
              key={c}
              type="button"
              className={color === c ? "swatch active" : "swatch"}
              style={{ background: c }}
              onClick={() => setColor(c)}
              title={c}
            />
          ))}
        </div>
        <label className="img-editor-size">
          Size
          <input
            type="range"
            min={1}
            max={24}
            value={size}
            onChange={(e) => setSize(Number(e.target.value))}
          />
          <span>{size}</span>
        </label>
        <div className="img-editor-actions">
          <button type="button" onClick={undo} disabled={!strokes.length}>
            Undo
          </button>
          <button type="button" onClick={redoOne} disabled={!redo.length}>
            Redo
          </button>
          <button type="button" onClick={clearAll} disabled={!strokes.length}>
            Clear
          </button>
          <button type="button" className="ghost" onClick={onCancel} disabled={busy}>
            Cancel
          </button>
          <button type="button" className="primary" onClick={apply} disabled={busy}>
            {busy ? "Applying…" : "Apply edits"}
          </button>
        </div>
      </div>
      <div className="img-editor-stage">
        <div
          className="img-editor-canvas-wrap"
          style={{
            aspectRatio: natural.w && natural.h ? `${natural.w} / ${natural.h}` : "16 / 9",
          }}
        >
          <canvas ref={baseRef} className="img-editor-base" />
          <canvas
            ref={overlayRef}
            className="img-editor-overlay"
            onPointerDown={onPointerDown}
            onPointerMove={onPointerMove}
            onPointerUp={onPointerUp}
            onPointerCancel={onPointerUp}
          />
        </div>
      </div>
      <p className="img-editor-hint">
        Draw on the screenshot, then Apply edits. Save image afterward to keep it in
        ~/Pictures/Screenshots.
      </p>
    </div>
  );
}
