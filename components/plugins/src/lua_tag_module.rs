use crate::{
    lua_simple_enum::{LuaAlignment, LuaLinkType},
    lua_tag::LuaTag,
};
use pulldown_cmark::{CodeBlockKind, CowStr, Tag};

#[derive(Debug, Clone, Copy)]
pub struct LuaTagModule;

impl mlua::UserData for LuaTagModule {
    fn add_methods<'lua, M: mlua::UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("paragraph", |_, _, ()| -> mlua::Result<LuaTag> {
            Ok(LuaTag(Tag::Paragraph))
        });

        methods.add_method("heading", |_, _, n: u32| -> mlua::Result<LuaTag> {
            Ok(LuaTag(Tag::Heading(n)))
        });

        methods.add_method("indented_code_block", |_, _, ()| -> mlua::Result<LuaTag> {
            Ok(LuaTag(Tag::CodeBlock(CodeBlockKind::Indented)))
        });

        methods.add_method("fenced_code_block", |_, _, info: String| -> mlua::Result<LuaTag> {
            Ok(LuaTag(Tag::CodeBlock(CodeBlockKind::Fenced(CowStr::Boxed(info.into_boxed_str())))))
        });

        methods.add_method("ordered_list", |_, _, start: u64| -> mlua::Result<LuaTag> {
            Ok(LuaTag(Tag::List(Some(start))))
        });

        methods.add_method("unordered_list", |_, _, ()| -> mlua::Result<LuaTag> {
            Ok(LuaTag(Tag::List(None)))
        });

        methods.add_method("footnote_definition", |_, _, def: String| -> mlua::Result<LuaTag> {
            Ok(LuaTag(Tag::FootnoteDefinition(CowStr::Boxed(def.into_boxed_str()))))
        });

        methods.add_method(
            "table",
            |_, _, alignments: Vec<LuaAlignment>| -> mlua::Result<LuaTag> {
                Ok(LuaTag(Tag::Table(alignments.into_iter().map(|x| x.0).collect())))
            },
        );

        methods.add_method("table_head", |_, _, ()| -> mlua::Result<LuaTag> {
            Ok(LuaTag(Tag::TableHead))
        });

        methods.add_method("table_row", |_, _, ()| -> mlua::Result<LuaTag> {
            Ok(LuaTag(Tag::TableRow))
        });

        methods.add_method("table_cell", |_, _, ()| -> mlua::Result<LuaTag> {
            Ok(LuaTag(Tag::TableCell))
        });

        methods.add_method("emphasis", |_, _, ()| -> mlua::Result<LuaTag> {
            Ok(LuaTag(Tag::Emphasis))
        });

        methods
            .add_method("strong", |_, _, ()| -> mlua::Result<LuaTag> { Ok(LuaTag(Tag::Strong)) });

        methods.add_method("strikethrough", |_, _, ()| -> mlua::Result<LuaTag> {
            Ok(LuaTag(Tag::Strikethrough))
        });

        methods.add_method(
            "link",
            |_,
             _,
             (link_type, destination, title): (LuaLinkType, String, String)|
             -> mlua::Result<LuaTag> {
                Ok(LuaTag(Tag::Link(
                    link_type.0,
                    CowStr::Boxed(destination.into_boxed_str()),
                    CowStr::Boxed(title.into_boxed_str()),
                )))
            },
        );

        methods.add_method(
            "image",
            |_,
             _,
             (link_type, destination, title): (LuaLinkType, String, String)|
             -> mlua::Result<LuaTag> {
                Ok(LuaTag(Tag::Image(
                    link_type.0,
                    CowStr::Boxed(destination.into_boxed_str()),
                    CowStr::Boxed(title.into_boxed_str()),
                )))
            },
        );
    }
}
