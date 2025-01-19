import { DEFAULT_CONTENT, DEBOUNCE_DELAY } from '../config.js';
import { debounce, sanitizeInput } from '../utils/helpers.js';
import { AppState } from './State.js';
import { ParserManager } from './Parser.js';
import { events } from '../utils/events.js';

export class Editor {
	constructor() {
		this.state = AppState.getInstance();
		this.parser = ParserManager.getInstance();

		// DOM elements
		this.editor = document.getElementById('editor');
		this.preview = document.getElementById('preview');
		this.debugOutput = document.getElementById('debug-output');

		// Set up content change handling
		events.on('content-changed', (content) => {
			this.state.saveContent(content);
			this.state.updateWordCount(content);
			this.updatePreview();
		});

		// Set up toolbar actions
		events.on('toolbar-hr', () => this.insertHorizontalRule());
		events.on('toolbar-link', () => this.insertLink());
		events.on('toolbar-image', () => this.insertImage());
		events.on('toolbar-code', () => this.insertCodeBlock());
		events.on('toolbar-quote', () => this.insertQuote());
		events.on('toolbar-nested-quote', () => this.insertNestedQuote());

		// Keyboard shortcuts
		events.bindShortcuts([{
				key: 'b',
				ctrl: true,
				callback: () => this.insertMarkup('js', 'sj')
			},
			{
				key: 'i',
				ctrl: true,
				callback: () => this.insertMarkup('jd', 'dj')
			},
			{
				key: 'k',
				ctrl: true,
				callback: () => this.insertLink()
			},
			{
				key: 'q',
				ctrl: true,
				callback: () => this.insertQuote()
			}
		]);

		events.on('export-start', () => {
            // Show loading indicator
            document.getElementById('exportIndicator').classList.add('loading');
        });

        events.on('export-success', () => {
            // Hide loading indicator and show success message
            document.getElementById('exportIndicator').classList.remove('loading');
            this.state.showNotification('Export completed successfully');
        });

        events.on('export-error', ({ error }) => {
            // Hide loading indicator and show error message
            document.getElementById('exportIndicator').classList.remove('loading');
            this.state.showNotification(`Export failed: ${error.message}`, 'error');
        });

		this.bindEvents();
		this.loadContent();
	}

	async exportToPDF() {
        try {
            await exporter.toPDF(this.preview);
        } catch (error) {
            console.error('Export failed:', error);
        }
    }

    exportAsHTML() {
        try {
            exporter.toHTML(this.preview);
        } catch (error) {
            console.error('HTML export failed:', error);
        }
    }

    exportAsText() {
        try {
            exporter.toText(this.editor.value);
        } catch (error) {
            console.error('Text export failed:', error);
        }
    }

