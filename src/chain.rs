use router;
use handlers;

pub fn chain() -> router::Router {
    let mut router = router::Router::new();
    router.get("/", handlers::home_handler, "get_home");
    router.post("/new_task", handlers::new_task_handler, "new_task");
    router
}

#[cfg(test)]
mod chain_tests {

    extern crate router;
    extern crate iron;
    extern crate iron_test;

    use iron::{Headers};
    use iron::status::Status;

    #[test]
    fn post_new_task_resonds_with_200() {
        let router = super::chain();
        let response = iron_test::request::post(
            "http://localhost:3000/new_task",
            Headers::new(),
            "",
            &router
        ).unwrap();

        assert_eq!(response.status, Some(Status::Ok))
    }

    #[test]
    fn get_root_responds_with_200() {
        let router = super::chain();
        let response = iron_test::request::get(
            "http://localhost:3000",
            Headers::new(),
            &router
        ).unwrap();

        assert_eq!(response.status, Some(Status::Ok))
    }
}