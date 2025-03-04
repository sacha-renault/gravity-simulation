pub mod components;
pub mod physics;

use components::body::Body;

fn main() {
    let body = Body::default();
    println!("{:?}", body)
}
