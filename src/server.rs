use tiny_http::{Server, Response, Method, Header, Request};
use std::{fs::File, error::Error, f64::consts::PI};
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

fn get_string(val: &mut Value, name: &str) -> String {
    if let Value::String(s) = val[name].take(){
        return s;
    } else {
        return String::new();
    }
}

fn get_number(val: &mut Value, name: &str) -> i32 {
    if let Value::Number(s) = val[name].take(){
        return s.as_i64().unwrap() as i32;
    } else {
        return 1;
    };
}

#[derive(Default, Debug)]
struct Params {
    axiom: String,
    rules: String,
    n: i32,
    width: usize,
    height: usize,
    line_len: i32,
    angle: f64,
    xoffset: i32,
    yoffset: i32
}

fn get_variables(content: String) -> Result<Params, Box<dyn Error>> {
    let mut val: Value = serde_json::from_str(&content)?;

    let mut params: Params = Default::default();

    params.axiom = get_string(&mut val, "axiom");
    params.rules = get_string(&mut val, "rules");
    params.n = get_number(&mut val, "n");
    params.width = get_number(&mut val, "width") as usize;
    params.height = get_number(&mut val, "height") as usize;
    params.line_len = get_number(&mut val, "line_len");
    params.angle = get_number(&mut val, "angle") as f64 / 360. * 2. * PI;
    params.xoffset = get_number(&mut val, "offsetX");
    params.yoffset = get_number(&mut val, "offsetY");
    Ok(params)
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
            let params = get_variables(content)?;
            // println!("{:?}", params);

            let mut model = Model::new(&params.axiom);
            model.interpret(&params.rules);
            model.generate(params.n);
            let pixels = model.render(params.width, params.height, params.xoffset, params.yoffset, params.line_len, params.angle);

            let response = Response::from_data(pixels);
            request.respond(response)?;
        },
        _ => {}
    }
    Ok(())
}

pub fn start_server() {
    let server_addr = "0.0.0.0:8000";
    let server = Server::http(server_addr).unwrap();
    println!("INFO: Server started at {server_addr}");
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