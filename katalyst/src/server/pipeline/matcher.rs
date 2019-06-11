use super::*;
use crate::prelude::*;

pub fn matcher(guard: RequestContext) -> AsyncResult<()> {
    match_int(guard).fut()
}

fn match_int(guard: RequestContext) -> Result<()> {
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
            let caps = route.pattern.captures(path).ok_or_else(
                || fail!(_ INTERNAL_SERVER_ERROR, format!("Captures not found for path {}", path)),
            )?;
            for name_option in route.pattern.capture_names() {
                if name_option.is_some() {
                    let name = name_option.ok_or_else(|| fail!(_ INTERNAL_SERVER_ERROR))?;
                    cap_map.insert(
                        name.to_string(),
                        caps.name(name)
                            .ok_or_else(|| fail!(_ INTERNAL_SERVER_ERROR, format!("Route {} has no placeholder for {}", path, name)))?
                            .as_str()
                            .to_string(),
                    );
                }
            }
            guard.set_match(Match::Matched { route: route.clone(), captures: cap_map })?;
            debug!("Request has been matched to route!");
            return Ok(());
        }
    }
    fail!(NOT_FOUND)
}
