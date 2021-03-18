use std::path::PathBuf;

use serde_derive::{Deserialize, Serialize};
use syntect::parsing::SyntaxSet;

pub const DEFAULT_HIGHLIGHT_THEME: &str = "base16-ocean-dark";

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "highlight_type")]
pub enum HighlighterSettings {
    /// Produce "span with styles" highlighting
    Inline {
        /// Which themes to use for code highlighting. See Readme for supported themes
        /// Defaults to "base16-ocean-dark"
        #[serde(default = "inline_highlight_theme_default")]
        highlight_theme: String,
    },
    /// Produce "span with classes" highlighting
    Classed {
        /// Which themes to use for code highlighting. See Readme for supported themes
        /// Defaults to ["base16-ocean-dark"]
        #[serde(default = "classed_highlight_theme_default")]
        highlight_theme: Vec<String>,
        /// Where to put theme CSS files
        #[serde(default = "default_themes_path")]
        themes_path: PathBuf,
    },
    /// Do not highlight code
    #[serde(other)]
    None,
}

fn inline_highlight_theme_default() -> String {
    DEFAULT_HIGHLIGHT_THEME.to_owned()
}

fn classed_highlight_theme_default() -> Vec<String> {
    vec![DEFAULT_HIGHLIGHT_THEME.to_owned()]
}

fn default_themes_path() -> PathBuf {
    "themes".into()
}

impl Default for HighlighterSettings {
    fn default() -> Self {
        Self::None
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct Markdown {
    #[serde(flatten)]
    pub highlighter: HighlighterSettings,
    /// Whether to render emoji aliases (e.g.: :smile: => 😄) in the markdown files
    pub render_emoji: bool,
    /// Whether external links are to be opened in a new tab
    /// If this is true, a `rel="noopener"` will always automatically be added for security reasons
    pub external_links_target_blank: bool,
    /// Whether to set rel="nofollow" for all external links
    pub external_links_no_follow: bool,
    /// Whether to set rel="noreferrer" for all external links
    pub external_links_no_referrer: bool,
    /// Whether smart punctuation is enabled (changing quotes, dashes, dots etc in their typographic form)
    pub smart_punctuation: bool,

    /// A list of directories to search for additional `.sublime-syntax` files in.
    pub extra_syntaxes: Vec<String>,
    /// The compiled extra syntaxes into a syntax set
    #[serde(skip_serializing, skip_deserializing)] // not a typo, 2 are need
    pub extra_syntax_set: Option<SyntaxSet>,
}

impl Markdown {
    pub fn has_external_link_tweaks(&self) -> bool {
        self.external_links_target_blank
            || self.external_links_no_follow
            || self.external_links_no_referrer
    }

    pub fn construct_external_link_tag(&self, url: &str, title: &str) -> String {
        let mut rel_opts = Vec::new();
        let mut target = "".to_owned();
        let title = if title == "" { "".to_owned() } else { format!("title=\"{}\" ", title) };

        if self.external_links_target_blank {
            // Security risk otherwise
            rel_opts.push("noopener");
            target = "target=\"_blank\" ".to_owned();
        }
        if self.external_links_no_follow {
            rel_opts.push("nofollow");
        }
        if self.external_links_no_referrer {
            rel_opts.push("noreferrer");
        }
        let rel = if rel_opts.is_empty() {
            "".to_owned()
        } else {
            format!("rel=\"{}\" ", rel_opts.join(" "))
        };

        format!("<a {}{}{}href=\"{}\">", rel, target, title, url)
    }
}

impl Default for Markdown {
    fn default() -> Markdown {
        Markdown {
            highlighter: HighlighterSettings::default(),
            render_emoji: false,
            external_links_target_blank: false,
            external_links_no_follow: false,
            external_links_no_referrer: false,
            smart_punctuation: false,
            extra_syntaxes: vec![],
            extra_syntax_set: None,
        }
    }
}
