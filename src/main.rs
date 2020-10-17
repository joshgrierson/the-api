mod api;

fn main() {
    const TEST_ROUTE: &str = "/lists";

    let processed= api::process_route(TEST_ROUTE);

    match processed {
        Ok(response) => println!("{}", response),
        Err(err) => println!("Routing error: {}", err.to_string())
    }
}
