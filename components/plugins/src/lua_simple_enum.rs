use pulldown_cmark::{Alignment, LinkType};

pub(crate) trait SimpleEnum: Sized {
    const ENUM_NAME: &'static str;

    fn tag_str(&self) -> &'static str;
    fn from_tag_str(s: &str) -> Option<Self>;
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
pub(crate) struct LuaSimpleEnum<T>(pub T);

pub(crate) type LuaAlignment = LuaSimpleEnum<Alignment>;
pub(crate) type LuaLinkType = LuaSimpleEnum<LinkType>;

impl<'lua, T: SimpleEnum> mlua::ToLua<'lua> for LuaSimpleEnum<T> {
    fn to_lua(self, lua: &'lua mlua::Lua) -> mlua::Result<mlua::Value<'lua>> {
        self.0.tag_str().to_lua(lua)
    }
}

impl<'lua, T: SimpleEnum> mlua::FromLua<'lua> for LuaSimpleEnum<T> {
    fn from_lua(lua_value: mlua::Value<'lua>, _lua: &'lua mlua::Lua) -> mlua::Result<Self> {
        if let mlua::Value::String(s) = &lua_value {
            if let Some(this) = T::from_tag_str(s.to_str()?) {
                Ok(Self(this))
            } else {
                Err(mlua::Error::FromLuaConversionError {
                    from: lua_value.type_name(),
                    to: T::ENUM_NAME,
                    message: Some(format!("invalid variant `{}`", String::from_utf8_lossy(s.as_bytes()))),
                })
            }
        } else {
            Err(mlua::Error::FromLuaConversionError {
                from: lua_value.type_name(),
                to: T::ENUM_NAME,
                message: Some("expected string".into()),
            })
        }
    }
}

macro_rules! impl_lua_simple_enum {
    ($($t:ty => {$($variant:ident),*},)*) => {
        $(
            impl SimpleEnum for $t {
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
