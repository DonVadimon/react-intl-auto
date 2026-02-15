//! CLI tool for extracting React Intl messages from source files
use globset::{Glob, GlobSetBuilder};
use walkdir::WalkDir;

use anyhow::{Context, Result};
use clap::Parser;
use react_intl_core::{extract_messages, CoreOptions, IncludeExportName, RemovePrefix};
use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};

/// React Intl Message Extractor CLI
#[derive(Parser, Debug)]
#[command(
    name = "react-intl-extract",
    about = "Extract React Intl messages from source files",
    version
)]
struct Args {
    /// File glob patterns to extract from (e.g., 'src/**/*.{ts,tsx}')
    #[arg(help = "Glob patterns for source files (e.g., 'src/**/*.{ts,tsx}')")]
    patterns: Vec<PathBuf>,

    /// Glob patterns to ignore (e.g., 'node_modules/**', '.git/**')
    #[arg(
        long,
        help = "Glob patterns to ignore (can be specified multiple times)",
        default_values = &["**/node_modules/**", "**/.git/**"]
    )]
    ignore: Vec<PathBuf>,

    /// Output file or directory
    #[arg(short, long, help = "Output file or directory")]
    output: Option<String>,

    /// Include source file location (file path and line number)
    #[arg(long, help = "Include source file location")]
    extract_source_location: bool,

    // CoreOptions as CLI arguments
    /// Remove prefix from path (can be 'true', 'false', or a specific prefix string)
    #[arg(
        long,
        help = "Remove prefix from path (true, false, or specific prefix)"
    )]
    remove_prefix: Option<String>,

    /// Use file basename for ID generation
    #[arg(long, help = "Use file basename for ID generation")]
    filebase: bool,

    /// Include export name in ID (can be 'true', 'false', or 'all')
    #[arg(long, help = "Include export name in ID (true, false, or all)")]
    include_export_name: Option<String>,

    /// Use object key for ID generation in defineMessages
    #[arg(long, help = "Use object key for ID generation in defineMessages")]
    use_key: bool,

    /// Module source name for react-intl imports
    #[arg(
        long,
        default_value = "react-intl",
        help = "Module source name for react-intl imports"
    )]
    module_source_name: String,

    /// Separator for ID path generation
    #[arg(long, default_value = ".", help = "Separator for ID path generation")]
    separator: String,

    /// Base path for relative path calculation
    #[arg(long, help = "Base path for relative path calculation")]
    relative_to: Option<String>,

    /// Hash message IDs
    #[arg(long, help = "Hash message IDs")]
    hash_id: bool,

    /// Hash algorithm (murmur3 or base64)
    #[arg(
        long,
        default_value = "murmur3",
        help = "Hash algorithm (murmur3 or base64)"
    )]
    hash_algorithm: String,
}

impl Args {
    /// Convert CLI arguments to CoreOptions
    fn to_core_options(&self) -> CoreOptions {
        let remove_prefix = self.remove_prefix.as_ref().and_then(|s| match s.as_str() {
            "true" => Some(RemovePrefix::Boolean(true)),
            "false" => Some(RemovePrefix::Boolean(false)),
            "" => None,
            _ => Some(RemovePrefix::String(s.clone())),
        });

        let include_export_name =
            self.include_export_name
                .as_ref()
                .and_then(|s| match s.as_str() {
                    "true" => Some(IncludeExportName::Boolean(true)),
                    "false" => Some(IncludeExportName::Boolean(false)),
                    "all" => Some(IncludeExportName::All),
                    _ => None,
                });

        CoreOptions {
            remove_prefix,
            filebase: self.filebase,
            include_export_name,
            use_key: self.use_key,
            module_source_name: self.module_source_name.clone(),
            separator: self.separator.clone(),
            relative_to: self.relative_to.clone(),
            hash_id: self.hash_id,
            hash_algorithm: self.hash_algorithm.clone(),
        }
    }
}

/// Check if file has supported extension
fn is_supported_file(path: &Path) -> bool {
    if let Some(ext) = path.extension() {
        matches!(
            ext.to_string_lossy().as_ref(),
            "ts" | "tsx" | "js" | "jsx" | "mjs" | "cjs"
        )
    } else {
        false
    }
}

