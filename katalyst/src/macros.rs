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
