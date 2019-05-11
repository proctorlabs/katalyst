use hyper::{Body, Request, Response};

#[derive(Debug)]
pub enum RequestContainer {
    Empty,
    Raw { data: Box<Request<Body>> },
    Parsed,
}

impl RequestContainer {
    pub fn new(req: Request<Body>) -> Self {
        RequestContainer::Raw {
            data: Box::new(req),
        }
    }

    pub fn raw(&self) -> &Request<Body> {
        match self {
            RequestContainer::Empty => unimplemented!(),
            RequestContainer::Raw { data } => data,
            RequestContainer::Parsed => unimplemented!(),
        }
    }

    pub fn take(self) -> Request<Body> {
        match self {
            RequestContainer::Empty => Request::default(),
            RequestContainer::Raw { data } => *data,
            RequestContainer::Parsed => unimplemented!(),
        }
    }
}

#[derive(Debug)]
pub enum ResponseContainer {
    Empty,
    Raw { data: Box<Response<Body>> },
    Parsed,
}

impl ResponseContainer {
    pub fn new(req: Response<Body>) -> Self {
        ResponseContainer::Raw {
            data: Box::new(req),
        }
    }

    pub fn take(self) -> Response<Body> {
        match self {
            ResponseContainer::Empty => Response::default(),
            ResponseContainer::Raw { data } => *data,
            ResponseContainer::Parsed => unimplemented!(),
        }
    }
}
