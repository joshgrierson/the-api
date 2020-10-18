use std::fmt;
use serde::Serialize;
use serde_json;

#[macro_use]
mod service;

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

#[derive(Serialize, Debug)]
pub struct ServerResponse {
    code: u16,
    data: Vec<service::ListObject>
}

impl fmt::Display for RoutingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", ser_data!(&self.error))
    }
}

impl fmt::Display for ServerResponse {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", ser_data!(&self))
    }
}

pub fn process_route(route: &str) -> Result<ServerResponse, RoutingError> {
    match route {
        "/lists" => {
            let lists = service::return_lists().unwrap();
            Ok(ServerResponse {
                code: 200,
                data: lists
            })
        },
        _ => Err(RoutingError {
            error: ErrorResponse {
                error_type: ErrorType::InvalidRoute,
                message: format!("Route {} not found", route.to_string())
            }
        })
    }
}