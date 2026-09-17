import { invoke } from "@tauri-apps/api/core";

export type SelectionSession = {
  capturePath?: string | null;
  previewDataUrl?: string | null;
  ocrText: string;
  ocrError?: string | null;
  contentType: string;
  actions: string[];
  region?: { x: number; y: number; width: number; height: number } | null;
  libraryItemId?: string | null;
  noteId?: string | null;
  clipboardImageCopied?: boolean;
};

export type AppSettings = {
  hotkey: string;
  searchProvider: string;
  aiProvider: string;
  aiModel: string;
  historyRetentionDays: string;
  saveCopiesToLibrary: boolean;
  cloudProcessing: boolean;
  hasApiKey: boolean;
};

export type LibraryItem = {
  id: string;
  collectionId: string;
  collectionName?: string;
  title: string;
  itemType: string;
  clipText?: string;
  ocrText?: string;
  tags: string[];
  screenshotPath?: string;
  contentType?: string;
  noteId?: string | null;
  linkedNoteTitle?: string | null;
  createdAt: string;
};

export type Note = {
  id: string;
  title: string;
  content: string;
  summary?: string;
  ocrText?: string;
  tags: string[];
  screenshotPath?: string;
  contentType?: string;
  isSmart: boolean;
  libraryItemId?: string | null;
  linkedLibraryTitle?: string | null;
  createdAt: string;
};

export type SearchHit = {
  kind: "library" | "note" | string;
  id: string;
  title: string;
  snippet: string;
  tags: string[];
  screenshotPath?: string | null;
  linkedKind?: string | null;
  linkedId?: string | null;
  linkedTitle?: string | null;
  createdAt: string;
};

export type HistoryItem = {
  id: string;
  action: string;
  inputSummary?: string;
  outputSummary?: string;
  contentType?: string;
  createdAt: string;
};

export type Collection = {
  id: string;
  name: string;
  isSystem: boolean;
};

export const api = {
  startSelection: () => invoke("start_selection"),
  cancelSelection: () => invoke("cancel_selection"),
  confirmSelection: (region: {
    x: number;
    y: number;
    width: number;
    height: number;
  }) => invoke<SelectionSession>("confirm_selection", { region }),
  getSession: () => invoke<SelectionSession>("get_session"),
  updateOcrText: (text: string) =>
    invoke<SelectionSession>("update_ocr_text", { text }),
  applyEditedCapture: (pngBase64: string) =>
    invoke<SelectionSession>("apply_edited_capture", { pngBase64 }),
  dismissToolbar: () => invoke("dismiss_toolbar"),
  actionCopy: (text?: string) => invoke("action_copy", { text }),
  actionCalculate: (text?: string) =>
    invoke<{ expression: string; normalized: string; answer: string }>(
      "action_calculate",
      { text },
    ),
  actionSearch: (text?: string, useAi = false) =>
    invoke<string>("action_search", { text, useAi }),
  actionSaveLibrary: (input: Record<string, unknown> = {}) =>
    invoke<LibraryItem>("action_save_library", { input }),
  actionSaveNote: (input: Record<string, unknown> = {}, smart = false) =>
    invoke<Note>("action_save_note", { input, smart }),
  actionAskAi: (question: string) =>
    invoke<{ answer: string }>("action_ask_ai", { question }),
  getSettings: () => invoke<AppSettings>("get_settings"),
  saveSettings: (settings: AppSettings, apiKey?: string) =>
    invoke("save_settings", { settings, apiKey: apiKey ?? null }),
  rebindHotkey: (shortcut: string) =>
    invoke("rebind_hotkey", { shortcut }),
  ocrAvailable: () => invoke<boolean>("ocr_available"),
  getOverlayBackdrop: () => invoke<string | null>("get_overlay_backdrop"),
  listCollections: () => invoke<Collection[]>("list_collections"),
  createCollection: (name: string) =>
    invoke<Collection>("create_collection", { name }),
  listLibrary: (collectionId?: string) =>
    invoke<LibraryItem[]>("list_library", {
      collectionId: collectionId ?? null,
    }),
  searchLibrary: (query: string) =>
    invoke<LibraryItem[]>("search_library", { query }),
  searchAll: (query: string) => invoke<SearchHit[]>("search_all", { query }),
  deleteLibraryItem: (id: string) => invoke("delete_library_item", { id }),
  listNotes: () => invoke<Note[]>("list_notes"),
  searchNotes: (query: string) => invoke<Note[]>("search_notes", { query }),
  deleteNote: (id: string) => invoke("delete_note", { id }),
  listHistory: () => invoke<HistoryItem[]>("list_history"),
  deleteHistory: (id: string) => invoke("delete_history", { id }),
  clearHistory: () => invoke("clear_history"),
  readCaptureDataUrl: (path: string) =>
    invoke<string>("read_capture_data_url", { path }),
};
