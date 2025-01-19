import init, { MarkupParserWrapper } from '../../wasm/markup_parser.js';
import { AppState } from './State.js';

export class ParserManager {
    static #instance = null;

    constructor() {
        if (ParserManager.#instance) {
            return ParserManager.#instance;
        }
        this.state = AppState.getInstance();
        this.parser = null;
        this.isInitialized = false;
        ParserManager.#instance = this;
    }

    static getInstance() {
        if (!ParserManager.#instance) {
            ParserManager.#instance = new ParserManager();
        }
        return ParserManager.#instance;
    }

    async initialize() {
        if (this.isInitialized) return true;

        try {
            await init();
            this.parser = new MarkupParserWrapper();
            this.isInitialized = true;
            console.log("Parser initialized successfully");
            return true;
        } catch (error) {
            console.error("Failed to initialize parser:", error);
            throw new Error(`Parser initialization failed: ${error.message}`);
        }
    }

    parseToHtml(input) {
        if (!this.isInitialized) {
            throw new Error("Parser not initialized");
        }

        try {
            return this.parser.parse_to_html(input);
        } catch (error) {
            console.error("Parsing error:", error);
            throw new Error(`Parsing failed: ${error.message}`);
        }
    }

    // Testing functionality moved to a separate TestManager class if needed
}
