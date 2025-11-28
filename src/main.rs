// partie 1
use svg::Document;
use svg::node::element::Rectangle;
use std::env;

fn main() {

    let args: Vec<String> = env::args().collect();
    
   
    let taille = if args.len() > 1 {
        args[1].parse().unwrap_or(729.0)
    } else { 
        729.0
    };
    
    let iterations_max = if args.len() > 2 {
        args[2].parse().unwrap_or(4)
    } else {
        4
    };
    
  
    println!("taille: {} pixels", taille);
    println!("itérations max: {}", iterations_max);
    println!();
   
    let mut a: i32 = 0;
    
 
    for iteration in 0..=iterations_max {
        generer_fichier_iteration(taille, iteration);
        a += 1
    }
  
    
    println!("\n{} fichiers générés ", a);
}



fn generer_fichier_iteration(taille: f64, iteration: u32) {
    
    let mut document = Document::new()
        .set("width", taille)
        .set("height", taille)
        .set("viewBox", (0, 0, taille, taille));
     
    
    
    let fond = Rectangle::new()
        .set("x", 0)
        .set("y", 0)
        .set("width", taille)
        .set("height", taille)
        .set("fill", "white");

    document = document.add(fond);
    

    let rectangles = generer_cantor(0.0, 0.0, taille, iteration);
    
    // add chaque rectangle au document SVG
    for rect in rectangles {
        document = document.add(rect);
    }
    
    
    let filename = format!("cantor_iteration_{}.svg", iteration);
   
    svg::save(&filename, &document).unwrap();
    
    // calculer et afficher le nombre de carrés générés (4^iteration)
    let nb_carres = 4_u32.pow(iteration);
    println!("{} créé - {} carrés", filename, nb_carres);
}


fn generer_cantor(x: f64, y: f64, taille: f64, iterations: u32) -> Vec<Rectangle> {
    let mut rectangles = Vec::new();
    
    // cas de base : si iterations == 0, on dessine un Rectangle final
    if iterations == 0 {
        let rect = Rectangle::new()
            .set("x", x)
            .set("y", y)
            .set("width", taille)
            .set("height", taille)
            .set("fill", "purple")
            .set("stroke", "none");
        
        rectangles.push(rect);
        return rectangles;
    }
    
    // diviser la taille par 3 a chaque carré
    let nouvelle_taille = taille / 3.0;
  

    
    // Coin supérieur gauche (0, 0)
    rectangles.extend(generer_cantor(x, y, nouvelle_taille, iterations - 1));
    
    // Coin supérieur droit (on saute 2 cases horizontalement)
    rectangles.extend(generer_cantor(x + 2.0 * nouvelle_taille, y, nouvelle_taille, iterations - 1));
    
    // coin inférieur gauche (on saute 2 cases verticalement)
    rectangles.extend(generer_cantor(x, y + 2.0 * nouvelle_taille, nouvelle_taille, iterations - 1));
    
    // coin inférieur droit (on saute 2 cases en diagonal)
    rectangles.extend(generer_cantor(
        x + 2.0 * nouvelle_taille,
        y + 2.0 * nouvelle_taille, 
        nouvelle_taille,
        iterations - 1));
    
    // retourner tous les rectangles (les 4 coins combinés)
    rectangles
}