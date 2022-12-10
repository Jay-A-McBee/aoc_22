use std::env;
use std::error::Error;
use std::fs::File;
use std::path::Path;

use handlebars::Handlebars;
use serde_json::json;

extern crate handlebars;
extern crate serde_json;

fn main() -> Result<(), Box<dyn Error>> {
    let mut reg = Handlebars::new();

    reg.register_template_file("template", "./src/template.hbs")
        .unwrap();

    let mut args = env::args().skip(1);

    let file_name = format!("../solutions/{}.rs", args.next().unwrap());
    let struct_name = args.next().unwrap();
    let input_file_name = args.next().unwrap();
    let file_path = Path::new(&file_name);

    let mut file = File::create(file_path).expect("Failed to create file");

    reg.render_to_write(
        "template",
        &json!({ "name": struct_name, "input_file_name": input_file_name }),
        &mut file,
    )
    .expect("Failed to write file");

    Ok(())
}
