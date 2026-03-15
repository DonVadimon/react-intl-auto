/**
 * Options for message extraction
 */
export interface ExtractOptions {
    /** Remove prefix from path (true, false, or specific prefix string) */
    removePrefix?: string;
    /** Module source name for react-intl imports (default: 'react-intl') */
    moduleSourceName?: string;
    /** Separator for ID path generation (default: '.') */
    separator?: string;
    /** Base path for relative path calculation */
    relativeTo?: string;
    /** Hash message IDs */
    hashId?: boolean;
    /** Hash algorithm: 'murmur3' or 'base64' (default: 'murmur3') */
    hashAlgorithm?: string;
    /** Include source file path in output */
    extractSourceLocation?: boolean;
    /** Output mode: 'aggregated' or 'perfile' (default: 'aggregated') */
    outputMode?: string;
}

/**
 * An extracted React Intl message
 */
export interface ExtractedMessage {
    /** Message ID */
    id: string;
    /** Default message text */
    defaultMessage: string;
    /** Optional description for translators */
    description?: string;
    /** Source file path (if extractSourceLocation is true) */
    file?: string;
}

/**
 * Result of message extraction
 */
export interface ExtractResult {
    /** Array of extracted messages */
    messages: ExtractedMessage[];
    /** Number of files processed */
    filesProcessed: number;
}

/**
 * Extract messages from files matching glob patterns (async)
 * @param patterns - Glob patterns for source files (e.g., `['src/*.{ts,tsx}']`)
 * @param options - Extraction options
 * @returns Promise resolving to extraction result
 */
export function extract(
    patterns: string[],
    options?: ExtractOptions
): Promise<ExtractResult>;

/**
 * Extract messages from files matching glob patterns (sync)
 * @param patterns - Glob patterns for source files (e.g., `['src/*.{ts,tsx}']`)
 * @param options - Extraction options
 * @returns Extraction result
 */
export function extractSync(
    patterns: string[],
    options?: ExtractOptions
): ExtractResult;

/**
 * Parse a single file and extract messages
 * @param filePath - Path to the file to parse
 * @param options - Extraction options
 * @returns Array of extracted messages
 */
export function parseFile(
    filePath: string,
    options?: ExtractOptions
): ExtractedMessage[];
