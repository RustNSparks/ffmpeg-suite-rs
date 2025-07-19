use std::collections::HashMap;
use ffmpeg_common::{StreamSpecifier, StreamType};
use std::fmt;

/// Stream mapping configuration
#[derive(Debug, Clone)]
pub struct StreamMap {
    /// Input file index
    input_index: usize,
    /// Stream specifier
    stream_spec: Option<StreamSpecifier>,
    /// Whether this is a negative mapping (exclude)
    negative: bool,
}

impl StreamMap {
    /// Map all streams from an input
    pub fn from_input(input_index: usize) -> Self {
        Self {
            input_index,
            stream_spec: None,
            negative: false,
        }
    }

    /// Map a specific stream
    pub fn specific(input_index: usize, stream_spec: StreamSpecifier) -> Self {
        Self {
            input_index,
            stream_spec: Some(stream_spec),
            negative: false,
        }
    }

    /// Map video streams from input
    pub fn video_from(input_index: usize) -> Self {
        Self::specific(input_index, StreamSpecifier::Type(StreamType::Video))
    }

    /// Map audio streams from input
    pub fn audio_from(input_index: usize) -> Self {
        Self::specific(input_index, StreamSpecifier::Type(StreamType::Audio))
    }

    /// Map subtitle streams from input
    pub fn subtitle_from(input_index: usize) -> Self {
        Self::specific(input_index, StreamSpecifier::Type(StreamType::Subtitle))
    }

    /// Map a specific stream by index
    pub fn stream_index(input_index: usize, stream_index: usize) -> Self {
        Self::specific(input_index, StreamSpecifier::Index(stream_index))
    }

    /// Exclude this mapping (negative map)
    pub fn exclude(mut self) -> Self {
        self.negative = true;
        self
    }

    /// Convert to command line format
    pub fn to_string(&self) -> String {
        let mut result = String::new();

        if self.negative {
            result.push('-');
        }

        result.push_str(&self.input_index.to_string());

        if let Some(ref spec) = self.stream_spec {
            result.push(':');
            result.push_str(&spec.to_string());
        }

        result
    }
}

impl fmt::Display for StreamMap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

/// Stream selection for processing
#[derive(Debug, Clone)]
pub struct StreamSelection {
    selections: Vec<SelectionRule>,
}

#[derive(Debug, Clone)]
enum SelectionRule {
    All,
    Type(StreamType),
    Index(usize),
    Program(usize),
    Language(String),
    Title(String),
    Metadata { key: String, value: String },
}

impl StreamSelection {
    /// Create a new stream selection
    pub fn new() -> Self {
        Self {
            selections: Vec::new(),
        }
    }

    /// Select all streams
    pub fn all() -> Self {
        Self {
            selections: vec![SelectionRule::All],
        }
    }

    /// Select by type
    pub fn by_type(stream_type: StreamType) -> Self {
        Self {
            selections: vec![SelectionRule::Type(stream_type)],
        }
    }

    /// Select by index
    pub fn by_index(index: usize) -> Self {
        Self {
            selections: vec![SelectionRule::Index(index)],
        }
    }

    /// Select by program
    pub fn by_program(program_id: usize) -> Self {
        Self {
            selections: vec![SelectionRule::Program(program_id)],
        }
    }

    /// Select by language
    pub fn by_language(lang: impl Into<String>) -> Self {
        Self {
            selections: vec![SelectionRule::Language(lang.into())],
        }
    }

    /// Select by title
    pub fn by_title(title: impl Into<String>) -> Self {
        Self {
            selections: vec![SelectionRule::Title(title.into())],
        }
    }

    /// Select by metadata
    pub fn by_metadata(key: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            selections: vec![SelectionRule::Metadata {
                key: key.into(),
                value: value.into(),
            }],
        }
    }

    /// Add another selection rule (OR operation)
    pub fn or(mut self, rule: SelectionRule) -> Self {
        self.selections.push(rule);
        self
    }

    /// Convert to stream maps
    pub fn to_maps(&self, input_index: usize) -> Vec<StreamMap> {
        self.selections
            .iter()
            .map(|rule| match rule {
                SelectionRule::All => StreamMap::from_input(input_index),
                SelectionRule::Type(t) => {
                    StreamMap::specific(input_index, StreamSpecifier::Type(*t))
                }
                SelectionRule::Index(i) => {
                    StreamMap::specific(input_index, StreamSpecifier::Index(*i))
                }
                SelectionRule::Program(p) => {
                    StreamMap::specific(input_index, StreamSpecifier::Program(*p))
                }
                SelectionRule::Language(lang) => StreamMap::specific(
                    input_index,
                    StreamSpecifier::Metadata {
                        key: "language".to_string(),
                        value: Some(lang.clone()),
                    },
                ),
                SelectionRule::Title(title) => StreamMap::specific(
                    input_index,
                    StreamSpecifier::Metadata {
                        key: "title".to_string(),
                        value: Some(title.clone()),
                    },
                ),
                SelectionRule::Metadata { key, value } => StreamMap::specific(
                    input_index,
                    StreamSpecifier::Metadata {
                        key: key.clone(),
                        value: Some(value.clone()),
                    },
                ),
            })
            .collect()
    }
}

impl Default for StreamSelection {
    fn default() -> Self {
        Self::new()
    }
}

