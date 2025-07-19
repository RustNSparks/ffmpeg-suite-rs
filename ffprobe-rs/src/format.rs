use std::fmt;

/// Output format for FFprobe
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutputFormat {
    /// Default format
    Default,
    /// Compact format
    Compact,
    /// CSV format
    Csv,
    /// Flat format
    Flat,
    /// INI format
    Ini,
    /// JSON format
    Json,
    /// XML format
    Xml,
}

impl OutputFormat {
    /// Get the format string for command line
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Default => "default",
            Self::Compact => "compact",
            Self::Csv => "csv",
            Self::Flat => "flat",
            Self::Ini => "ini",
            Self::Json => "json",
            Self::Xml => "xml",
        }
    }

    /// Check if format supports nested data
    pub fn supports_nested(&self) -> bool {
        matches!(self, Self::Json | Self::Xml)
    }

    /// Check if format is human-readable
    pub fn is_human_readable(&self) -> bool {
        matches!(self, Self::Default | Self::Ini)
    }

    /// Check if format is machine-parseable
    pub fn is_machine_parseable(&self) -> bool {
        matches!(self, Self::Json | Self::Xml | Self::Csv | Self::Flat)
    }
}

impl Default for OutputFormat {
    fn default() -> Self {
        Self::Json
    }
}

impl fmt::Display for OutputFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Writer options for output formats
#[derive(Debug, Clone, Default)]
pub struct WriterOptions {
    /// String validation mode
    pub string_validation: Option<StringValidation>,
    /// String validation replacement
    pub string_validation_replacement: Option<String>,
    /// Compact output (JSON)
    pub compact: bool,
    /// Fully qualified output (XML)
    pub fully_qualified: bool,
    /// XSD strict mode (XML)
    pub xsd_strict: bool,
    /// No key output (default, compact)
    pub nokey: bool,
    /// No print wrappers (default)
    pub noprint_wrappers: bool,
    /// Item separator (compact, csv)
    pub item_sep: Option<char>,
    /// Escape mode (compact, csv)
    pub escape: Option<EscapeMode>,
    /// Print section (compact)
    pub print_section: bool,
    /// Separator character (flat)
    pub sep_char: Option<char>,
    /// Hierarchical output (flat, ini)
    pub hierarchical: bool,
}

impl WriterOptions {
    /// Create new writer options
    pub fn new() -> Self {
        Self::default()
    }

    /// Set string validation mode
    pub fn string_validation(mut self, mode: StringValidation) -> Self {
        self.string_validation = Some(mode);
        self
    }

    /// Set string validation replacement
    pub fn string_validation_replacement(mut self, replacement: impl Into<String>) -> Self {
        self.string_validation_replacement = Some(replacement.into());
        self
    }

    /// Enable compact output
    pub fn compact(mut self, enable: bool) -> Self {
        self.compact = enable;
        self
    }

    /// Enable fully qualified output
    pub fn fully_qualified(mut self, enable: bool) -> Self {
        self.fully_qualified = enable;
        self
    }

    /// Enable XSD strict mode
    pub fn xsd_strict(mut self, enable: bool) -> Self {
        self.xsd_strict = enable;
        if enable {
            self.fully_qualified = true;
        }
        self
    }

    /// Set no key output
    pub fn nokey(mut self, enable: bool) -> Self {
        self.nokey = enable;
        self
    }

    /// Set no print wrappers
    pub fn noprint_wrappers(mut self, enable: bool) -> Self {
        self.noprint_wrappers = enable;
        self
    }

    /// Set item separator
    pub fn item_sep(mut self, sep: char) -> Self {
        self.item_sep = Some(sep);
        self
    }

    /// Set escape mode
    pub fn escape(mut self, mode: EscapeMode) -> Self {
        self.escape = Some(mode);
        self
    }

    /// Set print section
    pub fn print_section(mut self, enable: bool) -> Self {
        self.print_section = enable;
        self
    }

    /// Set separator character
    pub fn sep_char(mut self, sep: char) -> Self {
        self.sep_char = Some(sep);
        self
    }

    /// Set hierarchical output
    pub fn hierarchical(mut self, enable: bool) -> Self {
        self.hierarchical = enable;
        self
    }

