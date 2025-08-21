mod version;
mod domain;

fn main() {
    info_page();
}

fn info_page() {
    println!("Flight Travel Record v{}", version::get_version());
    println!();
    println!("Enter the number below to use features:");
}
