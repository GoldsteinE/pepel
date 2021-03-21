use std::{fmt, fs, io::Read as _, path::Path};

use errors::Result;
use pulldown_cmark::Event;

#[macro_use]
mod macros;
mod lua_event;
mod lua_event_module;
mod lua_simple_enum;
mod lua_tag;
mod lua_tag_module;

use crate::{lua_event::{LuaEvent, LuaEventKind}, lua_event_module::LuaEventModule, lua_tag_module::LuaTagModule};

pub struct Plugins {
    lua: mlua::Lua,
    markdown_hooks: Vec<mlua::RegistryKey>,
}

impl Plugins {
    pub fn load<P: AsRef<Path>, N: AsRef<Path>>(dir: P, plugins: &[N]) -> Result<Self> {
        let lua = mlua::Lua::new();
        lua.globals().set("Event", LuaEventModule)?;
        lua.globals().set("Tag", LuaTagModule)?;

        let dir = dir.as_ref();
        let mut markdown_hooks = Vec::new();
        let mut buf = Vec::new();
        for plugin in plugins {
            let mut path = dir.join(plugin);
            path.set_extension("lua");

            buf.clear();
            fs::File::open(path)?.read_to_end(&mut buf)?;
            let hook: mlua::Function = lua.load(&buf).call(())?;
            markdown_hooks.push(lua.create_registry_value(hook)?);
        }

        Ok(Self { lua, markdown_hooks })
    }

    fn process_lua_event(&self, ev: LuaEvent) -> Result<impl IntoIterator<Item = Event<'static>>> {
        let mut in_buf = vec![ev];
        let mut out_buf = Vec::new();

        for hook_key in &self.markdown_hooks {
            let hook: mlua::Function = self.lua.registry_value(hook_key)?;
            for ev in in_buf {
                out_buf.extend(hook.call::<_, Vec<LuaEvent>>(ev)?);
            }

            in_buf = out_buf;
            out_buf = Vec::new();
        }

        Ok(in_buf.into_iter().filter_map(|ev| ev.into()))
    }

    pub fn process_event<'a>(&self, ev: Event<'a>) -> Result<impl IntoIterator<Item = Event<'static>>> {
        self.process_lua_event(ev.into())
    }

    pub fn content_start(&self) -> Result<impl IntoIterator<Item = Event<'static>>> {
        self.process_lua_event(LuaEvent(LuaEventKind::ContentStart))
    }

    pub fn content_end(&self) -> Result<impl IntoIterator<Item = Event<'static>>> {
        self.process_lua_event(LuaEvent(LuaEventKind::ContentEnd))
    }
}

impl fmt::Debug for Plugins {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Plugins").field("hooks_count", &self.markdown_hooks.len()).finish()
    }
}
