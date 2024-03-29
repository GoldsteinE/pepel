pub mod filters;
pub mod global_fns;

use lazy_static::lazy_static;
use tera::{Context, Tera};

use errors::{Error, Result};

lazy_static! {
    pub static ref PEPEL_TERA: Tera = {
        let mut tera = Tera::default();
        tera.add_raw_templates(vec![
            ("__pepel_builtins/404.html", include_str!("builtins/404.html")),
            ("__pepel_builtins/atom.xml", include_str!("builtins/atom.xml")),
            ("__pepel_builtins/rss.xml", include_str!("builtins/rss.xml")),
            ("__pepel_builtins/sitemap.xml", include_str!("builtins/sitemap.xml")),
            ("__pepel_builtins/robots.txt", include_str!("builtins/robots.txt")),
            (
                "__pepel_builtins/split_sitemap_index.xml",
                include_str!("builtins/split_sitemap_index.xml"),
            ),
            ("__pepel_builtins/anchor-link.html", include_str!("builtins/anchor-link.html")),
            (
                "__pepel_builtins/shortcodes/youtube.html",
                include_str!("builtins/shortcodes/youtube.html"),
            ),
            (
                "__pepel_builtins/shortcodes/vimeo.html",
                include_str!("builtins/shortcodes/vimeo.html"),
            ),
            ("__pepel_builtins/shortcodes/gist.html", include_str!("builtins/shortcodes/gist.html")),
            (
                "__pepel_builtins/shortcodes/streamable.html",
                include_str!("builtins/shortcodes/streamable.html"),
            ),
            ("internal/alias.html", include_str!("builtins/internal/alias.html")),
        ])
        .unwrap();
        tera.register_filter("base64_encode", filters::base64_encode);
        tera.register_filter("base64_decode", filters::base64_decode);
        tera
    };
}

/// Renders the `internal/alias.html` template that will redirect
/// via refresh to the url given
pub fn render_redirect_template(url: &str, tera: &Tera) -> Result<String> {
    let mut context = Context::new();
    context.insert("url", &url);

    tera.render("internal/alias.html", &context)
        .map_err(|e| Error::chain(format!("Failed to render alias for '{}'", url), e))
}
