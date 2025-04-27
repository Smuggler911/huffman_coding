use crate::huffman::build_tree;

mod huffman;

fn main() {
    let text:String = "some test text is here".to_string();
    build_tree(text);
}
