pub mod huffman {
    use std::boxed::Box;
    use std::cmp::Ordering;
    use std::collections::*;

    // Node is a binary tree data structure
    // It will be used by huffman compression algorithm
    #[derive(Clone, PartialEq, Eq, Ord, std::fmt::Debug)]
    struct Node {
        color: Option<u8>,
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
        fn new (color:Option<u8>, freq:i32)->Node {
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
        let mut color_cnt: Vec<u8> = stream_vec;
        color_cnt.sort();
        let mut freq = 0;
        // println!("{:?}",color_cnt);
        
        let mut prev: u8 = color_cnt[0];
        for c in color_cnt {
            if c == prev {
                freq += 1;
            } else {
                freq_vec.push(Node::new(Option::Some(prev), freq));
                freq = 1;
                prev = c;
            }
        }
        // println!{"{:?}\n",freq_vec};

        freq_vec
    }
    fn construct_huffman_tree(freq: Vec<Node>)->Node{
        let mut pq = BinaryHeap::new();
        for node in freq {
            pq.push(node);
        }
        while pq.len()>1{
            let (a,b)=(pq.pop().unwrap(), pq.pop().unwrap());
            let new_node = Node {
                color: None,
                freq: a.freq + b.freq,
                left: Option::from(Box::from(a)),
                right: Option::from(Box::from(b)),
            };
            pq.push(new_node);
        }
        pq.pop().unwrap()
    }

    pub fn compress(stream_vec: Vec<u8>)->Vec<u8>{
        let frequency = freq_count(stream_vec);
        let mut compressed_data = Vec::new();
        let huffman_tree = construct_huffman_tree(frequency);


        compressed_data
    }
}