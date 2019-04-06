use crate::pipeline::*;

#[derive(Default)]
pub struct Matcher {}

impl Pipeline for Matcher {
    fn name(&self) -> &'static str {
        "matcher"
    }

    fn prepare_request(&self, mut ctx: Context) -> PipelineResult {
        let request = match &ctx.upstream.request {
            Some(r) => r,
            None => return Err(KatalystError::NotFound),
        };
        let config = ctx.engine.get_state()?;
        for route in config.routes.iter() {
            let method_match = match &route.methods {
                Some(methods) => {
                    let up_method = request.method();
                    methods.contains(up_method)
                }
                None => true,
            };
            let path = request.uri().path();
            if method_match && route.pattern.is_match(path) {
                let mut cap_map = HashMap::new();
                let caps = route.pattern.captures(path).unwrap();
                for name_option in route.pattern.capture_names() {
                    if name_option.is_some() {
                        let name = name_option.unwrap();
                        cap_map.insert(
                            name.to_string(),
                            caps.name(name).unwrap().as_str().to_string(),
                        );
                    }
                }
                ctx.context.matched_route = Some(route.clone());
                ctx.context.captures = Some(cap_map);
                debug!("Request has been matched to route!");
                return Ok(ctx);
            }
        }
        Err(KatalystError::NotFound)
    }
}
