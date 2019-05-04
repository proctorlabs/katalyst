use crate::prelude::*;

pub struct ModuleError {
    pub error: RequestFailure,
    pub context: Context,
}

impl Context {
    pub fn fail(self, error: RequestFailure) -> ModuleError {
        ModuleError {
            error,
            context: self,
        }
    }
}
