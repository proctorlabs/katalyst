#[macro_export]
macro_rules! try_fut {
    ($e:expr) => {
        match $e {
            Err(err) => {
                return Box::new(futures::future::err(err.into()));
            }
            Ok(value) => value,
        }
    };
}

#[macro_export]
macro_rules! ok {
    ($e:expr) => {
        Box::new(futures::future::ok::<Context, RequestFailure>($e))
    };
}

#[macro_export]
macro_rules! err {
    ($e:expr) => {
        Box::new(futures::future::err::<Context, RequestFailure>($e))
    };
}
