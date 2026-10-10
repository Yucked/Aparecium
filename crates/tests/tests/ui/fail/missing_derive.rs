fn requires_node<T: aparecium::Node>() {}

struct Plain;

fn main() {
    // `Plain` has no `#[derive(Node)]`; the diagnostic must say so.
    requires_node::<Plain>();
}
