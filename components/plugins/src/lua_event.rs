use pulldown_cmark::Event;
use mlua::MetaMethod;

use crate::lua_tag::LuaTag;

/// Wrapper around [`pulldown_cmark::Event`] implementing [`mlua::UserData`]
#[derive(Debug, Clone, PartialEq)]
pub struct LuaEvent(pub Event<'static>);

impl<'a> From<Event<'a>> for LuaEvent {
    fn from(ev: Event<'a>) -> Self {
        Self(match ev {
            Event::Start(tag) => Event::Start(LuaTag::from(tag).into()),
            Event::End(tag) => Event::End(LuaTag::from(tag).into()),
            Event::Text(text) => Event::Text(text.to_string().into()),
            Event::Code(code) => Event::Code(code.to_string().into()),
            Event::Html(html) => Event::Html(html.to_string().into()),
            Event::FootnoteReference(fref) => Event::FootnoteReference(fref.to_string().into()),
            Event::SoftBreak => Event::SoftBreak,
            Event::HardBreak => Event::HardBreak,
            Event::Rule => Event::Rule,
            Event::TaskListMarker(set) => Event::TaskListMarker(set),
        })
    }
}

impl From<LuaEvent> for Event<'static> {
    fn from(this: LuaEvent) -> Self {
        this.0
    }
}

impl mlua::UserData for LuaEvent {
    fn add_methods<'lua, M: mlua::UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_meta_method(MetaMethod::Eq, |_, this, other: LuaEvent| -> mlua::Result<bool> {
            Ok(this == &other)
        });

        impl_lua_is!(methods => {
            is_start_tag: Event::Start(_),
            is_end_tag: Event::End(_),
            is_text: Event::Text(_),
            is_code: Event::Code(_),
            is_html: Event::Html(_),
            is_footnote_reference: Event::FootnoteReference(_),
            is_soft_break: Event::SoftBreak,
            is_hard_break: Event::HardBreak,
            is_rule: Event::Rule,
            is_task_list_marker: Event::TaskListMarker(_), 
        });

        impl_lua_unwrap_str!(methods => {
            as_text: Event::Text,
            as_code: Event::Code,
            as_html: Event::Html,
            as_footnote_reference: Event::FootnoteReference,
        });

        impl_lua_unwrap_value!(methods => {
            as_task_list_marker -> bool: Event::TaskListMarker,
        });

        methods.add_method("as_start_tag", |_, this, ()| -> mlua::Result<Option<LuaTag>> {
            if let Event::Start(tag) = &this.0 {
                Ok(Some(tag.clone().into()))
            } else {
                Ok(None)
            }
        });

        methods.add_method("as_end_tag", |_, this, ()| -> mlua::Result<Option<LuaTag>> {
            if let Event::End(tag) = &this.0 {
                Ok(Some(tag.clone().into()))
            } else {
                Ok(None)
            }
        });
    }
}
