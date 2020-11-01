pub mod huffman {
    use std::boxed::Box;
    use std::cmp::Ordering;
    use std::collections::*;
    use std::collections::HashMap;

    // Node is a binary tree data structure
    // It will be used by huffman compression algorithm
    #[derive(Clone, PartialEq, Eq, Ord, std::fmt::Debug)]
    struct Node {
        color: u8,
        freq: i32,
        is_leaf: bool,
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
        fn new (color:u8, freq:i32, is_leaf:bool)->Node {
            Node {
                color,
                freq, 
                is_leaf,
                left:None,
                right:None,
            }
        }
    }
    fn freq_count(stream_vec: &Vec<u8>)->Vec<Node> {
        let mut freq_vec = Vec::new();
        let mut color_cnt: Vec<u8> = stream_vec.to_vec();
        color_cnt.sort();
        let mut freq = 0;
        
        let mut prev: u8 = color_cnt[0];
        for c in color_cnt {
            if c == prev {
                freq += 1;
            } else {
                freq_vec.push(Node::new(prev, freq, true));
                freq = 1;
                prev = c;
            }
        }
        freq_vec.push(Node::new(prev, freq, true));

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
                color: 0,
                freq: a.freq + b.freq,
                is_leaf: false,
                left: Option::from(Box::from(a)),
                right: Option::from(Box::from(b)),
            };
            pq.push(new_node);
        }
        pq.pop().unwrap()
    }

    // convert huffman tree to hashmap
    fn to_hashmap(node: &Node)->HashMap<u8,String> {
        let mut hm = HashMap::new();
        // // Huffman tree is complete binary tree
        // // i.e. node will have 0 or 2 children, 0 is not possible
        if node.left.is_none(){
            hm.insert(node.color,"0".to_string());
            return hm;
        }

        fn encode(hm:&mut HashMap<u8,String>, node:&Node, encoding: String){
            if node.is_leaf == true {
                hm.insert(node.color, String::from(&encoding));
            } else {
                let left_path = String::from(&encoding)+"0";
                let right_path = String::from(&encoding)+"1";
                if let Some(left) = &node.left {
                    encode(hm,&left,left_path);
                }
                if let Some(right) = &node.right {
                    encode(hm,&right,right_path);
                }
            };
        }
        encode(&mut hm, &node, "".to_string());
        return hm;
    }
    // convert huffman node to Vec of u8 using post-order traversal
    fn to_vec(huffman_node: &Node)->Vec<u8> {
        let mut output = Vec::new();
        fn post_order(node:&Node, output_vec:&mut Vec<u8>){
            if let Some(left) = &node.left {
               post_order(left.as_ref(), output_vec);
            }
            if let Some(right) = &node.right {
               post_order(right.as_ref(), output_vec);
            }
            output_vec.push(node.color);
       }
        post_order(huffman_node, &mut output);
        return output;
    }
    // convert huffman tree to vector of bytes
    // the first element is length of tree
    fn embed_tree(huffman_node:&Node)->Vec<u8>{
        let mut compressed_data = to_vec(huffman_node);
        compressed_data.insert(0, compressed_data.len() as u8);
        compressed_data
    }
    // map input color into corresponding codewords
    // the first element is padding(the number of zeros appended at the last for u8)
    fn compress_data(byte_stream: &Vec<u8>, huffman_node: &Node)->Vec<u8> {
        let mut out_byte_stream: Vec<u8> = Vec::new();
        let (mut byte, mut count) = (0,0);

        let huffman_map = to_hashmap(huffman_node);
        // println!{"hashmap:"}
        // println!("{:?}", huffman_map);

        for c in byte_stream {
            let encoding = huffman_map.get(c).unwrap();
            for e in encoding.bytes(){
                let bit: bool = (e-'0' as u8) != 0;
                byte = byte << 1 | (bit as u8);
                count = (count+1) % 8;
                if count == 0{
                    out_byte_stream.push(byte);
                    byte = 0;
                } 
            }
        }
        if count != 0 {
            let padding:u8 = 8 - count;
            byte <<= padding;
            out_byte_stream.push(byte);
            out_byte_stream.insert(0,padding);
        } else {
            out_byte_stream.insert(0,0);
        }
        out_byte_stream
    }


    pub fn compress(stream_vec: &Vec<u8>)->Vec<u8>{
        let frequency = freq_count(stream_vec);
        let huffman_tree = construct_huffman_tree(frequency);
        let mut compressed_data = Vec::from(embed_tree(&huffman_tree));

        compressed_data.extend(compress_data(stream_vec,&huffman_tree));

        compressed_data
    }
}