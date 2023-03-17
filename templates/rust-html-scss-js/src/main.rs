use arcanum::{WebServer};

fn main() {
    let mut server = WebServer::new("127.0.0.1", 7878);
    server.add_static_file_route("/style/**", "css/");
    server.run();
}