/// Stream disposition flags
#[derive(Debug, Clone, Default)]
pub struct StreamDisposition {
    pub default: bool,
    pub dub: bool,
    pub original: bool,
    pub comment: bool,
    pub lyrics: bool,
    pub karaoke: bool,
    pub forced: bool,
    pub hearing_impaired: bool,
    pub visual_impaired: bool,
    pub clean_effects: bool,
    pub attached_pic: bool,
    pub timed_thumbnails: bool,
    pub captions: bool,
    pub descriptions: bool,
    pub metadata: bool,
}

impl StreamDisposition {
    /// Create default disposition
    pub fn new() -> Self {
        Self::default()
    }

    /// Set as default stream
    pub fn set_default(mut self) -> Self {
        self.default = true;
        self
    }

    /// Set as forced stream
    pub fn set_forced(mut self) -> Self {
        self.forced = true;
        self
    }

    /// Build disposition string
    pub fn to_string(&self) -> String {
        let mut parts = Vec::new();

        if self.default {
            parts.push("default");
        }
        if self.dub {
            parts.push("dub");
        }
        if self.original {
            parts.push("original");
        }
        if self.comment {
            parts.push("comment");
        }
        if self.lyrics {
            parts.push("lyrics");
        }
        if self.karaoke {
            parts.push("karaoke");
        }
        if self.forced {
            parts.push("forced");
        }
        if self.hearing_impaired {
            parts.push("hearing_impaired");
        }
        if self.visual_impaired {
            parts.push("visual_impaired");
        }
        if self.clean_effects {
            parts.push("clean_effects");
        }
        if self.attached_pic {
            parts.push("attached_pic");
        }
        if self.timed_thumbnails {
            parts.push("timed_thumbnails");
        }
        if self.captions {
            parts.push("captions");
        }
        if self.descriptions {
            parts.push("descriptions");
        }
        if self.metadata {
            parts.push("metadata");
        }

        parts.join("+")
    }
}

/// Common stream mapping patterns
pub mod patterns {
    use super::*;

    /// Map best quality video and audio
    pub fn best_quality() -> Vec<StreamMap> {
        vec![
            StreamMap::video_from(0),
            StreamMap::audio_from(0),
        ]
    }

    /// Map all video, select specific audio language
    pub fn video_with_language(language: &str) -> Vec<StreamMap> {
        vec![
            StreamMap::video_from(0),
            StreamMap::specific(
                0,
                StreamSpecifier::Metadata {
                    key: "language".to_string(),
                    value: Some(language.to_string()),
                },
            ),
        ]
    }

    /// Map multiple audio tracks
    pub fn multi_audio() -> Vec<StreamMap> {
        vec![
            StreamMap::video_from(0),
            StreamMap::audio_from(0),
        ]
    }

    /// Map for subtitles extraction
    pub fn subtitles_only() -> Vec<StreamMap> {
        vec![StreamMap::subtitle_from(0)]
    }

    /// Map everything except subtitles
    pub fn no_subtitles() -> Vec<StreamMap> {
        vec![
            StreamMap::video_from(0),
            StreamMap::audio_from(0),
        ]
    }

    /// Map specific streams by index
    pub fn by_indices(indices: &[(usize, usize)]) -> Vec<StreamMap> {
        indices
            .iter()
            .map(|(input, stream)| StreamMap::stream_index(*input, *stream))
            .collect()
    }

    /// Complex multi-input mapping
    pub fn multi_input_merge(input_count: usize) -> Vec<StreamMap> {
        let mut maps = vec![StreamMap::video_from(0)];

        for i in 0..input_count {
            maps.push(StreamMap::audio_from(i));
        }

        maps
    }
}

/// Stream metadata builder
#[derive(Debug, Clone, Default)]
pub struct StreamMetadata {
    metadata: HashMap<String, String>,
}

impl StreamMetadata {
    /// Create new stream metadata
    pub fn new() -> Self {
        Self::default()
    }

    /// Set title
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.metadata.insert("title".to_string(), title.into());
        self
    }

    /// Set language
    pub fn language(mut self, lang: impl Into<String>) -> Self {
        self.metadata.insert("language".to_string(), lang.into());
        self
    }

    /// Set handler name
    pub fn handler(mut self, name: impl Into<String>) -> Self {
        self.metadata.insert("handler_name".to_string(), name.into());
        self
    }

    /// Add custom metadata
    pub fn custom(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata.insert(key.into(), value.into());
        self
    }

    /// Get metadata map
    pub fn into_map(self) -> HashMap<String, String> {
        self.metadata
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stream_map() {
        let map = StreamMap::from_input(0);
        assert_eq!(map.to_string(), "0");

        let map = StreamMap::video_from(1);
        assert_eq!(map.to_string(), "1:v");

        let map = StreamMap::stream_index(0, 2).exclude();
        assert_eq!(map.to_string(), "-0:2");
    }

    #[test]
    fn test_stream_selection() {
        let selection = StreamSelection::by_type(StreamType::Audio)
            .or(SelectionRule::Language("eng".to_string()));

        let maps = selection.to_maps(0);
        assert_eq!(maps.len(), 2);
    }

    #[test]
    fn test_stream_disposition() {
        let disp = StreamDisposition::new()
            .set_default()
            .set_forced();

        let s = disp.to_string();
        assert!(s.contains("default"));
        assert!(s.contains("forced"));
    }

    #[test]
    fn test_patterns() {
        let best = patterns::best_quality();
        assert_eq!(best.len(), 2);

        let lang = patterns::video_with_language("eng");
        assert_eq!(lang.len(), 2);
    }
}