/// Find files matching the glob patterns, excluding ignored patterns
fn find_files(
    include_patterns: &[PathBuf],
    ignore_patterns: &[PathBuf],
) -> Result<HashSet<PathBuf>> {
    let mut files = HashSet::new();

    // Build glob matcher for include patterns
    let mut include_builder = GlobSetBuilder::new();
    let mut base_dirs = Vec::new();

    for glob_path in include_patterns {
        let glob_str = glob_path
            .to_str()
            .context("Invalid UTF-8 in glob pattern")?;

        include_builder.add(Glob::new(glob_str)?);
        base_dirs.push(find_base_dir(glob_str));
    }

    let include_matcher = include_builder.build()?;

    // Build glob matcher for exclude patterns
    let mut exclude_builder = GlobSetBuilder::new();
    for ignore_path in ignore_patterns {
        let ignore_str = ignore_path
            .to_str()
            .context("Invalid UTF-8 in ignore pattern")?;
        exclude_builder.add(Glob::new(ignore_str)?);
    }
    let exclude_matcher = exclude_builder.build()?;

    for base_dir in base_dirs {
        // Walk the directory and match files
        WalkDir::new(&base_dir)
            .into_iter()
            .filter_map(|e| e.ok())
            .for_each(|entry| {
                let path = entry.path();
                if path.is_file()
                    && include_matcher.is_match(path)
                    && !exclude_matcher.is_match(path)
                {
                    // Only process supported file types
                    if is_supported_file(path) {
                        files.insert(path.to_path_buf());
                    }
                }
            });
    }

    Ok(files)
}

/// Find the base directory from a glob pattern
fn find_base_dir(pattern: &str) -> PathBuf {
    // Find the first glob meta-character
    let glob_chars = ['*', '?', '[', '{'];
    let mut base_end = pattern.len();

    for &c in &glob_chars {
        if let Some(pos) = pattern.find(c) {
            // Find the last path separator before the glob character
            if let Some(sep_pos) = pattern[..pos].rfind('/') {
                base_end = sep_pos;
            } else if let Some(sep_pos) = pattern[..pos].rfind('\\') {
                base_end = sep_pos;
            } else {
                base_end = 0;
            }
            break;
        }
    }

    if base_end == 0 {
        PathBuf::from(".")
    } else {
        PathBuf::from(&pattern[..base_end])
    }
}

/// Extract messages from a single file
fn extract_from_file(
    file_path: &Path,
    options: &CoreOptions,
    include_source_location: bool,
) -> Result<Vec<react_intl_core::ExtractedMessage>> {
    let content = fs::read_to_string(file_path)
        .with_context(|| format!("Failed to read file: {}", file_path.display()))?;

    let messages = extract_messages(
        &content,
        file_path.to_string_lossy().as_ref(),
        options,
        include_source_location,
    );

    Ok(messages)
}

/// Determine output mode based on output path
#[derive(Debug)]
enum OutputMode {
    /// Single aggregated JSON file
    Aggregated(PathBuf),
    /// Directory with per-file JSON files
    PerFile(PathBuf),
}

/// Get output mode from output path
fn get_output_mode(output: Option<&str>) -> OutputMode {
    match output {
        None => OutputMode::Aggregated(PathBuf::from("messages.json")),
        Some(path) => {
            let path = PathBuf::from(path);
            // If path ends with / or exists as directory, treat as per-file mode
            if path.to_string_lossy().ends_with('/') || (path.exists() && path.is_dir()) {
                OutputMode::PerFile(path)
            } else {
                OutputMode::Aggregated(path)
            }
        }
    }
}

