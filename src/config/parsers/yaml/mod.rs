use crate::config::builder::{GatewayBuilder, RouteBuilder};
use std::string::String;
use yaml_rust::yaml::Yaml;
use yaml_rust::YamlLoader;

pub fn process_yaml<'a, 'b>(yaml_contents: String) -> GatewayBuilder<'a> {
    let mut builder = GatewayBuilder::new();
    let docs = YamlLoader::load_from_str(&yaml_contents).unwrap();
    let doc = &docs[0];
    let mut routes = process_routes(&doc["routes"]);
    builder.push_routes(&mut routes);
    builder
}

fn process_routes<'a>(doc: &Yaml) -> Vec<RouteBuilder<'a>> {
    return match doc {
        Yaml::Array(ref node) => {
            let mut routes = vec![];
            for n in node {
                routes.push(process_route(n));
            }
            routes
        }
        _ => panic!("Unexpected node type occurred! Please check configuration file syntax."),
    };
}

fn process_route<'a>(doc: &Yaml) -> RouteBuilder<'a> {
    let mut route: RouteBuilder<'a> = RouteBuilder::new();

    match doc {
        Yaml::Hash(ref node) => {
            for (k, v) in node {
                if k.as_str().unwrap() == "pattern" {
                    let newval = String::from(v.as_str().unwrap());
                    route = route.set_pattern(newval);
                } else if k.as_str().unwrap() == "children" {
                    route = route.add_children(&mut process_routes(v));
                }
            }
        }
        _ => panic!("Unexpected node type occurred! Please check configuration file syntax."),
    };
    route
}
