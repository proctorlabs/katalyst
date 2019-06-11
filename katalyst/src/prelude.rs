/*!
This module provides a "prelude" useful for extending Katalyst functionality
*/

pub use crate::{
    app::Katalyst,
    context::*,
    error::{GatewayError::*, *},
    expression::*,
    modules::*,
};
pub(crate) use crate::{parser::*, *};
pub(crate) use futures::prelude::*;
pub(crate) use std::sync::Arc;

macro_rules! req {
    ($item:expr, $enum:ident :: $name:ident) => {
        match $item {
            $enum::$name(mtch) => mtch,
            _ => return Err(RequiredComponent { name: stringify!($enum::$name).to_string() }),
        }
    };
    (:$item:expr, $enum:ident :: $name:ident) => {
        match $item {
            $enum::$name(mtch) => mtch,
            _ => {
                return Box::new(err(RequiredComponent {
                    name: stringify!($enum::$name).to_string(),
                }))
            }
        }
    };
    ($name:expr) => {
        if let Some(mtch) = $name {
            mtch
        } else {
            return Err(err!(
                RequiredComponent,
                format!("Required component {} not found!", stringify!($name)),
                name: stringify!($name).to_string()
            ));
        }
    };
    (:$name:expr) => {
        if let Some(mtch) = $name {
            mtch
        } else {
            return Box::new(err(err!(
                RequiredComponent,
                format!("Required component {} not found!", stringify!($name)),
                name: stringify!($name).to_string()
            )));
        }
    };
}

macro_rules! err {
    ($name:ident, $message:expr $(, $arg_name:ident : $val:expr),*) => {
        err!(_ $name, $message, None $(, $arg_name : $val )* )
    };
    ($name:ident, $message:expr, $e:expr $(, $arg_name:ident : $val:expr),*) => {
        err!(_ $name, $message, Some(Box::new($e)) $(, $arg_name : $val )* )
    };
    (_ $name:ident, $message:expr, $e:expr $(, $arg_name:ident : $val:expr),*) => {
        GatewayError::$name {
            message: $message.into(),
            module_path: module_path!(),
            line: line!(),
            col: column!(),
            source: $e,
            $($arg_name: $val,)*
        }
    };
}

macro_rules! fail {
    (_$code:ident) => {
        fail!(_$code, "Request failed")
    };
    (_$code:ident, $message:expr) => {
        err!(RequestFailed, $message, status: http::StatusCode::$code)
    };
    (_$code:ident, $message:expr, $source:ident) => {
        err!(RequestFailed, $message, $source, status: http::StatusCode::$code)
    };
    ($code:ident) => {
        Err(fail!(_$code))
    };
    ($code:ident, $message:expr) => {
        Err(fail!(_$code, $message))
    };
    ($code:ident, $message:expr, $source:ident) => {
        Err(fail!(_$code, $message, $source))
    };
    (:$code:ident) => {
        Box::new(err(fail!(_$code)))
    };
    (:$code:ident, $message:expr) => {
        Box::new(err(fail!(_$code, $message)))
    };
    (:$code:ident, $message:expr, $source:ident) => {
        Box::new(err(fail!(_$code, $message, $source)))
    };
    (=> $code:ident) => {
        return fail!($code);
    };
    (=> $code:ident, $message:expr) => {
        return fail!($code, $message);
    };
    (=> $code:ident, $message:expr, $source:ident) => {
        return fail!($code, $message, $source);
    };
    (=> :$code:ident) => {
        return fail!(:$code);
    };
    (=> :$code:ident, $message:expr) => {
        return fail!(:$code, $message);
    };
    (=> :$code:ident, $message:expr, $source:ident) => {
        return fail!(:$code, $message, $source);
    };
}

macro_rules! ensure {
    (:$res:expr) => {
        match $res {
            Ok(res) => res,
            Err(e) => return Box::new(err(e)),
        }
    };
}

macro_rules! module_unwrap {
    ($name:ident, $mt:expr) => {
        Arc::new(match $mt {
            Module::$name(mtch) => mtch,
            _ => {
                return Err(err!(
                    RequiredComponent,
                    format!("No module with the name {} is registered", stringify!(Module::$name)),
                    name: stringify!(Module::$name).to_string()
                ))
            }
        })
    };
}
