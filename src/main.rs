#[macro_use] extern crate rocket;
mod ip_address;
mod status;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![
            status::get_status, 
            ip_address::get_ip_address,
        ])
}