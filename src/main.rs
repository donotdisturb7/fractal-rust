use svg::Document;
use svg::node::element::Path;
use svg::node::element::path::Data;
use arg::Args;
use std::env;



fn main() {
    let args: Vec<String> = env::args().collect();
    let fill_test = args.get(1).expect("Veuillez fournir un fichier en argument");

    println!("{}", fill_test);



    let data = Data::new()
        .move_to((10, 10))
        .line_by((0, 50))
        .line_by((50, 0))
        .line_by((0, -50))
        .close();

    let path = Path::new()
        .set("fill", "none")
        .set("stroke", &**fill_test)
        .set("stroke-width", 3)
        .set("d", data);

    let document = Document::new()
        .set("viewBox", (0, 0, 70, 70))
        .add(path);
    
    svg::save("image.svg", &document).unwrap();
}

