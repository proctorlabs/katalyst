use super::*;
use crate::prelude::*;

pub fn matcher(guard: ContextGuard) -> AsyncResult<()> {
    match_int(guard).fut()
}

fn match_int(guard: ContextGuard) -> Result<()> {
    let metadata = guard.metadata()?;
    let config = guard.katalyst()?.get_instance()?;
    let method = guard.method();
    for route in config.routes.iter() {
        let method_match = match &route.methods {
            Some(methods) => methods.contains(&method),
            None => true,
        };
        let path = metadata.url.path();
        if method_match && route.pattern.is_match(path) {
            let mut cap_map = HashMap::new();
            let caps =
                route.pattern.captures(path).ok_or_else(|| GatewayError::InternalServerError)?;
            for name_option in route.pattern.capture_names() {
                if name_option.is_some() {
                    let name = name_option.ok_or_else(|| GatewayError::InternalServerError)?;
                    cap_map.insert(
                        name.to_string(),
                        caps.name(name)
                            .ok_or_else(|| GatewayError::InternalServerError)?
                            .as_str()
                            .to_string(),
                    );
                }
            }
            guard.set_match(MatchInfo { route: route.clone(), captures: cap_map })?;
            debug!("Request has been matched to route!");
            return Ok(());
        }
    }
    Err(NotFound)
}
