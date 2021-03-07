use std::{fmt, fs, io::Read as _, path::Path};

use errors::Result;
use pulldown_cmark::Event;

#[macro_use]
mod macros;
mod lua_event;
mod lua_simple_enum;
mod lua_tag;
mod lua_event_module;
mod lua_tag_module;

use crate::{lua_event::LuaEvent, lua_event_module::LuaEventModule, lua_tag_module::LuaTagModule};

pub struct Plugins {
    lua: mlua::Lua,
    markdown_hooks: Vec<mlua::RegistryKey>,
}

impl Plugins {
    pub fn read_dir<P: AsRef<Path>>(dir: P) -> Result<Self> {
        let lua = mlua::Lua::new();
        lua.globals().set("Event", LuaEventModule)?;
        lua.globals().set("Tag", LuaTagModule)?;

        let mut markdown_hooks = Vec::new();
        let mut buf = Vec::new();
        for entry in dir.as_ref().read_dir()? {
            let entry = entry?;
            let path = entry.path();

            if path.extension() == Some("lua".as_ref()) {
                buf.clear();
                fs::File::open(path)?.read_to_end(&mut buf)?;
                let hook: mlua::Function = lua.load(&buf).call(())?;
                markdown_hooks.push(lua.create_registry_value(hook)?);
            }
        }

        Ok(Self { lua, markdown_hooks })
    }

    pub fn process_event<'a>(&self, ev: Event<'a>) -> Result<impl IntoIterator<Item = Event<'a>>> {
        let mut in_buf = vec![LuaEvent::from(ev)];
        let mut out_buf = Vec::new();

        for hook_key in &self.markdown_hooks {
            let hook: mlua::Function = self.lua.registry_value(hook_key)?;
            for ev in in_buf {
                out_buf.extend(hook.call::<_, Vec<LuaEvent>>(ev)?);
            }

            in_buf = out_buf;
            out_buf = Vec::new();
        }

        Ok(in_buf.into_iter().map(|LuaEvent(ev)| ev))
    }
}

impl fmt::Debug for Plugins {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Plugins").field("hooks_count", &self.markdown_hooks.len()).finish()
    }
}
