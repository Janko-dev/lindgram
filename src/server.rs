use tiny_http::{Server, Response, Method, Header, Request};
use std::{fs::File, error::Error};
use serde_json::{Value};

use crate::lsystem::Model;

fn send_response(request: Request, path: &str, content_type: &str) -> Result<(), Box<dyn Error>> {
    let header = Header::from_bytes("Content-Type", content_type)
        .expect("expected correct header");
    let f = File::open(path)?;
    let mut response = Response::from_file(f);
    response.add_header(header);
    request.respond(response)?;
    Ok(())
}

fn get_axiom_rules_n(content: String) -> Result<(String, String, i32), Box<dyn Error>> {
    let mut val: Value = serde_json::from_str(&content)?;
    let axiom = if let Value::String(s) = val["axiom"].take(){
        s
    } else {
        String::new()
    };

    let rules = if let Value::String(s) = val["rules"].take(){
        s
    } else {
        String::new()
    };

    let n = if let Value::Number(s) = val["n"].take(){
        s.as_i64().unwrap() as i32
    } else {
        1
    };

    Ok((axiom, rules, n))
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
            let mut content = String::new();
            let mut request = request;
            request.as_reader().read_to_string(&mut content).unwrap();
            let (axiom, rules, n) = get_axiom_rules_n(content)?;

            let mut model = Model::new(&axiom);
            model.interpret(&rules);
            model.generate(n);

            println!("{:?}", model.rules);
            println!("{:?}", model.error_stack);
            println!("{:?}", model.axiom);
            let data = vec![51 as u8; 500*500*4];

            let header = Header::from_bytes("Content-Type", "application/octet-stream")
                .expect("expected correct header");
            let mut response = Response::from_data(data);
            response.add_header(header);
            request.respond(response)?;
        },
        _ => {}
    }
    Ok(())
}

pub fn start_server() {
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