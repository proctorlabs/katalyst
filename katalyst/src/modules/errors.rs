use crate::prelude::*;

pub struct ModuleError {
    pub error: GatewayError,
    pub context: Context,
}

impl Context {
    pub fn fail(self, error: GatewayError) -> ModuleError {
        ModuleError {
            error,
            context: self,
        }
    }
}
