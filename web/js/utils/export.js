import { AppState } from '../core/State.js';
import { events } from './events.js';

export class Exporter {
    static #instance = null;

    constructor() {
        if (Exporter.#instance) {
            return Exporter.#instance;
        }
        this.state = AppState.getInstance();
        Exporter.#instance = this;
    }

    static getInstance() {
        if (!Exporter.#instance) {
            Exporter.#instance = new Exporter();
        }
        return Exporter.#instance;
    }

    // PDF export configuration
    static PDF_OPTIONS = {
        margin: 1,
        filename: 'hrml-document.pdf',
        image: { type: 'jpeg', quality: 0.98 },
        html2canvas: {
            scale: 2,
            useCORS: true,
            logging: false
        },
        jsPDF: {
            unit: 'in',
            format: 'letter',
            orientation: 'portrait'
        }
    };

    /**
     * Exports content to PDF
     * @param {HTMLElement} element - The element to export
     * @param {Object} options - Optional configuration overrides
     * @returns {Promise<void>}
     */
    async toPDF(element, options = {}) {
        try {
            events.emit('export-start', { type: 'pdf' });

            const mergedOptions = {
                ...Exporter.PDF_OPTIONS,
                ...options,
                filename: options.filename || `document-${new Date().toISOString()}.pdf`
            };

            await html2pdf()
                .set(mergedOptions)
                .from(element)
                .save();

            events.emit('export-success', { type: 'pdf' });
        } catch (error) {
            console.error('PDF export failed:', error);
            events.emit('export-error', { type: 'pdf', error });
            throw new Error(`PDF export failed: ${error.message}`);
        }
    }

    /**
     * Exports content as HTML
     * @param {HTMLElement} element - The element to export
     * @returns {string} HTML string
     */
    toHTML(element) {
        try {
            events.emit('export-start', { type: 'html' });

            const html = element.innerHTML;
            const blob = new Blob([html], { type: 'text/html' });
            const url = URL.createObjectURL(blob);

            this.downloadFile(url, 'document.html');
            events.emit('export-success', { type: 'html' });

            return html;
        } catch (error) {
            console.error('HTML export failed:', error);
            events.emit('export-error', { type: 'html', error });
            throw new Error(`HTML export failed: ${error.message}`);
        }
    }

    /**
     * Exports raw content
     * @param {string} content - The content to export
     * @returns {string} Raw content
     */
    toText(content) {
        try {
            events.emit('export-start', { type: 'text' });

            const blob = new Blob([content], { type: 'text/plain' });
            const url = URL.createObjectURL(blob);

            this.downloadFile(url, 'document.txt');
            events.emit('export-success', { type: 'text' });

            return content;
        } catch (error) {
            console.error('Text export failed:', error);
            events.emit('export-error', { type: 'text', error });
            throw new Error(`Text export failed: ${error.message}`);
        }
    }

    /**
     * Helper method to trigger file download
     * @private
     */
    downloadFile(url, filename) {
        const link = document.createElement('a');
        link.href = url;
        link.download = filename;
        document.body.appendChild(link);
        link.click();
        document.body.removeChild(link);
        URL.revokeObjectURL(url);
    }
}

// Export singleton instance
export const exporter = Exporter.getInstance();
