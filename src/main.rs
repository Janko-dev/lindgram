use tiny_http::{Server, Response, Method, Header, Request};
use std::{fs::File, error::Error};

fn send_response(request: Request, path: &str, content_type: &str) -> Result<(), Box<dyn Error>> {
    let header = Header::from_bytes("Content-Type", content_type)
        .expect("expected correct header");
    let f = File::open(path)?;
    let mut response = Response::from_file(f);
    response.add_header(header);
    request.respond(response)?;
    Ok(())
}

fn handle_request(request: Request) -> Result<(), Box<dyn Error>> {
    
    match (request.method(), request.url()) {
        (Method::Get, "/") => {
            send_response(request, "client/index.html", "text/html; charset=utf-8")?;
        },
        (Method::Get, "/index.js") => {
            send_response(request, "client/index.js", "text/javascript; charset=utf-8")?;        
        },
        (Method::Post, "/generate") => {
            
        },
        _ => {}
    }
    Ok(())
}

fn main(){
    let server = Server::http("0.0.0.0:8000").unwrap();

    for request in server.incoming_requests() {
        println!("INFO: {:?}, url: {:?}",
            request.method(),
            request.url(),
        );
        handle_request(request).map_err(|e| {
            eprintln!("{e}");
        }).unwrap();
    }
}