pub mod huffman {
    use std::boxed::Box;
    use std::cmp::Ordering;
    use std::collections::*;

    // Node is a binary tree data structure
    // It will be used by huffman compression algorithm
    #[derive(Clone, PartialEq, Eq, Ord, std::fmt::Debug)]
    struct Node {
        color: u8,
        freq: i32,
        left: Option<Box<Node>>,
        right:Option<Box<Node>>,
    }
    impl PartialOrd for Node {
        fn partial_cmp(self: &Node, other:&Node) -> Option<Ordering> {
            let cmp = self.freq.cmp(&other.freq);
            Some(cmp.reverse()) // For min heap
        }
    }
    impl Node {
        // create a leaf node
        fn new (color:u8, freq:i32)->Node {
            Node {
                color,
                freq, 
                left:None,
                right:None,
            }
        }
    }
    fn freq_count(stream_vec: Vec<u8>)->Vec<Node> {
        let mut freq_vec = Vec::new();
    }
}