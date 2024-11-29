use parser::Parser;

mod parser;

fn main() {
    let content_string = "\
    /* Multi-line comment \n\
     * Hello
     */
    \"Hello\";
    // Comment \n \
    42;
    \"Hello, World!  \";





    42;
    ";

    let mut parser = Parser::new(content_string.to_owned()).unwrap();
    let node = parser.parse();
    println!("{:#?}", node);
}
