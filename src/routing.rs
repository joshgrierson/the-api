use std::fmt;

#[derive(Debug)]
pub struct RoutingError;

impl fmt::Display for RoutingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Invalid route")
    }
}

pub fn process_route(route: &str) -> Result<bool, RoutingError> {
    match route {
        "/lists" => Ok(true),
        _ => Err(RoutingError)
    }
}