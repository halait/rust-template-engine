use std::env;

use yartl_engine::render_file;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        panic!("Usage: yartl_engine path_to_source path_to_json_context")
    }
    render_file(&args[1], &args[2]);
}
