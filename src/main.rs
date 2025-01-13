use crate::prelude::*;

mod errors;
mod parser;
mod prelude;
mod tokenizer;
mod tree;

fn main() {
    let content_string = "
        class Point {
            constructor(x, y) {
                this.x = x;
                this.y = y;
            }

            sum() {
                return this.x + this.y;
            }
        }

        class Point3D extends Point {
            constructor(x, y, z) {
                super(x, y);
                this.z = z;
            }
        
            sum() {
                return super.sum() + this.z;
            }
        }

        let p = new Point3D(10, 20, 30);
        p.sum();
    ";

    let mut parser = Parser::new(content_string.to_owned()).unwrap();
    let node = parser.parse();
    println!("{:#?}", node);
}
