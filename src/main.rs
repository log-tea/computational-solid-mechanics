mod finite_element;

extern crate nalgebra as na;
use na::Vector3;
use finite_element::lagrange::add;

fn main() {
    println!("Hello, world!");
    add(1, 2);

    let v = Vector3::new(1, 2, 3);
    print!("{}",v)
    
}
