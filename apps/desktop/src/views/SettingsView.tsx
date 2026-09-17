import { useEffect, useState } from "react";
import { api, AppSettings } from "../api";

export default function SettingsView() {
  const [settings, setSettings] = useState<AppSettings | null>(null);
  const [apiKey, setApiKey] = useState("");
  const [msg, setMsg] = useState("");

  useEffect(() => {
    api.getSettings().then(setSettings).catch(console.error);
  }, []);

  if (!settings) return <p className="msg">Loading settings…</p>;

  return (
    <div>
      <div className="panel-title">
        <h2>Settings</h2>
      </div>
      <p className="panel-lead">
        Hotkey, search, and AI settings for your Scoop workspace.
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
    </div>
  );
}
