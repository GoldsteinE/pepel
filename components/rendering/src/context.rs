use std::borrow::Cow;
use std::collections::HashMap;

use config::Config;
use plugins::Plugins;
use tera::{Context, Tera};

/// All the information from the zola site that is needed to render HTML from markdown
#[derive(Debug)]
pub struct RenderContext<'a> {
    pub tera: Cow<'a, Tera>,
    pub config: &'a Config,
    pub plugins: Option<&'a Plugins>,
    pub tera_context: Context,
    pub current_page_permalink: &'a str,
    pub permalinks: Cow<'a, HashMap<String, String>>,
}

impl<'a> RenderContext<'a> {
    pub fn new(
        tera: &'a Tera,
        config: &'a Config,
        plugins: Option<&'a Plugins>,
        current_page_permalink: &'a str,
        permalinks: &'a HashMap<String, String>,
    ) -> RenderContext<'a> {
        let mut tera_context = Context::new();
        tera_context.insert("config", config);
        Self {
            tera: Cow::Borrowed(tera),
            tera_context,
            current_page_permalink,
            permalinks: Cow::Borrowed(permalinks),
            config,
            plugins,
        }
    }

    // In use in the markdown filter
    pub fn from_config(config: &'a Config) -> RenderContext<'a> {
        Self {
            tera: Cow::Owned(Tera::default()),
            tera_context: Context::new(),
            current_page_permalink: "",
            permalinks: Cow::Owned(HashMap::new()),
            plugins: None,
            config,
        }
    }
}