	bindEvents() {
		// Editor events
		events.bindEvents('editor', [{
				type: 'input',
				handler: (e) => events.emit('content-changed', e.target.value)
			},
			{
				type: 'blur',
				handler: () => this.state.saveContent(this.editor.value)
			}
		]);

		// Toolbar button events using delegation
		events.delegate(document.body, 'click', '.toolbar-button', (e, target) => {
			const action = target.dataset.action;
			switch (action) {
				case 'hr':
					this.insertHorizontalRule();
					break;
				case 'link':
					this.insertLink();
					break;
				case 'image':
					this.insertImage();
					break;
				case 'code':
					this.insertCodeBlock();
					break;
				case 'quote':
					this.insertQuote();
					break;
				case 'nested-quote':
					this.insertNestedQuote();
					break;
				case 'bold':
					this.insertMarkup('js', 'sj');
					break;
				case 'italic':
					this.insertMarkup('jd', 'dj');
					break;
				case 'underline':
					this.insertMarkup('ju', 'uj');
					break;
				default:
					console.warn(`Unknown toolbar action: ${action}`);
			}
		});

		// Export functionality
		events.bindEvents('exportPDF', [{
			type: 'click',
			handler: () => this.exportToPDF()
		}]);

		// Theme toggle
		events.bindEvents('darkModeToggle', [{
			type: 'click',
			handler: () => this.state.toggleDarkMode()
		}]);

		// Debug panel toggle
		events.bindEvents('debugToggle', [{
			type: 'click',
			handler: () => this.state.toggleDebug()
		}]);

		// Keyboard shortcuts
		events.bindShortcuts([{
				key: 'b',
				ctrl: true,
				callback: () => this.insertMarkup('js', 'sj')
			},
			{
				key: 'i',
				ctrl: true,
				callback: () => this.insertMarkup('jd', 'dj')
			},
			{
				key: 'u',
				ctrl: true,
				callback: () => this.insertMarkup('ju', 'uj')
			},
			{
				key: 'k',
				ctrl: true,
				callback: () => this.insertLink()
			},
			{
				key: 'q',
				ctrl: true,
				callback: () => this.insertQuote()
			},
			{
				key: 's',
				ctrl: true,
				callback: (e) => {
					e.preventDefault();
					this.state.saveContent(this.editor.value);
				}
			},
			{
				key: 'e',
				ctrl: true,
				callback: (e) => {
					e.preventDefault();
					this.exportToPDF();
				}
			}
		]);

		// Word count updates
		events.on('content-changed', (content) => {
			this.state.updateWordCount(content);
		});

		// Save indicator
		events.on('content-saved', () => {
			this.state.showSaveIndicator();
		});

		// Preview updates
		events.on('content-changed', () => {
			this.updatePreview();
		});

		// Handle window unload
		window.addEventListener('beforeunload', () => {
			this.state.saveContent(this.editor.value);
		});
	}

	loadContent() {
		this.editor.value = this.state.loadContent();
		this.updatePreview();
	}

	updatePreview() {
		try {
			const input = sanitizeInput(this.editor.value);
			const html = this.parser.parseToHtml(input);
			this.preview.innerHTML = html;
			hljs.highlightAll();
			this.logDebug("Preview updated", {
				input,
				html
			});
		} catch (error) {
			console.error("Parser error:", error);
			this.preview.innerHTML = `<pre class="error">Error: ${error.message}</pre>`;
			this.logDebug("Parser error", {
				error: error.message
			});
		}
	}

	insertMarkup(prefix, suffix = '', ensureNewline = true) {
		const start = this.editor.selectionStart;
		const end = this.editor.selectionEnd;
		const text = this.editor.value;
		const selectedText = text.slice(start, end);

		const prependNewline = (ensureNewline && start > 0 && text[start - 1] !== '\n') ? '\n' : '';
		const appendNewline = (ensureNewline && !suffix.endsWith('\n')) ? '\n' : '';

		const newText = text.slice(0, start) +
			prependNewline +
			prefix +
			selectedText +
			suffix +
			appendNewline +
			text.slice(end);

		this.editor.value = newText;
		this.state.saveContent(newText);
		this.updatePreview();

		const newPosition = start + prependNewline.length + prefix.length + selectedText.length + suffix.length;
		this.editor.setSelectionRange(newPosition, newPosition);
		this.editor.focus();
	}

	// Insert methods remain the same
	insertHorizontalRule() {
		this.insertMarkup('js', '\n', true);
	}
	insertLink() {
		this.insertMarkup(`jg [text] gh [url] hg`, '\n');
	}
	insertImage() {
		this.insertMarkup(`jh [alt] gh [url] hj`, '\n', true);
	}
	insertCodeBlock() {
		this.insertMarkup(`jkd [prefix] \n [code] \ndkj`, '\n', true);
	}
	insertQuote() {
		this.insertMarkup('kl ', '\n', true);
	}
	insertNestedQuote() {
		this.insertMarkup('kll ', '\n', true);
	}

	logDebug(message, details = null) {
		if (!this.state.isDebugMode()) return;

		console.log(message);
		if (this.debugOutput) {
			let logEntry = message;
			if (details) {
				logEntry += '\n' + JSON.stringify(details, null, 2);
			}
			this.debugOutput.textContent += logEntry + '\n\n';
		}
	}
}
