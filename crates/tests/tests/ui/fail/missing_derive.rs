fn requires_node<T: umbriel::Node>() {}

struct Plain;

fn main() {
    // `Plain` has no `#[derive(Node)]`; the diagnostic must say so.
    requires_node::<Plain>();
}
