use crate::pipeline::*;
use crate::prelude::*;

pub fn matcher(mut ctx: Context) -> ModuleResultSync {
    let request = ctx.upstream.request()?;
    let config = ctx
        .engine
        .get_state()
        .context("Failed to get configuration")?;
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
            let caps = route
                .pattern
                .captures(path)
                .ok_or_else(|| RequestFailure::Internal)?;
            for name_option in route.pattern.capture_names() {
                if name_option.is_some() {
                    let name = name_option.ok_or_else(|| RequestFailure::Internal)?;
                    cap_map.insert(
                        name.to_string(),
                        caps.name(name)
                            .ok_or_else(|| RequestFailure::Internal)?
                            .as_str()
                            .to_string(),
                    );
                }
            }
            ctx.detail.matched_route = Some(route.clone());
            ctx.detail.captures = Some(cap_map);
            debug!("Request has been matched to route!");
            return Ok(ctx);
        }
    }
    Err(RequestFailure::NotFound)
}
