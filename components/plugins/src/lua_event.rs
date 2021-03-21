use mlua::MetaMethod;
use pulldown_cmark::Event;

use crate::lua_tag::LuaTag;

#[derive(Debug, Clone, PartialEq)]
pub enum LuaEventKind {
    Markdown(Event<'static>),
    ContentStart,
    ContentEnd,
}

/// Wrapper around [`pulldown_cmark::Event`] implementing [`mlua::UserData`]
#[derive(Debug, Clone, PartialEq)]
pub struct LuaEvent(pub LuaEventKind);

impl<'a> From<Event<'a>> for LuaEvent {
    fn from(ev: Event<'a>) -> Self {
        Self(LuaEventKind::Markdown(match ev {
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
        }))
    }
}

impl From<LuaEvent> for Option<Event<'static>> {
    fn from(this: LuaEvent) -> Self {
        match this.0 {
            LuaEventKind::Markdown(event) => Some(event),
            LuaEventKind::ContentStart | LuaEventKind::ContentEnd => None,
        }
    }
}

impl mlua::UserData for LuaEvent {
    fn add_methods<'lua, M: mlua::UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_meta_method(MetaMethod::Eq, |_, this, other: LuaEvent| -> mlua::Result<bool> {
            Ok(this == &other)
        });

        impl_lua_is!(methods => {
            is_start_tag: LuaEventKind::Markdown(Event::Start(_)),
            is_end_tag: LuaEventKind::Markdown(Event::End(_)),
            is_text: LuaEventKind::Markdown(Event::Text(_)),
            is_code: LuaEventKind::Markdown(Event::Code(_)),
            is_html: LuaEventKind::Markdown(Event::Html(_)),
            is_footnote_reference: LuaEventKind::Markdown(Event::FootnoteReference(_)),
            is_soft_break: LuaEventKind::Markdown(Event::SoftBreak),
            is_hard_break: LuaEventKind::Markdown(Event::HardBreak),
            is_rule: LuaEventKind::Markdown(Event::Rule),
            is_task_list_marker: LuaEventKind::Markdown(Event::TaskListMarker(_)),
            is_content_start: LuaEventKind::ContentStart,
            is_content_end: LuaEventKind::ContentEnd,
        });

        impl_lua_unwrap_str!(methods => {
            as_text: inner = Event::Text | LuaEvent(LuaEventKind::Markdown(inner)),
            as_code: inner = Event::Code | LuaEvent(LuaEventKind::Markdown(inner)),
            as_html: inner = Event::Html | LuaEvent(LuaEventKind::Markdown(inner)),
            as_footnote_reference: inner = Event::FootnoteReference | LuaEvent(LuaEventKind::Markdown(inner)),
        });

        methods.add_method("as_task_list_marker", |_, this, ()| -> mlua::Result<Option<bool>> {
            if let LuaEventKind::Markdown(Event::TaskListMarker(checked)) = &this.0 {
                Ok(Some(*checked))
            } else {
                Ok(None)
            }
        });

        methods.add_method("as_start_tag", |_, this, ()| -> mlua::Result<Option<LuaTag>> {
            if let LuaEventKind::Markdown(Event::Start(tag)) = &this.0 {
                Ok(Some(tag.clone().into()))
            } else {
                Ok(None)
            }
        });

        methods.add_method("as_end_tag", |_, this, ()| -> mlua::Result<Option<LuaTag>> {
            if let LuaEventKind::Markdown(Event::End(tag)) = &this.0 {
                Ok(Some(tag.clone().into()))
            } else {
                Ok(None)
            }
        });
    }
}
