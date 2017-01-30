#[macro_use]
extern crate nickel;

use nickel::{Nickel, HttpRouter, StaticFilesHandler};
use std::collections::HashMap;


fn main() {
    let mut server = Nickel::new();


    server.utilize(router! {

        get "/status/:status_code" => |req, res| {
            let mut data = HashMap::new();
            data.insert("content", "here some content");
            data.insert("status_code", req.param("status_code").unwrap());
            return res.render("assets/templates/layout.tpl", &data);
        }
    });
    server.utilize(StaticFilesHandler::new("assets/templates"));
    server.listen("127.0.0.1:6767");
}
