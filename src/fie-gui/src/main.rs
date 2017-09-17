extern crate sylphid;

use std::path::Path;
use sylphid::sylphid::Sylphid;

fn main() {
    let mut s = Sylphid::new();
    s.load_from_file(&Path::new("sp.jpg"));
    s.run(5,5000,1);
    for i in 0..(s.result_size()) {
        println!("{:?}",s.result_at(i));
    }
}
