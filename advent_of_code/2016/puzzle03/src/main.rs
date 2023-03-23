use std::fs::File;
use std::io::{BufReader, BufRead};


#[derive(Debug, Clone, Copy)]
struct Triangle(i32, i32, i32);

impl From<String> for Triangle {
    fn from(s: String) -> Self {
        let v = s.split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect::<Vec<i32>>();
        Triangle::from(v[0], v[1], v[2])
    }
}

impl Triangle {

    fn from(a: i32, b: i32, c: i32) -> Self {
        Self(a, b, c)
    }


    fn valid(&self) -> bool {
        let mut v = vec![self.0, self.1, self.2];
        v.sort();
        v[0] + v[1] > v[2]
    }
}



fn main() {
    let file = File::open("3").expect("can not read file 3");
    let buffer = BufReader::new(file);
    let mut triangles: Vec<Triangle> = vec![];
    for line in buffer.lines() {    
        triangles.push(line.unwrap().into());
    }


    // Part One:
    let mut possible = 0;
    for triangle in triangles.iter() {
        if triangle.valid() {
            possible += 1;
        }
    }
    println!("Part One: {}", possible);

    // Part Two
    possible = 0;
    
    for i in (0..triangles.len()).step_by(3) {
        let t1 = triangles[i];
        let t2 = triangles[i+1];
        let t3 = triangles[i+2];
    
        let new_t1 = Triangle::from(t1.0, t2.0, t3.0);
        let new_t2 = Triangle::from(t1.1, t2.1, t3.1);
        let new_t3 = Triangle::from(t1.2, t2.2, t3.2);
    
        if new_t1.valid() { possible += 1;}
        if new_t2.valid() { possible += 1;}
        if new_t3.valid() { possible += 1;}
    }

    println!("Part Two: {}", possible);

}
