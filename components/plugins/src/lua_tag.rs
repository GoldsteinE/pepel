use pulldown_cmark::{CodeBlockKind, LinkType, Tag};

use crate::lua_simple_enum::LuaSimpleEnum;

/// Lua representation of [`pulldown_cmark::Tag::Link`] and [`pulldown_cmark::Tag::Image`]
#[derive(Debug, Clone)]
pub struct LuaLink {
    link_type: LinkType,
    destination: String,
    title: String,
}

impl mlua::UserData for LuaLink {
    fn add_methods<'lua, M: mlua::UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("type", |_, this, ()| -> mlua::Result<&'static str> {
            Ok(this.link_type.tag_str())
        });

        methods.add_method("destination", |_, this, ()| -> mlua::Result<String> {
            Ok(this.destination.clone())
        });

        methods
            .add_method("title", |_, this, ()| -> mlua::Result<String> { Ok(this.title.clone()) });
    }
}

/// Wrapper around [`pulldown_cmark::Tag`] implementing [`mlua::UserData`]
#[derive(Debug, Clone)]
pub struct LuaTag(pub Tag<'static>);

impl<'a> From<Tag<'a>> for LuaTag {
    fn from(tag: Tag<'a>) -> Self {
        Self(match tag {
            Tag::Paragraph => Tag::Paragraph,
            Tag::Heading(n) => Tag::Heading(n),
            Tag::BlockQuote => Tag::BlockQuote,
            Tag::CodeBlock(kind) => Tag::CodeBlock(match kind {
                CodeBlockKind::Indented => CodeBlockKind::Indented,
                CodeBlockKind::Fenced(info) => CodeBlockKind::Fenced(info.into_string().into()),
            }),
            Tag::List(start) => Tag::List(start),
            Tag::Item => Tag::Item,
            Tag::FootnoteDefinition(def) => Tag::FootnoteDefinition(def.into_string().into()),
            Tag::Table(alignments) => Tag::Table(alignments),
            Tag::TableHead => Tag::TableHead,
            Tag::TableRow => Tag::TableRow,
            Tag::TableCell => Tag::TableCell,
            Tag::Emphasis => Tag::Emphasis,
            Tag::Strong => Tag::Strong,
            Tag::Strikethrough => Tag::Strikethrough,
            Tag::Link(link_type, destination, title) => {
                Tag::Link(link_type, destination.into_string().into(), title.into_string().into())
            }
            Tag::Image(link_type, destination, title) => {
                Tag::Image(link_type, destination.into_string().into(), title.into_string().into())
            }
        })
    }
}

impl From<LuaTag> for Tag<'static> {
    fn from(this: LuaTag) -> Self {
        this.0
    }
}

impl mlua::UserData for LuaTag {
    fn add_methods<'lua, M: mlua::UserDataMethods<'lua, Self>>(methods: &mut M) {
        impl_lua_is!(methods => {
            is_paragraph: Tag::Paragraph,
            is_heading: Tag::Heading(_),
            is_blockquote: Tag::BlockQuote,
            is_code_block: Tag::CodeBlock(_),
            is_list: Tag::List(_),
            is_item: Tag::Item,
            is_footnote_definition: Tag::FootnoteDefinition(_),
            is_table: Tag::Table(_),
            is_table_head: Tag::TableHead,
            is_table_row: Tag::TableRow,
            is_table_cell: Tag::TableCell,
            is_emphasis: Tag::Emphasis,
            is_strong: Tag::Strong,
            is_strikethrough: Tag::Strikethrough,
            is_link: Tag::Link(_, _, _),
            is_image: Tag::Image(_, _, _),
        });

        impl_lua_unwrap_value!(methods => {
            as_heading -> u32: Tag::Heading,
            as_list_start -> Option<u64>: Tag::List,
        });

        impl_lua_unwrap_str!(methods => {
            as_footnote_definition: Tag::FootnoteDefinition,
        });

        methods.add_method("as_info_string", |_, this, ()| -> mlua::Result<Option<String>> {
            if let Tag::CodeBlock(CodeBlockKind::Fenced(info_string)) = &this.0 {
                Ok(Some(info_string.as_ref().to_owned()))
            } else {
                Ok(None)
            }
        });

        methods.add_method(
            "as_table_alignments",
            |_, this, ()| -> mlua::Result<Option<Vec<&'static str>>> {
                if let Tag::Table(alignments) = &this.0 {
                    Ok(Some(alignments.iter().map(|alignment| alignment.tag_str()).collect()))
                } else {
                    Ok(None)
                }
            },
        );

        methods.add_method("as_link", |_, this, ()| -> mlua::Result<Option<LuaLink>> {
            if let Tag::Link(link_type, destination, title) = &this.0 {
                Ok(Some(LuaLink {
                    link_type: *link_type,
                    destination: destination.as_ref().to_owned(),
                    title: title.as_ref().to_owned(),
                }))
            } else {
                Ok(None)
            }
        });

        methods.add_method("as_image", |_, this, ()| -> mlua::Result<Option<LuaLink>> {
            if let Tag::Image(link_type, destination, title) = &this.0 {
                Ok(Some(LuaLink {
                    link_type: *link_type,
                    destination: destination.as_ref().to_owned(),
                    title: title.as_ref().to_owned(),
                }))
            } else {
                Ok(None)
            }
        });
    }
}
