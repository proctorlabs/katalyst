pub mod builder;
pub mod parsers;

use regex::Regex;

#[derive(Clone, Debug)]
pub struct Gateway {
    pub routes: Vec<Route>,
    pub listener: Listener,
}

#[derive(Clone, Debug)]
pub struct Route {
    pub pattern: Regex,
    pub children: Option<Box<[Route]>>,
}

#[derive(Clone, Debug)]
pub struct Listener {}
