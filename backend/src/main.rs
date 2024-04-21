use rocket::request::{self, FromRequest, Request};
use rocket::outcome::Outcome::*;
use rocket::http::Status;
use std::fmt;

mod services;

#[macro_use] extern crate rocket;


struct ClientIp(std::net::IpAddr);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for ClientIp {
	type Error = ();

	async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
		match req.client_ip() {
			Some(ip) => Success(ClientIp(ip)),
			None => Error((Status::BadRequest, ())),
		}
	}
}


impl fmt::Display for ClientIp {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{:?}", self.0)
	}
}

#[post("/", format = "json")]
fn new_connection(client_ip: ClientIp) -> String {
	println!("{}", client_ip);

	// TODO: get request to their ip address at correct port,
	// if get is success
		// TODO: give token
		// TODO: establish a way of knowning whether post was called or not (give tokens?) (post is only called once)
	
	// step one: make it so that when i recieve connection, i ping address and port and wait for response, 
	// if no response then not a node, and send response to post being: yeah dont work, check firewall
		// if no response then not a node, and send response to post being: yeah dont work, check firewall
	format!("YES!")
}

#[get("/")]
fn get_nodes() -> &'static str {
	// TODO: establish a way of knowning whether post was called or not (give tokens?)
	"fellas"
}

#[delete("/")]
fn disconnect_node() -> &'static str {
	// TODO: delete node associated with token from db
	"yessir"
}


#[launch]
fn rocket() -> _ {
	rocket::build().mount("/api", routes![new_connection, get_nodes, disconnect_node])
}
