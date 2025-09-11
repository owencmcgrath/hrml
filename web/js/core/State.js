import { DEFAULT_CONTENT } from "../config.js";
import { events } from "../utils/events.js";

export class AppState {
  static #instance = null;

  constructor() {
    if (AppState.#instance) {
      return AppState.#instance;
    }

    this.initializeState();
    AppState.#instance = this;
  }

  showNotification(message, type = "info") {
    const notification = document.getElementById("notification");
    if (!notification) return;

    notification.textContent = message;
    notification.className = `notification ${type}`;
    notification.style.display = "block";

    setTimeout(() => {
      notification.style.display = "none";
    }, 3000);
  }

  initializeState() {
    this.darkMode = localStorage.getItem("darkMode") === "true";
    this.content = localStorage.getItem("hrmlContent") || DEFAULT_CONTENT;
    this.debugMode = false;

    this.uiState = {
      saveIndicatorVisible: false,
      wordCount: 0,
      charCount: 0,
      lastSaved: null,
    };

    this.themeElements = {
      sunIcon: document.querySelector(".theme-toggle .sun"),
      moonIcon: document.querySelector(".theme-toggle .moon"),
    };

    this.applyInitialTheme();
  }

  static getInstance() {
    if (!AppState.#instance) {
      AppState.#instance = new AppState();
    }
    return AppState.#instance;
  }

  toggleDarkMode() {
    this.darkMode = !this.darkMode;
    document.body.classList.toggle("dark-mode", this.darkMode);
    localStorage.setItem("darkMode", this.darkMode);
    this.updateThemeIcons();
  }

  applyInitialTheme() {
    document.body.classList.toggle("dark-mode", this.darkMode);
    this.updateThemeIcons();
  }

  updateThemeIcons() {
    const { sunIcon, moonIcon } = this.themeElements;
    if (this.darkMode) {
      sunIcon.style.display = "block";
      moonIcon.style.display = "none";
    } else {
      sunIcon.style.display = "none";
      moonIcon.style.display = "block";
    }
  }

  saveContent(content) {
    this.content = content;
    localStorage.setItem("hrmlContent", content);
    this.uiState.lastSaved = new Date();
    this.showSaveIndicator();
  }

  loadContent() {
    return this.content;
  }

  updateWordCount(text) {
    this.uiState.wordCount = text
      .trim()
      .split(/\s+/)
      .filter((word) => word.length > 0).length;
    this.uiState.charCount = text.length;
    this.updateWordCountDisplay();
  }

  updateWordCountDisplay() {
    const counter = document.getElementById("wordCounter");
    if (counter) {
      counter.textContent = `Words: ${this.uiState.wordCount} | Characters: ${this.uiState.charCount}`;
    }
  }

  showSaveIndicator() {
    const indicator = document.getElementById("saveIndicator");
    if (!indicator) return;

    this.uiState.saveIndicatorVisible = true;
    indicator.classList.add("show");

    setTimeout(() => {
      indicator.classList.remove("show");
      this.uiState.saveIndicatorVisible = false;
    }, 2000);
  }

  toggleDebug() {
    this.debugMode = !this.debugMode;
    const debugPanel = document.getElementById("debug-panel");
    if (debugPanel) {
      debugPanel.style.display = this.debugMode ? "block" : "none";
    }
    return this.debugMode;
  }

  isDebugMode() {
    return this.debugMode;
  }

  exportState() {
    return {
      darkMode: this.darkMode,
      content: this.content,
      debugMode: this.debugMode,
      uiState: {
        ...this.uiState,
      },
      lastSaved: this.uiState.lastSaved?.toISOString(),
    };
  }
}
