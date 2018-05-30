extern crate phashmap;

use phashmap::*;

fn main() {
    let mut h8: PHashMap<u8, u8> = PHashMap::new();
    h8.insert(8, 0);
    h8.insert(9, 2);
    h8.insert(9, 3);
    h8.insert(10, 4);
    println!("{:?}", h8.get(8));
    println!("{:?}", h8.get(11));
    println!("{:?}", h8.get(9));
    h8.update(9, 5);
    println!("{:?}", h8.get(9));
    *h8.get_mut_def(9, 10) += 1;
    println!("{:?}", h8.get(9));
    *h8.get_mut_def(19, 0) += 1;
    println!("{:?}", h8.get(19));    
}
