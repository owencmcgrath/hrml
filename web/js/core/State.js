import { DEFAULT_CONTENT } from '../config.js';
import { events } from '../utils/events.js';

export class AppState {
    static #instance = null;

    constructor() {
        if (AppState.#instance) {
            return AppState.#instance;
        }

        this.initializeState();
        AppState.#instance = this;
    }

    initializeState() {
        // Theme state
        this.darkMode = localStorage.getItem('darkMode') === 'true';

        // Content state
        this.content = localStorage.getItem('hrmlContent') || DEFAULT_CONTENT;

        // Debug state
        this.debugMode = false;

        // UI state
        this.uiState = {
            saveIndicatorVisible: false,
            wordCount: 0,
            charCount: 0,
            lastSaved: null,
        };

        // Theme elements
        this.themeElements = {
            sunIcon: document.querySelector('.theme-toggle .sun'),
            moonIcon: document.querySelector('.theme-toggle .moon'),
        };

        this.applyInitialTheme();
    }

    static getInstance() {
        if (!AppState.#instance) {
            AppState.#instance = new AppState();
        }
        return AppState.#instance;
    }

    // Theme management
    toggleDarkMode() {
        this.darkMode = !this.darkMode;
        document.body.classList.toggle('dark-mode', this.darkMode);
        localStorage.setItem('darkMode', this.darkMode);
        this.updateThemeIcons();
    }

    applyInitialTheme() {
        document.body.classList.toggle('dark-mode', this.darkMode);
        this.updateThemeIcons();
    }

    updateThemeIcons() {
        const {
            sunIcon,
            moonIcon
        } = this.themeElements;
        if (this.darkMode) {
            sunIcon.style.display = 'block';
            moonIcon.style.display = 'none';
        } else {
            sunIcon.style.display = 'none';
            moonIcon.style.display = 'block';
        }
    }

    // Content management
    saveContent(content) {
        this.content = content;
        localStorage.setItem('hrmlContent', content);
        this.uiState.lastSaved = new Date();
        this.showSaveIndicator();
    }

    loadContent() {
        return this.content;
    }

    // Word count management
    updateWordCount(text) {
        this.uiState.wordCount = text.trim().split(/\s+/)
            .filter(word => word.length > 0).length;
        this.uiState.charCount = text.length;
        this.updateWordCountDisplay();
    }

    updateWordCountDisplay() {
        const counter = document.getElementById('wordCounter');
        if (counter) {
            counter.textContent =
                `Words: ${this.uiState.wordCount} | Characters: ${this.uiState.charCount}`;
        }
    }

    // UI state management
    showSaveIndicator() {
        const indicator = document.getElementById('saveIndicator');
        if (!indicator) return;

        this.uiState.saveIndicatorVisible = true;
        indicator.classList.add('show');

        setTimeout(() => {
            indicator.classList.remove('show');
            this.uiState.saveIndicatorVisible = false;
        }, 2000);
    }

    // Debug management
    toggleDebug() {
        this.debugMode = !this.debugMode;
        const debugPanel = document.getElementById('debug-panel');
        if (debugPanel) {
            debugPanel.style.display = this.debugMode ? 'block' : 'none';
        }
        return this.debugMode;
    }

    isDebugMode() {
        return this.debugMode;
    }

    // Export current state (useful for debugging or saving)
    exportState() {
        return {
            darkMode: this.darkMode,
            content: this.content,
            debugMode: this.debugMode,
            uiState: {
                ...this.uiState
            },
            lastSaved: this.uiState.lastSaved?.toISOString()
        };
    }
}
