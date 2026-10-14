fn requires_node<T: umbriel::Node>() {}

struct Plain;

fn main() {
    // `Plain` is not declared with `#[model(node)]`; the diagnostic must say so.
    requires_node::<Plain>();
}
