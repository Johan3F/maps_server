use rocket::{get, routes, Build, Rocket};

pub fn add_routes(rocket: Rocket<Build>) -> Rocket<Build> {
    rocket.mount("/", routes![health])
}

#[get("/health")]
pub fn health() {}

#[cfg(test)]
mod test {
    use crate::rocket;
    use rocket::http::Status;
    use rocket::local::blocking::Client;

    #[test]
    fn health_endpoint() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let response = client.get("/health").dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.into_string(), None)
    }
}
