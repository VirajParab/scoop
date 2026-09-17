import { useCallback, useEffect, useState } from "react";
import { api, AppSettings, OcrStatus } from "../api";

const LANG_LABELS: Record<string, string> = {
  eng: "English",
  spa: "Spanish",
  fra: "French",
  deu: "German",
  ita: "Italian",
  por: "Portuguese",
  rus: "Russian",
  jpn: "Japanese",
  chi_sim: "Chinese (Simplified)",
};

export default function SettingsView() {
  const [settings, setSettings] = useState<AppSettings | null>(null);
  const [apiKey, setApiKey] = useState("");
  const [msg, setMsg] = useState("");
  const [ocr, setOcr] = useState<OcrStatus | null>(null);
  const [ocrBusy, setOcrBusy] = useState(false);
  const [ocrMsg, setOcrMsg] = useState("");
  const [downloadLang, setDownloadLang] = useState("eng");

  const refreshOcr = useCallback(() => {
    api.ocrStatus()
      .then((status) => {
        setOcr(status);
        setDownloadLang((current) => {
          if (status.missingLanguages.includes(current)) return current;
          if (status.missingLanguages.length) return status.missingLanguages[0];
          return current || "eng";
        });
      })
      .catch(console.error);
  }, []);

  useEffect(() => {
    api.getSettings().then(setSettings).catch(console.error);
    refreshOcr();
  }, [refreshOcr]);

  if (!settings) return <p className="msg">Loading settings…</p>;

  const langOptions =
    ocr?.missingLanguages?.length
      ? ocr.missingLanguages
      : Object.keys(LANG_LABELS);

  return (
    <div>
      <div className="panel-title">
        <h2>Settings</h2>
      </div>
      <p className="panel-lead">
        Hotkey, search, AI, and OCR settings for your Scoop workspace.
      </p>
      <div className="form-grid">
        <label>
          Global hotkey
          <input
            value={settings.hotkey}
            onChange={(e) =>
              setSettings({ ...settings, hotkey: e.target.value })
            }
          />
        </label>
        <label>
          Search provider
          <select
            value={settings.searchProvider}
            onChange={(e) =>
              setSettings({ ...settings, searchProvider: e.target.value })
            }
          >
            <option value="duckduckgo">DuckDuckGo</option>
            <option value="google">Google</option>
            <option value="bing">Bing</option>
          </select>
        </label>
        <label>
          AI provider
          <select
            value={settings.aiProvider}
            onChange={(e) =>
              setSettings({ ...settings, aiProvider: e.target.value })
            }
          >
            <option value="openai">OpenAI</option>
          </select>
        </label>
        <label>
          AI model
          <input
            value={settings.aiModel}
            onChange={(e) =>
              setSettings({ ...settings, aiModel: e.target.value })
            }
          />
        </label>
        <label>
          API key {settings.hasApiKey ? "(saved)" : "(not set)"}
          <input
            type="password"
            value={apiKey}
            placeholder={settings.hasApiKey ? "••••••••" : "sk-…"}
            onChange={(e) => setApiKey(e.target.value)}
          />
        </label>
        <label>
          History retention (days)
          <input
            value={settings.historyRetentionDays}
            onChange={(e) =>
              setSettings({
                ...settings,
                historyRetentionDays: e.target.value,
              })
            }
          />
        </label>
        <label>
          <span>
            <input
              type="checkbox"
              checked={settings.cloudProcessing}
              onChange={(e) =>
                setSettings({
                  ...settings,
                  cloudProcessing: e.target.checked,
                })
              }
            />{" "}
            Cloud AI processing
          </span>
        </label>
        <label>
          <span>
            <input
              type="checkbox"
              checked={settings.saveCopiesToLibrary}
              onChange={(e) =>
                setSettings({
                  ...settings,
                  saveCopiesToLibrary: e.target.checked,
                })
              }
            />{" "}
            Also save copied text to Library
          </span>
        </label>
        <button
          onClick={async () => {
            try {
              await api.saveSettings(
                settings,
                apiKey.trim() ? apiKey.trim() : undefined,
              );
              await api.rebindHotkey(settings.hotkey);
              setMsg("Saved.");
              setApiKey("");
              setSettings(await api.getSettings());
            } catch (e) {
              setMsg(String(e));
            }
          }}
        >
          Save settings
        </button>
        {msg && <p className="msg">{msg}</p>}
      </div>

      <section className="settings-ocr">
        <h3>OCR</h3>
        <p className="panel-lead">
          Scoop uses system Tesseract. Download language packs here anytime —
          no reinstall needed.
        </p>
        {ocr ? (
          <div className="ocr-status">
            <div className="ocr-status-row">
              <span className="ocr-label">Engine</span>
              <span
                className={
                  ocr.engineInstalled ? "ocr-pill ok" : "ocr-pill warn"
                }
              >
                {ocr.engineInstalled
                  ? ocr.engineVersion || "Installed"
                  : "Not installed"}
              </span>
            </div>
            <div className="ocr-status-row">
              <span className="ocr-label">English data</span>
              <span
                className={ocr.languageReady ? "ocr-pill ok" : "ocr-pill warn"}
              >
                {ocr.languageReady ? "Ready" : "Missing"}
              </span>
            </div>
            {ocr.installedLanguages.length > 0 && (
              <div className="ocr-status-row">
                <span className="ocr-label">Installed</span>
                <span className="ocr-langs">
                  {ocr.installedLanguages.join(", ")}
                </span>
              </div>
            )}
            <p className="ocr-hint">{ocr.hint}</p>
            {!ocr.engineInstalled && (
              <p className="ocr-hint code-hint">
                <code>sudo apt install tesseract-ocr</code>
                {" · "}
                <code>make install-ocr</code>
              </p>
            )}
            <p className="ocr-path">Data folder: {ocr.tessdataDir}</p>
          </div>
        ) : (
          <p className="msg">Checking OCR…</p>
        )}

        <div className="form-grid ocr-actions">
          <label>
            Language pack
            <select
              value={downloadLang}
              onChange={(e) => setDownloadLang(e.target.value)}
              disabled={ocrBusy}
            >
              {langOptions.map((code) => (
                <option key={code} value={code}>
                  {LANG_LABELS[code] || code} ({code})
                </option>
              ))}
            </select>
          </label>
          <div className="ocr-btn-row">
            <button
              type="button"
              disabled={ocrBusy || !downloadLang}
              onClick={async () => {
                setOcrBusy(true);
                setOcrMsg("");
                try {
                  const next = await api.downloadOcrLanguage(downloadLang);
                  setOcr(next);
                  setOcrMsg(
                    `Downloaded ${LANG_LABELS[downloadLang] || downloadLang}.`,
                  );
                } catch (e) {
                  setOcrMsg(String(e));
                } finally {
                  setOcrBusy(false);
                }
              }}
            >
              {ocrBusy ? "Downloading…" : "Download language pack"}
            </button>
            <button
              type="button"
              className="btn-secondary"
              disabled={ocrBusy}
              onClick={() => {
                setOcrMsg("");
                refreshOcr();
              }}
            >
              Refresh status
            </button>
          </div>
          {ocrMsg && <p className="msg">{ocrMsg}</p>}
        </div>
      </section>
    </div>
  );
}
