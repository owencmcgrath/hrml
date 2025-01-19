/**
 * Debounces a function
 * @param {Function} func - Function to debounce
 * @param {number} wait - Milliseconds to wait
 * @returns {Function} Debounced function
 */
export function debounce(func, wait) {
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

/**
 * Sanitizes input text
 * @param {string} input - Text to sanitize
 * @returns {string} Sanitized text
 */
export function sanitizeInput(input) {
    if (typeof input !== 'string') {
        return '';
    }
    return input
        .replace(/\0/g, '') // Remove null bytes
        .replace(/\r\n/g, '\n') // Normalize line endings
        .replace(/\r/g, '\n');
}

/**
 * Generates a unique ID
 * @returns {string} Unique ID
 */
export function generateId() {
    return Date.now().toString(36) + Math.random().toString(36).substr(2);
}

/**
 * Deep clones an object
 * @param {Object} obj - Object to clone
 * @returns {Object} Cloned object
 */
export function deepClone(obj) {
    if (obj === null || typeof obj !== 'object') {
        return obj;
    }
    return JSON.parse(JSON.stringify(obj));
}

/**
 * Formats a date
 * @param {Date|string|number} date - Date to format
 * @param {string} [format='short'] - Format style ('short', 'long', 'relative')
 * @returns {string} Formatted date
 */
export function formatDate(date, format = 'short') {
    const d = new Date(date);

    if (isNaN(d.getTime())) {
        return 'Invalid date';
    }

    if (format === 'relative') {
        const diff = (new Date() - d) / 1000;
        if (diff < 60) return 'just now';
        if (diff < 3600) return `${Math.floor(diff / 60)}m ago`;
        if (diff < 86400) return `${Math.floor(diff / 3600)}h ago`;
        return d.toLocaleDateString();
    }

    if (format === 'long') {
        return d.toLocaleString();
    }

    return d.toLocaleDateString();
}

/**
 * Truncates text to a specified length
 * @param {string} text - Text to truncate
 * @param {number} length - Maximum length
 * @param {string} [suffix='...'] - Suffix to add
 * @returns {string} Truncated text
 */
export function truncate(text, length, suffix = '...') {
    if (text.length <= length) return text;
    return text.substring(0, length - suffix.length) + suffix;
}

/**
 * Validates an email address
 * @param {string} email - Email to validate
 * @returns {boolean} Whether email is valid
 */
export function isValidEmail(email) {
    const re = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;
    return re.test(email);
}

/**
 * Formats file size
 * @param {number} bytes - Size in bytes
 * @returns {string} Formatted size
 */
export function formatFileSize(bytes) {
    if (bytes === 0) return '0 Bytes';
    const k = 1024;
    const sizes = ['Bytes', 'KB', 'MB', 'GB', 'TB'];
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i];
}

/**
 * Escapes HTML special characters
 * @param {string} html - String to escape
 * @returns {string} Escaped string
 */
export function escapeHtml(html) {
    const div = document.createElement('div');
    div.textContent = html;
    return div.innerHTML;
}

/**
 * Checks if an element is in viewport
 * @param {HTMLElement} element - Element to check
 * @returns {boolean} Whether element is in viewport
 */
export function isInViewport(element) {
    const rect = element.getBoundingClientRect();
    return (
        rect.top >= 0 &&
        rect.left >= 0 &&
        rect.bottom <= (window.innerHeight || document.documentElement.clientHeight) &&
        rect.right <= (window.innerWidth || document.documentElement.clientWidth)
    );
}

/**
 * Retrieves a value from localStorage with error handling
 * @param {string} key - Key to retrieve
 * @param {*} defaultValue - Default value if key doesn't exist
 * @returns {*} Retrieved value or default
 */
export function getFromStorage(key, defaultValue = null) {
    try {
        const item = localStorage.getItem(key);
        return item ? JSON.parse(item) : defaultValue;
    } catch (error) {
        console.error(`Error reading from localStorage:`, error);
        return defaultValue;
    }
}

/**
 * Saves a value to localStorage with error handling
 * @param {string} key - Key to save under
 * @param {*} value - Value to save
 * @returns {boolean} Whether save was successful
 */
export function saveToStorage(key, value) {
    try {
        localStorage.setItem(key, JSON.stringify(value));
        return true;
    } catch (error) {
        console.error(`Error saving to localStorage:`, error);
        return false;
    }
}

/**
 * Copies text to clipboard
 * @param {string} text - Text to copy
 * @returns {Promise<boolean>} Whether copy was successful
 */
export async function copyToClipboard(text) {
    try {
        await navigator.clipboard.writeText(text);
        return true;
    } catch (error) {
        console.error('Failed to copy:', error);
        return false;
    }
}

/**
 * Creates a throttled function
 * @param {Function} func - Function to throttle
 * @param {number} limit - Throttle limit in milliseconds
 * @returns {Function} Throttled function
 */
export function throttle(func, limit) {
    let inThrottle;
    return function(...args) {
        if (!inThrottle) {
            func.apply(this, args);
            inThrottle = true;
            setTimeout(() => inThrottle = false, limit);
        }
    };
}

/**
 * Checks if running in development mode
 * @returns {boolean} Whether in development mode
 */
export function isDevelopment() {
    return process.env.NODE_ENV === 'development';
}

/**
 * Logs messages only in development mode
 * @param {...*} args - Arguments to log
 */
export function devLog(...args) {
    if (isDevelopment()) {
        console.log(...args);
    }
}
