# Privacy & security

**Status:** Accepted  
**Principle:** Selected screen content is ephemeral unless the user explicitly keeps it.

## Default behavior

1. Captures land in **cache**, not permanent storage.
2. Captures are **deleted** when selection is cancelled, the result UI is dismissed, or TTL expires.
3. Permanent storage happens only when the user:
   - Saves to the **Library** (screenshot and/or text), or
   - Saves a **Note** (with optional screenshot), or
   - Enables history retention that includes media (off by default or short TTL).
4. API keys never written to plaintext logs or shared telemetry.

## Settings users control

```text
AI Provider
API Key
Cloud processing (on/off)
Local processing (when available)
History retention
Auto-delete screenshots
Also save copies to Library (off by default)
```

See [settings.md](../features/settings.md).

## Data sent to cloud AI

When cloud processing is enabled and the user runs an AI action, providers may receive:

- Cropped screenshot (vision actions)
- OCR text
- Detected content type
- User question / prompts
- Optional app context (v2)

**Never send** full-desktop screenshots when a region was selected. **Never send** keyring secrets.

Show a clear indicator when an action will call the network.

## Local-only path

Always available without network:

- Hotkey + marquee + capture
- OCR (local engine)
- Math (local engine)
- Exact Search (opens browser; query stays local until browser navigates)
- Copy
- **Library** browse / arrange / keyword search
- Notes CRUD + keyword search
- History metadata (depending on settings)

## Sensitive applications (v2)

Users can exclude apps:

```text
Password Manager
Banking Apps
Private Browser Windows
```

When excluded app is focused: Scoop hotkey disabled (or no-ops with brief notice).

MVP: document as planned; implement exclusion list UI as stretch only if desktop metadata is reliable.

## Security practices (engineering)

| Area | Practice |
|------|----------|
| Secrets | Freedesktop Secret Service / keyring |
| IPC | No world-writable sockets with capture access |
| Temp files | Restrictive permissions (`0600`); unique paths |
| Dependencies | Pin versions; review native capture crates |
| Updates | Signed releases when packaging matures |
| Logging | Redact OCR/screenshot paths content in default log level |

## Threat notes (lightweight)

- **Malware on same machine** can already screenshot; Scoop should not widen that (permissions, IPC).
- **Prompt injection** via on-screen text: treat OCR as untrusted input; don’t execute tool-like instructions from selections.
- **Shoulder surfing**: overlay results can be dismissed quickly; no always-on content bar.

## Compliance posture (MVP)

No claim of HIPAA/SOC2. Document data flows for users. Opt-in analytics only, with no screenshot payloads in telemetry.
