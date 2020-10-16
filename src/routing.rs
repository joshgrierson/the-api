use std::fmt;
use serde::Serialize;
use serde_json;

#[derive(Serialize, Debug)]
enum ErrorType {
    InvalidRoute
}

#[derive(Serialize, Debug)]
struct ErrorResponse {
    error_type: ErrorType,
    message: String
}

#[derive(Debug)]
pub struct RoutingError {
    error: ErrorResponse
}

impl fmt::Display for RoutingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", serde_json::to_string(&self.error).unwrap())
    }
}

pub fn process_route(route: &str) -> Result<bool, RoutingError> {
    match route {
        "/lists" => Ok(true),
        _ => Err(RoutingError {
            error: ErrorResponse {
                error_type: ErrorType::InvalidRoute,
                message: format!("Route {} not found", route.to_string())
            }
        })
    }
}