import init, { MarkupParserWrapper } from '../wasm/markup_parser.js';

// Make parser globally accessible
window.parser = null;
let editor = document.getElementById('editor');
let preview = document.getElementById('preview');
let debugOutput = document.getElementById('debug-output');
let debugMode = false; // Add this line - it was missing

const defaultContent = `jf hey
jff welcome to the HRML demo
jfff you can do up to six headings!
This is some bold js text sj, some italic jd text dj and some underlined ju text uj!
kl let's create a list!
ja info
ja more info
jl first
jl second
jl third!
kll some code!
jkd python
# Some Python!
print("this is so much better than Markdown, right? ;)")
dkj
jffff cool car!
jh [dream car, really] gh [https://i.kinja-img.com/image/upload/c_fill,h_675,pg_1,q_80,w_1200/823337c1eb4bc7e0ffc884d3eaf1fb22.jpg] hj
js
jg [check out my other work!] gh [https://owencmcgrath.com] hg`;

// Debounce function to prevent too frequent updates
function debounce(func, wait) {
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

// Detailed logging function
function logDebug(message, details = null) {
    if (!debugMode) return;

    console.log(message);
    let logEntry = message;
    if (details) {
        logEntry += '\n' + JSON.stringify(details, null, 2);
    }
    debugOutput.textContent += logEntry + '\n\n';
}

function sanitizeInput(input) {
    return input
        .replace(/\0/g, '')
        .replace(/\r\n/g, '\n')
        .replace(/\r/g, '\n');
}

// Initialize the parser
async function initParser() {
    try {
        await init();
        window.parser = new MarkupParserWrapper();
        console.log("Parser initialized");

        // Load saved content after parser is initialized
        loadContent(); // Add this line

        // Run tests
        // await testParser();
    } catch (error) {
        console.error("Failed to initialize parser:", error);
        debugOutput.textContent = `Error: ${error.message}`;
    }
}

function updatePreview() {
    try {
        const input = sanitizeInput(editor.value);
        console.log("Processing input:", input);

        const html = window.parser.parse_to_html(input);
        console.log("Generated HTML:", html);

        preview.innerHTML = html;
        hljs.highlightAll(); // Add this line to enable syntax highlighting

        logDebug("Preview updated", { input, html });

        showSaveIndicator();
    } catch (error) {
        console.error("Parser error:", error);
        preview.innerHTML = `<pre style="color: red;">Error: ${error.message}</pre>`;
        logDebug("Parser error", { error: error.message });
    }
}

function insertMarkup(prefix, suffix = '', ensureNewline = true) {
    let start = editor.selectionStart;
    let end = editor.selectionEnd;
    let text = editor.value;
    let selectedText = text.slice(start, end);

    let prependNewline = '';
    if (ensureNewline && start > 0 && text[start - 1] !== '\n') {
        prependNewline = '\n';
    }

    let appendNewline = '';
    if (ensureNewline && !suffix.endsWith('\n')) {
        appendNewline = '\n';
    }

    let newText = text.slice(0, start) +
                  prependNewline +
                  prefix +
                  (selectedText) +
                  suffix +
                  appendNewline +
                  text.slice(end);

    editor.value = newText;
    updatePreview();

    let newPosition = start + prependNewline.length + prefix.length + (selectedText || 'text').length + suffix.length;
    editor.setSelectionRange(newPosition, newPosition);
    editor.focus();
}

// Insert functions
function insertHorizontalRule() {
    insertMarkup('js', '\n', true);
}

function insertLink() {
  insertMarkup(`jg [text] gh [url] hg`, '\n');
}

function insertImage() {
  insertMarkup(`jh [alt] gh [url] hj`, '\n', true);
}

function insertCodeBlock() {
  insertMarkup(`jkd [prefix] \n [code] \ndkj`, '\n', true);
}

function insertQuote() {
    insertMarkup('kl ', '\n', true);
}

function insertNestedQuote() {
    insertMarkup('kll ', '\n', true);
}

function exportToPDF() {
    const element = document.getElementById('preview');
    const opt = {
        margin: 1,
        filename: 'hrml-document.pdf',
        image: { type: 'jpeg', quality: 0.98 },
        html2canvas: { scale: 2 },
        jsPDF: { unit: 'in', format: 'letter', orientation: 'portrait' }
    };

    html2pdf().set(opt).from(element).save();
}

// Add a helper function to visualize HTML structure
function visualizeHTML(html) {
    return html
        .replace(/</g, '&lt;')
        .replace(/>/g, '&gt;')
        .replace(/\n/g, '<br>')
        .replace(/\s{2}/g, '&nbsp;&nbsp;');
}

function toggleDarkMode() {
    document.body.classList.toggle('dark-mode');
    localStorage.setItem('darkMode', document.body.classList.contains('dark-mode'));

    // Update icon visibility
    const sunIcon = document.querySelector('.theme-toggle .sun');
    const moonIcon = document.querySelector('.theme-toggle .moon');

    if (document.body.classList.contains('dark-mode')) {
        sunIcon.style.display = 'block';
        moonIcon.style.display = 'none';
    } else {
        sunIcon.style.display = 'none';
        moonIcon.style.display = 'block';
    }
}

if (localStorage.getItem('darkMode') === 'true') {
    document.body.classList.add('dark-mode');
    // Set initial icon state
    const sunIcon = document.querySelector('.theme-toggle .sun');
    const moonIcon = document.querySelector('.theme-toggle .moon');
    sunIcon.style.display = 'block';
    moonIcon.style.display = 'none';
} else {
    const sunIcon = document.querySelector('.theme-toggle .sun');
    const moonIcon = document.querySelector('.theme-toggle .moon');
    sunIcon.style.display = 'none';
    moonIcon.style.display = 'block';
}

function showSaveIndicator() {
    const indicator = document.getElementById('saveIndicator');
    indicator.classList.add('show');
    setTimeout(() => {
        indicator.classList.remove('show');
    }, 2000);
}

function updateWordCount() {
    const text = editor.value;
    const words = text.trim().split(/\s+/).filter(word => word.length > 0).length;
    const chars = text.length;
    document.getElementById('wordCounter').textContent =
        `Words: ${words} | Characters: ${chars}`;
}

editor.addEventListener('input', updateWordCount);

document.getElementById('darkModeToggle').addEventListener('click', toggleDarkMode);

// Event listeners
const debouncedUpdate = debounce(updatePreview, 250);
editor.addEventListener('input', debouncedUpdate);

function saveContent() {
    const content = editor.value;
    localStorage.setItem('hrmlContent', content);
}

function loadContent() {
    const savedContent = localStorage.getItem('hrmlContent');
    if (savedContent) {
        editor.value = savedContent;
        updatePreview();
    } else {
        editor.value = defaultContent;
        updatePreview();
    }
}

// Remove the existing editor.addEventListener for input
// and replace with this one (place near other event listeners)
editor.addEventListener('input', debounce(() => {
    saveContent();
    updatePreview();
}, 250));

// Make functions globally available
Object.assign(window, {
    insertMarkup,
    insertHorizontalRule,
    insertLink,
    insertImage,
    insertCodeBlock,
    insertQuote,
    insertNestedQuote,
    updatePreview,
    exportToPDF,
    saveContent
});

// Initialize the parser (only once)
initParser().catch(console.error);

// function toggleDebug() {
//     debugMode = !debugMode;
//     const debugPanel = document.getElementById('debug-panel');
//     debugPanel.style.display = debugMode ? 'block' : 'none';
//     debugOutput.textContent = ''; // Clear previous debug output
//     if (debugMode) {
//         updatePreview();
//     }
// }

// async function testParser() {
//     if (!window.parser) {
//         console.error("Parser not initialized");
//         return;
//     }

//     const testCases = [
//         // Original tests
//         "js bold text sj",
//         "ja list item",
//         "kl quote",
//         "jl numbered item",
//         "jf heading",

//         // New list-specific tests
//         "ja simple list item",
//         "ja list item with js bold sj",
//         "ja list item with jd italic dj",

//         // Multiple list items
//         `ja first item
// ja second item
// ja third item`,

//         // Mixed list types
//         `ja unordered item 1
// jl ordered item 1
// ja unordered item 2`,

//         // Lists with inline formatting
//         `ja item with js bold text sj
// ja item with jd italic text dj
// ja item with ju underlined text uj`,

//         // Complex list items
//         `ja item with js bold sj and jd italic dj
// ja item with a js bold sj section`,

//         // Edge cases
//         "ja ",  // Empty list item
//         "ja  test", // Multiple spaces
//         "ja\ttest"  // Tab character
//     ];

//     console.group("Parser Tests");
//     for (const test of testCases) {
//         console.log("=".repeat(50));
//         console.log("Test input:", test);
//         try {
//             const result = window.parser.parse_to_html(test);
//             console.log("Test output:", result);
//             logDebug("Test case", {
//                 input: test,
//                 output: result,
//                 outputHTML: result,  // Shows actual HTML
//                 containsContent: result.includes(">test<"),  // Check if content is present
//                 listTags: {
//                     hasUL: result.includes("<ul>"),
//                     hasLI: result.includes("<li>"),
//                     emptyLI: result.includes("<li></li>")
//                 }
//             });
//         } catch (error) {
//             console.error("Test failed:", error);
//             logDebug("Test failed", { input: test, error: error.message });
//         }
//     }
//     console.groupEnd();
// }