    /// Build command line arguments for writer options
    pub fn build_args(&self, format: OutputFormat) -> Vec<String> {
        let mut args = Vec::new();
        let prefix = format.as_str();

        // Common options
        if let Some(ref validation) = self.string_validation {
            args.push(format!("{}:string_validation={}", prefix, validation.as_str()));
        }

        if let Some(ref replacement) = self.string_validation_replacement {
            args.push(format!("{}:string_validation_replacement={}", prefix, replacement));
        }

        // Format-specific options
        match format {
            OutputFormat::Json => {
                if self.compact {
                    args.push(format!("{}:compact=1", prefix));
                }
            }
            OutputFormat::Xml => {
                if self.fully_qualified {
                    args.push(format!("{}:fully_qualified=1", prefix));
                }
                if self.xsd_strict {
                    args.push(format!("{}:xsd_strict=1", prefix));
                }
            }
            OutputFormat::Default => {
                if self.nokey {
                    args.push(format!("{}:nokey=1", prefix));
                }
                if self.noprint_wrappers {
                    args.push(format!("{}:noprint_wrappers=1", prefix));
                }
            }
            OutputFormat::Compact | OutputFormat::Csv => {
                if self.nokey {
                    args.push(format!("{}:nokey=1", prefix));
                }
                if let Some(sep) = self.item_sep {
                    args.push(format!("{}:item_sep={}", prefix, sep));
                }
                if let Some(ref escape) = self.escape {
                    args.push(format!("{}:escape={}", prefix, escape.as_str()));
                }
                if self.print_section {
                    args.push(format!("{}:print_section=1", prefix));
                }
            }
            OutputFormat::Flat => {
                if let Some(sep) = self.sep_char {
                    args.push(format!("{}:sep_char={}", prefix, sep));
                }
                if self.hierarchical {
                    args.push(format!("{}:hierarchical=1", prefix));
                }
            }
            OutputFormat::Ini => {
                if self.hierarchical {
                    args.push(format!("{}:hierarchical=1", prefix));
                }
            }
        }

        args
    }
}

/// String validation mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StringValidation {
    /// Fail on invalid strings
    Fail,
    /// Ignore invalid strings
    Ignore,
    /// Replace invalid strings
    Replace,
}

impl StringValidation {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Fail => "fail",
            Self::Ignore => "ignore",
            Self::Replace => "replace",
        }
    }
}

/// Escape mode for compact/csv formats
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EscapeMode {
    /// C-style escaping
    C,
    /// CSV-style escaping
    Csv,
    /// No escaping
    None,
}

impl EscapeMode {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::C => "c",
            Self::Csv => "csv",
            Self::None => "none",
        }
    }
}

/// Format presets for common use cases
pub mod presets {
    use super::*;

    /// JSON format for API consumption
    pub fn json_api() -> (OutputFormat, WriterOptions) {
        (
            OutputFormat::Json,
            WriterOptions::new()
                .compact(true)
                .string_validation(StringValidation::Replace)
                .string_validation_replacement(""),
        )
    }

    /// JSON format for human reading
    pub fn json_pretty() -> (OutputFormat, WriterOptions) {
        (
            OutputFormat::Json,
            WriterOptions::new()
                .compact(false)
                .string_validation(StringValidation::Replace),
        )
    }

    /// XML for validation
    pub fn xml_strict() -> (OutputFormat, WriterOptions) {
        (
            OutputFormat::Xml,
            WriterOptions::new()
                .xsd_strict(true)
                .string_validation(StringValidation::Fail),
        )
    }

    /// CSV for spreadsheet import
    pub fn csv_excel() -> (OutputFormat, WriterOptions) {
        (
            OutputFormat::Csv,
            WriterOptions::new()
                .item_sep(',')
                .escape(EscapeMode::Csv)
                .nokey(true),
        )
    }

    /// Flat format for shell scripts
    pub fn flat_shell() -> (OutputFormat, WriterOptions) {
        (
            OutputFormat::Flat,
            WriterOptions::new()
                .sep_char('_')
                .hierarchical(true),
        )
    }

    /// INI format for configuration
    pub fn ini_config() -> (OutputFormat, WriterOptions) {
        (
            OutputFormat::Ini,
            WriterOptions::new().hierarchical(true),
        )
    }

    /// Compact format for logging
    pub fn compact_log() -> (OutputFormat, WriterOptions) {
        (
            OutputFormat::Compact,
            WriterOptions::new()
                .item_sep('|')
                .print_section(true)
                .escape(EscapeMode::C),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_output_format() {
        assert_eq!(OutputFormat::Json.as_str(), "json");
        assert!(OutputFormat::Json.supports_nested());
        assert!(OutputFormat::Json.is_machine_parseable());
        assert!(!OutputFormat::Json.is_human_readable());
    }

    #[test]
    fn test_writer_options() {
        let opts = WriterOptions::new()
            .compact(true)
            .string_validation(StringValidation::Replace)
            .string_validation_replacement("?");

        let args = opts.build_args(OutputFormat::Json);
        assert!(args.iter().any(|arg| arg.contains("compact=1")));
        assert!(args.iter().any(|arg| arg.contains("string_validation=replace")));
    }

    #[test]
    fn test_presets() {
        let (format, opts) = presets::json_api();
        assert_eq!(format, OutputFormat::Json);
        assert!(opts.compact);

        let (format, opts) = presets::xml_strict();
        assert_eq!(format, OutputFormat::Xml);
        assert!(opts.xsd_strict);
        assert!(opts.fully_qualified);
    }
}