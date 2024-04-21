use std::io::{Read, Write};
use std::net::TcpStream;
use rocket::request::{self, FromRequest, Request};
use rocket::outcome::Outcome::*;
use rocket::http::Status;
use rocket::serde::json::Json;
use serde::Deserialize;

mod services;

use services::req::{self, RType, Response};
use std::net::SocketAddr;


#[macro_use] extern crate rocket;


struct ClientIp(std::net::IpAddr);


#[derive(Deserialize)]
struct Port(u16);

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

#[post("/", format = "json", data = "<port>")]
async fn new_connection(client_ip: ClientIp, port: Json<Port>) {
	println!("New node: {}:{}", client_ip.0, port.0.0);
	
	
	let address: SocketAddr = format!("{}:{}", client_ip.0, port.0.0).parse().unwrap();

	// test tcp connection to node
	match TcpStream::connect_timeout(&address, std::time::Duration::from_secs(5)) {

		Ok(mut stream) => {
			// create req object
			let req: req::Request = req::Request {
				req_type: RType::ConnectTest
			};

			// serialize req
			let bytes: Vec<u8> = bincode::serialize(&req).unwrap();
			let buffer_size: [u8; 4] = (bytes.len() as u32).to_le_bytes();

			// send req
			if stream.write_all(&buffer_size).is_err()  || stream.write_all(&bytes).is_err() {
				// TODO: return bad status code
				return;
			}	

			// get response
			match req::handle_response(stream) {
				Ok(_) => {},
				Err(e) => {
					// TODO: return bad status code
					eprintln!("{e}");
					return;
				}
			}
		},
		Err(e) => {
			// TODO: return bad status code
			eprintln!("{e}");
			return;
		}
	};
	
	// TODO: create entry in db. The IP with the successful connection to node is saved. Only those IPs can send other requests
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
