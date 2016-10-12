use router;

pub fn router() -> router::Router {
    let mut router = router::Router::new();
    router
}

#[cfg(test)]
mod routing_tests {

    extern crate router;
    extern crate iron;
    extern crate iron_test;

    use iron::{Headers};
    use iron::status::Status;

    use handlers;

    #[test]
    fn test_router() {
        let mut router = super::router();
        router.get("/", handlers::home_handler, "get_home");

        let response = iron_test::request::get(
            "http://localhost:3000",
            Headers::new(),
            &router
        ).unwrap();

        assert_eq!(response.status, Some(Status::Ok));
    }
}