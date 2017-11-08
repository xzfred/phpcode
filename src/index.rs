use rocket::{self, Rocket};

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

pub fn rocket() -> Rocket {
    rocket::ignite()
        .mount("/", routes![index])
}