/// Write messages to output
fn write_output(
    all_messages: Vec<(PathBuf, Vec<react_intl_core::ExtractedMessage>)>,
    mode: OutputMode,
) -> Result<()> {
    match mode {
        OutputMode::Aggregated(output_path) => {
            // Flatten all messages into single array
            let messages: Vec<_> = all_messages
                .into_iter()
                .flat_map(|(_, msgs)| msgs)
                .collect();

            // Ensure parent directory exists
            if let Some(parent) = output_path.parent() {
                fs::create_dir_all(parent)?;
            }

            let json = serde_json::to_string_pretty(&messages)
                .context("Failed to serialize messages to JSON")?;

            fs::write(&output_path, json)
                .with_context(|| format!("Failed to write to {}", output_path.display()))?;

            println!(
                "Extracted {} messages to {}",
                messages.len(),
                output_path.display()
            );
        }
        OutputMode::PerFile(output_dir) => {
            fs::create_dir_all(&output_dir)?;

            let mut total_messages = 0;

            for (file_path, messages) in &all_messages {
                if messages.is_empty() {
                    continue;
                }

                // Calculate relative path for output file
                let relative_path = file_path
                    .file_stem()
                    .and_then(|s| s.to_str())
                    .unwrap_or("unknown");

                let output_file = output_dir.join(format!("{}.json", relative_path));

                // Ensure parent directory exists
                if let Some(parent) = output_file.parent() {
                    fs::create_dir_all(parent)?;
                }

                let json = serde_json::to_string_pretty(&messages)
                    .context("Failed to serialize messages to JSON")?;

                fs::write(&output_file, json)
                    .with_context(|| format!("Failed to write to {}", output_file.display()))?;

                total_messages += messages.len();
            }

            // Also write aggregated file
            let all_msgs: Vec<_> = all_messages
                .iter()
                .flat_map(|(_, msgs)| msgs.clone())
                .collect::<Vec<_>>();

            let aggregated_path = output_dir.join("messages.json");
            let json = serde_json::to_string_pretty(&all_msgs)
                .context("Failed to serialize aggregated messages")?;
            fs::write(&aggregated_path, json)
                .with_context(|| format!("Failed to write to {}", aggregated_path.display()))?;

            println!(
                "Extracted {} messages to {} (directory mode)",
                total_messages,
                output_dir.display()
            );
        }
    }

    Ok(())
}

fn main() -> Result<()> {
    let args = Args::parse();

    println!("React Intl Extract CLI");
    println!("Patterns:",);
    for pattern in &args.patterns {
        println!("  - {}", pattern.to_str().unwrap_or("invalid glob"));
    }

    if !args.ignore.is_empty() {
        println!("Ignore patterns:");
        for ignore in &args.ignore {
            println!("  - {}", ignore.to_str().unwrap_or("invalid glob"));
        }
    }

    if let Some(ref output) = args.output {
        println!("Output: {}", output);
    }

    // Find files
    let files = find_files(&args.patterns, &args.ignore)?;
    println!("Found {} files", files.len());

    if files.is_empty() {
        println!("No files found matching patterns");
        return Ok(());
    }

    // Extract messages from all files
    let core_options = args.to_core_options();
    let include_source_location = args.extract_source_location;
    let mut all_messages: Vec<(PathBuf, Vec<react_intl_core::ExtractedMessage>)> = Vec::new();

    for file in files {
        match extract_from_file(&file, &core_options, include_source_location) {
            Ok(messages) => {
                if !messages.is_empty() {
                    println!("  {} - {} messages", file.display(), messages.len());
                    all_messages.push((file, messages));
                }
            }
            Err(e) => {
                eprintln!("  Warning: Failed to process {}: {}", file.display(), e);
            }
        }
    }

    // Determine output mode and write
    let output_mode = get_output_mode(args.output.as_deref());
    write_output(all_messages, output_mode)?;

    println!("Done!");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_base_dir_file() {
        assert_eq!(
            find_base_dir("./src/components/App.tsx"),
            PathBuf::from("./src/components/App.tsx")
        );
    }

    #[test]
    fn test_find_base_dir_file_alternative() {
        assert_eq!(
            find_base_dir("./src/components/{App,Button}.tsx"),
            PathBuf::from("./src/components")
        );
    }

    #[test]
    fn test_find_base_dir_file_multi_alternative() {
        assert_eq!(
            find_base_dir("./src/{components,layouts}/{App,Button}.tsx"),
            PathBuf::from("./src")
        );
    }

    #[test]
    fn test_find_base_dir_file_mid_alternative() {
        assert_eq!(
            find_base_dir("./src/{components,layouts}/App.tsx"),
            PathBuf::from("./src")
        );
    }

    #[test]
    fn test_find_base_dir_wildcard_file_alternative() {
        assert_eq!(
            find_base_dir("./src/**/{App,Button}.tsx"),
            PathBuf::from("./src")
        );
    }

    #[test]
    fn test_find_base_dir_wildcard() {
        assert_eq!(find_base_dir("./src/**/*.tsx"), PathBuf::from("./src"));
    }

    #[test]
    fn test_find_base_dir_wildcard_ext_alternative() {
        assert_eq!(find_base_dir("./src/**/*.{ts,tsx}"), PathBuf::from("./src"));
    }
}
