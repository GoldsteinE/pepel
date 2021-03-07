use pulldown_cmark::{CowStr, Event};

use crate::lua_event::LuaEvent;
use crate::lua_tag::LuaTag;

#[derive(Debug, Clone, Copy)]
pub struct LuaEventModule;

impl mlua::UserData for LuaEventModule {
    fn add_methods<'lua, M: mlua::UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("tag_start", |_lua, _this, tag: LuaTag| -> mlua::Result<LuaEvent> {
            Ok(LuaEvent(Event::Start(tag.0)))
        });

        methods.add_method("tag_end", |_, _, tag: LuaTag| -> mlua::Result<LuaEvent> {
            Ok(LuaEvent(Event::End(tag.0)))
        });

        methods.add_method("text", |_, _, text: String| -> mlua::Result<LuaEvent> {
            Ok(LuaEvent(Event::Text(CowStr::Boxed(text.into_boxed_str()))))
        });

        methods.add_method("html", |_, _, html: String| -> mlua::Result<LuaEvent> {
            Ok(LuaEvent(Event::Html(CowStr::Boxed(html.into_boxed_str()))))
        });

        methods.add_method("footnote_reference", |_, _, fref: String| -> mlua::Result<LuaEvent> {
            Ok(LuaEvent(Event::FootnoteReference(CowStr::Boxed(fref.into_boxed_str()))))
        });

        methods.add_method("soft_break", |_, _, ()| -> mlua::Result<LuaEvent> {
            Ok(LuaEvent(Event::SoftBreak))
        });

        methods.add_method("hard_break", |_, _, ()| -> mlua::Result<LuaEvent> {
            Ok(LuaEvent(Event::HardBreak))
        });

        methods.add_method("rule", |_, _, ()| -> mlua::Result<LuaEvent> {
            Ok(LuaEvent(Event::Rule))
        });

        methods.add_method("task_list_marker", |_, _, set: bool| -> mlua::Result<LuaEvent> {
            Ok(LuaEvent(Event::TaskListMarker(set)))
        });
    }
}
