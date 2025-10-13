export interface PluginOptions {
  /** Remove prefix from file path */
  removePrefix?: boolean | string | RegExp;
  /** Include filename in ID */
  filebase?: boolean;
  /** Include export name in ID */
  includeExportName?: boolean | 'all';
  /** Extract comments as descriptions */
  extractComments?: boolean;
  /** Use key property instead of hash */
  useKey?: boolean;
  /** Module source name */
  moduleSourceName?: string;
  /** ID separator */
  separator?: string;
  /** Relative path for ID generation */
  relativeTo?: string;
}

/**
 * SWC plugin for automatic react-intl ID management
 */
declare function swcPluginReactIntlAuto(options?: PluginOptions): {
  name: string;
  config: () => {
    jsc: {
      parser: {
        syntax: 'typescript';
        tsx: boolean;
        decorators: boolean;
      };
      transform: {
        react: {
          runtime: 'automatic';
        };
      };
      experimental: {
        plugins: [string, PluginOptions][];
      };
    };
  };
};

export = swcPluginReactIntlAuto;
