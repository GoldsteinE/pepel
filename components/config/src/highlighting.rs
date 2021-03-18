use lazy_static::lazy_static;
use syntect::parsing::SyntaxReference;
use syntect::parsing::SyntaxSet;
use syntect::{dumps::from_binary, html::ClassedHTMLGenerator};
use syntect::{highlighting::ThemeSet, html::ClassStyle, util::LinesWithEndings};

use crate::config::{markup::HighlighterSettings, Config};

lazy_static! {
    pub static ref SYNTAX_SET: SyntaxSet = {
        let ss: SyntaxSet =
            from_binary(include_bytes!("../../../sublime/syntaxes/newlines.packdump"));
        ss
    };
    pub static ref THEME_SET: ThemeSet =
        from_binary(include_bytes!("../../../sublime/themes/all.themedump"));
}

// FIXME: unneccesary move
fn wrap_pre_code(language: Option<&str>, mut code: String) -> String {
    let lang_prefix;
    let prefix = match language {
        Some(language) => {
            lang_prefix = format!(r#"<pre><code class="language-{0}" data-lang="{0}">"#, language);
            lang_prefix.as_str()
        }
        None => r#"<pre><code>"#,
    };
    code.insert_str(0, prefix);
    code.push_str("</code></pre>");
    code
}

pub fn highlight_code(config: &Config, language: Option<&str>, code: String) -> String {
    let (syntax, syntax_set) = find_syntax(language, config);
    match &config.markdown.highlighter {
        HighlighterSettings::Inline { highlight_theme } => {
            let theme_set: &ThemeSet = &*THEME_SET;
            syntect::html::highlighted_html_for_string(
                &code,
                syntax_set,
                syntax,
                &theme_set.themes[highlight_theme],
            )
        }
        HighlighterSettings::Classed { .. } => {
            let mut hl = ClassedHTMLGenerator::new_with_class_style(
                syntax,
                syntax_set,
                ClassStyle::SpacedPrefixed { prefix: "zola-hl-" },
            );

            for line in LinesWithEndings::from(&code) {
                hl.parse_html_for_line_which_includes_newline(line);
            }

            wrap_pre_code(language, hl.finalize())
        }
        HighlighterSettings::None => wrap_pre_code(language, code),
    }
}

fn find_syntax<'a>(lang: Option<&str>, config: &'a Config) -> (&'a SyntaxReference, &'a SyntaxSet) {
    if let Some(ref lang) = lang {
        // The JS syntax hangs a lot... the TS syntax is probably better anyway.
        // https://github.com/getzola/zola/issues/1241
        // https://github.com/getzola/zola/issues/1211
        // https://github.com/getzola/zola/issues/1174
        let lang = if *lang == "js" || *lang == "javascript" { "ts" } else { lang };

        if let Some(ref extra) = config.markdown.extra_syntax_set {
            let syntax = extra.find_syntax_by_token(lang);
            if let Some(syntax) = syntax {
                return (syntax, extra);
            }
        }

        if let Some(syntax) = SYNTAX_SET.find_syntax_by_token(lang) {
            return (syntax, &*SYNTAX_SET);
        }
    }

    (SYNTAX_SET.find_syntax_plain_text(), &*SYNTAX_SET)
}
