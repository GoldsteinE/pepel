use pulldown_cmark::{Alignment, LinkType};

pub(crate) trait LuaSimpleEnum: Sized {
    const ENUM_NAME: &'static str;

    fn tag_str(&self) -> &'static str;
    fn from_tag_str(s: &str) -> Option<Self>;
}

/*
impl<T: LuaSimpleEnum> mlua::ToLua for T {
    fn to_lua(self, lua: &'lua mlua::Lua) -> mlua::Result<mlua::Value<'lua>> {
        self.tag_str().to_lua()
    }
}

impl<T: LuaSimpleEnum> mlua::FromLua for T {
    fn from_lua(lua_value: mlua::Value<'lua>, lua: &'lua mlua::Lua) -> mlua::Result<Self> {
        if let mlua::Value::String(s) = lua_value {
            if let Some(this) = Self::from_tag_str(s.to_str()?) {
                Ok(this)
            } else {
                Err(mlua::Error::FromLuaConversionError {
                    from: lua_value.type_name(),
                    to: Self::ENUM_NAME,
                    message: Some(format!("invalid variant `{}`", s)),
                })
            }
        } else {
            Err(mlua::Error::FromLuaConversionError {
                from: lua_value.type_name(),
                to: Self::ENUM_NAME,
                message: Some("expected string".into()),
            })
        }
    }
}
*/

macro_rules! impl_lua_simple_enum {
    ($($t:ty => {$($variant:ident),*},)*) => {
        $(
            impl LuaSimpleEnum for $t {
                const ENUM_NAME: &'static str = stringify!($ty);

                fn tag_str(&self) -> &'static str {
                    match self {
                        $(
                            Self::$variant => stringify!($variant),
                        )*
                    }
                }

                fn from_tag_str(s: &str) -> Option<Self> {
                    match s {
                        $(
                            stringify!($variant) => Some(Self::$variant),
                        )*
                        _ => None
                    }
                }
            }
        )*
    }
}

impl_lua_simple_enum! {
    Alignment => {None, Left, Center, Right},
    LinkType => {
        Inline,
        Reference, ReferenceUnknown,
        Collapsed, CollapsedUnknown,
        Shortcut, ShortcutUnknown,
        Autolink,
        Email
    },
}
