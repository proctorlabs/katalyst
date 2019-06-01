#![allow(dead_code)]
#![allow(clippy::eval_order_dependence)]
use super::*;
use crate::prelude::*;
use pest::Parser;
use pest_derive::*;

#[derive(Parser)]
#[grammar = "expression/expr.pest"]
#[allow(dead_code)]
struct TemplateParser;

#[derive(Debug)]
pub enum ExpressionMetadata {
    Raw(String),
    Number(i64),
    Bool(bool),
    Text(String),
    Expression { module: String, method: String, args: Vec<ExpressionMetadata> },
}

impl ExpressionMetadata {
    pub fn compile(
        self,
        directory: &BuilderDirectory,
    ) -> std::result::Result<Arc<CompiledExpression>, GatewayError> {
        match self {
            ExpressionMetadata::Expression { module, method, args } => {
                let builder = directory.get(&module.as_str());
                match builder {
                    Some(b) => {
                        let mut c_args: Vec<Arc<CompiledExpression>> = vec![];
                        for arg in args.into_iter() {
                            c_args.push(arg.compile(directory)?);
                        }
                        Ok(Arc::new(CompiledExpressionNode {
                            name: module.to_owned(),
                            render_fn: b.make_fn(&method, &c_args)?,
                            args: c_args,
                            result: ExpressionResultType::Text,
                        }))
                    }
                    None => Err(GatewayError::ExpressionItemNotFound(module)),
                }
            }
            ExpressionMetadata::Raw(text) | ExpressionMetadata::Text(text) => Ok(Arc::new(text)),
            ExpressionMetadata::Number(number) => Ok(Arc::new(number)),
            ExpressionMetadata::Bool(cnd) => Ok(Arc::new(cnd)),
        }
    }
}

pub fn parse_template(
    input: &str,
    directory: &BuilderDirectory,
) -> Result<Vec<Arc<CompiledExpression>>> {
    let tokens = TemplateParser::parse(Rule::template, input)?;
    let metadata = parse_tokens(tokens)?;
    let mut result = vec![];
    for item in metadata.into_iter() {
        result.push(item.compile(directory)?);
    }
    Ok(result)
}

fn parse_tokens(
    pairs: pest::iterators::Pairs<'_, Rule>,
) -> std::result::Result<Vec<ExpressionMetadata>, GatewayError> {
    let mut result = vec![];
    for pair in pairs {
        match pair.as_rule() {
            Rule::raw_block => result.push(ExpressionMetadata::Raw(pair.as_str().into())),
            Rule::number_lit => result.push(ExpressionMetadata::Number(pair.as_str().parse()?)),
            Rule::true_lit => result.push(ExpressionMetadata::Bool(true)),
            Rule::false_lit => result.push(ExpressionMetadata::Bool(false)),
            Rule::string_lit => result
                .push(ExpressionMetadata::Text(pair.into_inner().as_str().replace("\\'", "'"))),
            Rule::object_call => result.push(parse_object(pair.into_inner())?),
            Rule::EOI => return Ok(result),
            _ => {
                return Err(GatewayError::ExpressionLexicalError(
                    "Unexpected element found!".into(),
                ))
            }
        }
    }
    Ok(result)
}

fn parse_object(
    mut pairs: pest::iterators::Pairs<'_, Rule>,
) -> std::result::Result<ExpressionMetadata, GatewayError> {
    let module = pairs.next().unwrap().as_str().to_string();
    let method = pairs.next().unwrap().as_str().to_string();
    Ok(ExpressionMetadata::Expression { module, method, args: parse_tokens(pairs)? })
}

#[cfg(test)]
mod test {
    use super::*;

    lazy_static! {
        static ref BUILDERS: BuilderDirectory = BuilderDirectory::default();
    }

    #[test]
    fn test_parser() -> std::result::Result<(), GatewayError> {
        let result = parse_template("/this/is/a/ {{ 'path' }} /to/something", &BUILDERS)?;
        println!("{:?}", result);
        Ok(())
    }
}
