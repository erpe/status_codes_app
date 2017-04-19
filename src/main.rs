mod codes;

#[macro_use]
extern crate nickel;

use nickel::{Nickel, StaticFilesHandler};
use nickel::status::StatusCode;
use std::collections::HashMap;


fn main() {
    let mut server = Nickel::new();

    server.utilize(router! {
        get "/status/:status_code" => |req, mut res| {
            let mut data = HashMap::new();
            let arg = req.param("status_code").unwrap();
            let i_arg = arg.parse::<u16>().unwrap();
            data.insert("status_code", arg);
            res.set(extract_status_code(i_arg));
            return res.render("assets/templates/layout.tpl", &data);
        }
    });
    server.utilize(StaticFilesHandler::new("assets/templates"));
    server.listen("0.0.0.0:8080");
}


fn extract_status_code(sc: u16) -> StatusCode  {
    return StatusCode::from_u16(sc)
}
