use rocket::{ catch, catchers, get, launch, Request, routes };

#[get("/hello")]
fn index() -> &'static str {
    "Hello, world!"
}

#[catch(404)]
fn not_found(req: &Request) -> String {
    format!("I couldn't find '{}'. Try something else?", req.uri())
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index]).register("/", catchers![not_found])
}
