use crate::{expression::*, prelude::*};
use std::{fmt, sync::Arc};
use unstructured::Document;

pub struct CompiledExpressionNode {
    pub name: String,
    pub result: ExpressionResultType,
    pub args: Vec<Arc<dyn CompiledExpression>>,
    pub render_fn: ExpressionRenderMethod,
}

impl CompiledExpression for CompiledExpressionNode {
    fn render(&self, guard: &ContextGuard) -> RenderResult {
        Ok(self.result(guard)?.to_string())
    }

    fn result(&self, guard: &ContextGuard) -> ExpressionResult {
        (self.render_fn)(guard, &&self.args)
    }

    fn result_type(&self) -> Document {
        "".into()
    }
}

impl CompiledExpression for Document {
    fn render(&self, _: &ContextGuard) -> RenderResult {
        Ok(self.to_string())
    }

    fn result(&self, _: &ContextGuard) -> ExpressionResult {
        Ok(self.clone())
    }

    fn result_type(&self) -> Document {
        self.clone()
    }
}

impl CompiledExpression for String {
    fn render(&self, _: &ContextGuard) -> RenderResult {
        Ok(self.to_string())
    }

    fn result(&self, _: &ContextGuard) -> ExpressionResult {
        Ok(self.as_str().into())
    }

    fn result_type(&self) -> Document {
        "".into()
    }
}

impl CompiledExpression for i64 {
    fn render(&self, _: &ContextGuard) -> RenderResult {
        Ok(self.to_string())
    }

    fn result(&self, _: &ContextGuard) -> ExpressionResult {
        Ok((*self).into())
    }

    fn result_type(&self) -> Document {
        (0 as i64).into()
    }
}

impl CompiledExpression for bool {
    fn render(&self, _: &ContextGuard) -> RenderResult {
        Ok(self.to_string())
    }

    fn result(&self, _: &ContextGuard) -> ExpressionResult {
        Ok((*self).into())
    }

    fn result_type(&self) -> Document {
        true.into()
    }
}

impl fmt::Debug for CompiledExpressionNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}(", &self.name)?;
        self.args.fmt(f)?;
        write!(f, ") -> ")?;
        match self.result {
            ExpressionResultType::Text => write!(f, "str"),
            ExpressionResultType::Number => write!(f, "i64"),
            ExpressionResultType::Boolean => write!(f, "bool"),
        }
    }
}
