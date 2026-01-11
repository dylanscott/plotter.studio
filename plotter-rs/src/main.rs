use std::env;
use std::fs;
use usvg::{Node, Options, Tree};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <svg-file>", args[0]);
        std::process::exit(1);
    }

    let svg_path = &args[1];
    let svg_data = fs::read(svg_path).expect("Failed to read SVG file");

    let opt = Options::default();
    let tree = Tree::from_data(&svg_data, &opt).expect("Failed to parse SVG");

    println!("SVG size: {:?}", tree.size());
    println!("\nTree structure:");
    print_group(tree.root(), 0);
}

fn print_group(group: &usvg::Group, indent: usize) {
    let prefix = "  ".repeat(indent);

    if !group.id().is_empty() {
        println!("{}Group id=\"{}\"", prefix, group.id());
    } else {
        println!("{}Group", prefix);
    }

    println!("{}  transform: {:?}", prefix, group.transform());
    println!("{}  bounding_box: {:?}", prefix, group.bounding_box());

    for node in group.children() {
        print_node(node, indent + 1);
    }
}

fn print_node(node: &Node, indent: usize) {
    let prefix = "  ".repeat(indent);

    match node {
        Node::Group(group) => {
            print_group(group, indent);
        }
        Node::Path(path) => {
            if !path.id().is_empty() {
                println!("{}Path id=\"{}\"", prefix, path.id());
            } else {
                println!("{}Path", prefix);
            }
            println!("{}  visible: {}", prefix, path.is_visible());
            println!("{}  bounding_box: {:?}", prefix, path.bounding_box());

            if let Some(fill) = path.fill() {
                println!("{}  fill: {:?}", prefix, fill.paint());
            }
            if let Some(stroke) = path.stroke() {
                println!("{}  stroke: {:?}", prefix, stroke.paint());
            }

            println!("{}  segments:", prefix);
            for segment in path.data().segments() {
                println!("{}    {:?}", prefix, segment);
            }
        }
        Node::Image(image) => {
            if !image.id().is_empty() {
                println!("{}Image id=\"{}\"", prefix, image.id());
            } else {
                println!("{}Image", prefix);
            }
            println!("{}  size: {:?}", prefix, image.size());
        }
        Node::Text(text) => {
            if !text.id().is_empty() {
                println!("{}Text id=\"{}\"", prefix, text.id());
            } else {
                println!("{}Text", prefix);
            }
            println!("{}  bounding_box: {:?}", prefix, text.bounding_box());
        }
    }
}
