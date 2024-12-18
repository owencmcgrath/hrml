// Import WASM at the top of the file
import init, { MarkupParserWrapper } from '../wasm/markup_parser.js';

let editor = document.getElementById('editor');
let preview = document.getElementById('preview');
let parser = null;

// Initialize the parser
async function initParser() {
    try {
        await init();
        parser = new MarkupParserWrapper();
        console.log("Parser initialized!");
        updatePreview(); // Initial preview
    } catch (error) {
        console.error("Parser initialization failed:", error);
    }
}

function updatePreview() {
    if (parser) {
        try {
            const html = parser.parse_to_html(editor.value);
            preview.innerHTML = html;
        } catch (error) {
            console.error("Error parsing content:", error);
            preview.textContent = "Error parsing content";
        }
    } else {
        preview.textContent = editor.value;
    }
}

function insertMarkup(prefix, suffix = '') {
    let start = editor.selectionStart;
    let end = editor.selectionEnd;
    let text = editor.value;
    let selectedText = text.slice(start, end);

    let newText = text.slice(0, start) +
                  prefix +
                  (selectedText || 'text') +
                  suffix +
                  text.slice(end);

    editor.value = newText;
    updatePreview();

    // Reset cursor position
    let newPosition = start + prefix.length + (selectedText || 'text').length + suffix.length;
    editor.setSelectionRange(newPosition, newPosition);
    editor.focus();
}

// Add event listener for input
editor.addEventListener('input', updatePreview);

// Make insertMarkup available globally
window.insertMarkup = insertMarkup;

// Initialize when the page loads
initParser();
