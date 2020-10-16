mod routing;
mod service;

fn main() {
    const TEST_ROUTE: &str = "/lists";

    let processed: Result<bool, routing::RoutingError> = routing::process_route(TEST_ROUTE);

    match processed {
        Ok(_) => {
            let lists = service::return_lists().unwrap();
            println!("{}", ser_data!(lists))
        },
        Err(err) => println!("Routing error: {}", err.to_string())
    }
}
