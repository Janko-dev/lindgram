mod server;
mod lexer;
mod lsystem;

use lsystem::{Model};
// use minifb::{Key, Window, WindowOptions};

// const WIDTH: usize = 400;
// const HEIGHT: usize = 400;

fn main(){
    // let rules = "A => BBA\n B => AB";
    // let mut model = Model::new("A");
    // model.interpret(rules);
    // model.generate(2);
    
    // println!("{:?}", model.rules);
    // println!("{:?}", model.error_stack);
    // println!("{:?}", model.axiom);

    server::start_server();


    // let mut window = Window::new(
    //     "Test - ESC to exit",
    //     WIDTH,
    //     HEIGHT,
    //     WindowOptions::default(),
    // )
    // .unwrap_or_else(|e| {
    //     panic!("{}", e);
    // });

    // // Limit to max ~60 fps update rate
    // window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    // while window.is_open() && !window.is_key_down(Key::Escape) {
    //     for i in model.img_data.iter_mut() {
    //         *i = 0; // write something more funny here!
    //     }

    //     // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
    //     window
    //         .update_with_buffer(&model.img_data, WIDTH, HEIGHT)
    //         .unwrap();
    // }
}