fn main() {
    let empty_box = None;
    let treasure = Some("gold coins");

    whats_inside(empty_box);
    whats_inside(treasure);

}


fn whats_inside(item: Option<&str>) {
    match item {
        None => println!("Oh no, there's nothing in the box."),
        Some(inside) => println!("Oh wow, you found some {inside} inside the box !"),
    }
}
