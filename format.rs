fn format_thing(name: String) -> String {
    format!("hi there {}", name)
}

fn main() {
    println!("{}", format_thing("Cliff".to_string()));
}
