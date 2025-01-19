import { DEBOUNCE_DELAY } from '../config.js';

export class EventManager {
    static #instance = null;

    constructor() {
        if (EventManager.#instance) {
            return EventManager.#instance;
        }
        this.handlers = new Map();
        this.initialize();
        EventManager.#instance = this;
    }

    initialize() {
        // Any initialization logic here
    }

    static getInstance() {
        if (!EventManager.#instance) {
            EventManager.#instance = new EventManager();
        }
        return EventManager.#instance;
    }

    // Event delegation helper
    delegate(element, eventType, selector, handler) {
        const wrappedHandler = (e) => {
            const target = e.target.closest(selector);
            if (target && element.contains(target)) {
                handler(e, target);
            }
        };
        element.addEventListener(eventType, wrappedHandler);
        return () => element.removeEventListener(eventType, wrappedHandler);
    }

    // Debounce utility
    debounce(func, wait = DEBOUNCE_DELAY) {
        let timeout;
        return function executedFunction(...args) {
            const later = () => {
                clearTimeout(timeout);
                func(...args);
            };
            clearTimeout(timeout);
            timeout = setTimeout(later, wait);
        };
    }

    // Custom event emitter
    on(eventName, handler) {
        if (!this.handlers.has(eventName)) {
            this.handlers.set(eventName, new Set());
        }
        this.handlers.get(eventName).add(handler);
        return () => this.off(eventName, handler);
    }

    off(eventName, handler) {
        const handlers = this.handlers.get(eventName);
        if (handlers) {
            handlers.delete(handler);
        }
    }

    emit(eventName, data) {
        const handlers = this.handlers.get(eventName);
        if (handlers) {
            handlers.forEach(handler => handler(data));
        }
    }

    // Keyboard shortcuts
    bindShortcuts(shortcuts) {
        document.addEventListener('keydown', (e) => {
            shortcuts.forEach(({ key, ctrl, alt, shift, callback }) => {
                if (e.key.toLowerCase() === key.toLowerCase() &&
                    e.ctrlKey === !!ctrl &&
                    e.altKey === !!alt &&
                    e.shiftKey === !!shift) {
                    e.preventDefault();
                    callback(e);
                }
            });
        });
    }

    // DOM event binding with automatic cleanup
    bindEvents(elementId, events) {
        const element = document.getElementById(elementId);
        if (!element) return null;

        const cleanupFunctions = events.map(({ type, handler }) => {
            const wrappedHandler = this.debounce(handler);
            element.addEventListener(type, wrappedHandler);
            return () => element.removeEventListener(type, wrappedHandler);
        });

        return () => cleanupFunctions.forEach(cleanup => cleanup());
    }
}

// Export singleton instance
export const events = EventManager.getInstance();
