use std::alloc::alloc;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::iter::Map;
use std::ops::{Deref, DerefMut, Index};
use std::ptr::null;

macro_rules! post_increment {
    ($i:ident) => {{
        let old = $i;
        $i+=1;
        old.try_into().unwrap()
    }};
}

#[derive(Ord, Eq, PartialEq, PartialOrd, Debug)]
struct Node {
    char:char,
    freq:i32,
    left:Option<Box<Node>>,
    right:Option<Box<Node>>,
}

impl Node {
     fn make_node (char:char, freq:i32,left:Option<Box<Node>>,right:Option<Box<Node>>) -> Node {
        Node{
            char,
            freq,
            left,
            right
        }
    }

}


impl Compare for Node {
    fn operator(a: Node, b: Node) -> bool {
         b.freq < a.freq
    }
}

pub trait Compare {
    fn operator(a:Node, b:Node) -> bool;
}


fn encode(root: Option<&Box<Node>>, str:String, huff_code: &mut HashMap<char, String>) -> HashMap<char, String> {
    if root.is_none(){
        return huff_code.clone();
    }

    let root = root.unwrap();

    if root.left.is_none() && root.right.is_none() {
            huff_code.insert(root.char, str.clone());
    }

    encode(Option::from(&root.left), str.clone() + "0", huff_code);
    encode(Option::from(&root.right), str.clone() + "1", huff_code);

    huff_code.clone()
}

fn decode(root: Option<&Box<Node>>, mut index:i32, str: String) -> i32{
    if root.is_none(){
        return index;
    }
    let root = root.unwrap();

    if root.left.is_none() && root.right.is_none(){
        println!("{:?}",&root.char);
        return index;
    }

    index  += 1;

    if str.as_bytes()[index as usize] as char == '0'{
       index = decode(Option::from(&root.left), index, str);
    }
    else {
        index = decode(Option::from(&root.right), index, str);
    }

    index

}

pub fn build_tree(text:String){
    let mut freq:HashMap<char, i32> = HashMap::new();
    let mut i = 0usize;
    for ch in  text.chars(){
        freq.insert(ch, post_increment!(i));
    }


   let mut pq: BinaryHeap<Box<Node>>
       = BinaryHeap::new();

    for pair in freq{
        pq.push(Box::new(Node::make_node(pair.0, pair.1,None,None)));
    }

    while pq.len() != 1{
        let left:Option<Box<Node>>= pq.pop();
        let right:Option<Box<Node>> = pq.pop();
        let sum:i32 = left.as_ref().unwrap().freq + right.as_ref().unwrap().freq;
        pq.push(Box::new(Node::make_node(
            '0',
            sum,
            left,
            right
        )));
    }

    let root:Option<&Box<Node>> = pq.peek();

    let mut huff_code:HashMap<char, String> = HashMap::new();
    huff_code = encode(root.clone(), "".parse().unwrap(), &mut huff_code.clone());


    println!("Huffman codes are: \n");
    for pair in huff_code.clone(){
        println!("{:?}, ' ' {:?}",pair.0, pair.1);
    }
    println!("\nOriginal string:\n {}",text);

    println!("\nEncoded string is \n");
    let mut str:String = "".to_string();
    for ch in text.chars(){
        str += &*huff_code[&ch];

    }
    println!("{}",str);


    println!("\nDecoded string is \n");
    let mut index = -1;

    while index < (str.len()-2) as i32 {
        index = decode(root,index ,str.clone());
    }

}