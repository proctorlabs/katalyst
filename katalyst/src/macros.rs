#[macro_export]
macro_rules! try_fut {
    ($c:expr,$e:expr) => {
        match $e {
            Err(err) => {
                return Box::new(futures::future::err($c.fail(err.into())));
            }
            Ok(value) => value,
        }
    };
}

#[macro_export]
macro_rules! try_req {
    ($c:expr,$e:expr) => {
        match $e {
            Err(err) => {
                return Err($c.fail(err.into()));
            }
            Ok(value) => value,
        }
    };
}

#[macro_export]
macro_rules! ok {
    ($e:expr) => {
        Box::new(futures::future::ok::<Context, ModuleError>($e))
    };
}

#[macro_export]
macro_rules! err {
    ($c:expr,$e:expr) => {
        Box::new(futures::future::err::<Context, ModuleError>($c.fail($e)))
    };
}

#[cfg(test)]
mod test {
    use crate::prelude::*;

    #[derive(ExpressionBinding)]
    #[allow(dead_code)]
    enum ThingBinding {
        #[expression(method = test)]
        One,
        #[expression(method = test)]
        Two,
    }

    impl ThingBinding {
        fn test(_: &Context, _: &[ExpressionArg]) -> ExpressionResult {
            unimplemented!()
        }
    }

    #[test]
    fn derive_test() {}

}
