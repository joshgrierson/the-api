mod routing;

fn main() {
    const TEST_ROUTE: &str = "/list";

    let processed: Result<bool, routing::RoutingError> = routing::process_route(TEST_ROUTE);

    match processed {
        Ok(_) => println!("Found route {}", TEST_ROUTE),
        Err(err) => println!("Routing error: {}", err.to_string())
    }
}
