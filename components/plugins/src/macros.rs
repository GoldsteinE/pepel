macro_rules! impl_lua_is {
    ($methods:ident => { $($is_method:ident: $variant:pat,)* }) => {
        $(
            $methods.add_method(stringify!($is_method), |_, this, ()| -> mlua::Result<bool> {
                Ok(matches!(this.0, $variant))
            });
        )*
    };
}

macro_rules! impl_lua_unwrap_str {
    ($methods:ident => { $($as_method:ident: $variant:path,)* }) => {
        $(
            // FIXME: is there a way to pass borrowed string?
            $methods.add_method(stringify!($as_method), |_, this, ()| -> mlua::Result<Option<String>> {
                if let $variant(inner) = &this.0 {
                    Ok(Some(inner.as_ref().to_owned()))
                } else {
                    Ok(None)
                }
            });
        )*
    };

    ($methods:ident => { $($as_method:ident: $ident:ident = $variant:path | $upper:pat,)* }) => {
        $(
            // FIXME: is there a way to pass borrowed string?
            $methods.add_method(stringify!($as_method), |_, this, ()| -> mlua::Result<Option<String>> {
                if let $upper = &this {
                    if let $variant(inner) = $ident {
                        Ok(Some(inner.as_ref().to_owned()))
                    } else {
                        Ok(None)
                    }
                } else {
                    Ok(None)
                }
            });
        )*
    };
}

macro_rules! impl_lua_unwrap_value {
    ($methods:ident => { $($as_method:ident -> $type:ty: $variant:path,)* }) => {
        $(
            $methods.add_method(stringify!($as_method), |_, this, ()| -> mlua::Result<Option<$type>> {
                if let $variant(inner) = &this.0 {
                    // FIXME: is there a way to avoid cloning?
                    Ok(Some(inner.clone()))
                } else {
                    Ok(None)
                }
            });
        )*
    }
}
