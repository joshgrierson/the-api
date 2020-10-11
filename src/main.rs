use std::fmt;

#[derive(Debug)]
struct RoutingError;

impl fmt::Display for RoutingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Invalid route")
    }
}

fn process_route(route: &str) -> Result<bool, RoutingError> {
    match route {
        "/lists" => Ok(true),
        _ => Err(RoutingError)
    }
}

fn main() {
    const TEST_ROUTE: &str = "/list";

    let processed: Result<bool, RoutingError> = process_route(TEST_ROUTE);

    match processed {
        Ok(_) => println!("Found route {}", TEST_ROUTE),
        Err(err) => println!("Routing error: {}", err.to_string())
    }
}
