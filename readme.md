# rusty-classic-huffman-for-img
  
## About 

  The code is for experimenting classic huffman compression
with some raw data images. (ie. Baboon and lena in raw, 
halftone and binary color)

## Usage
- Clone directory to local 
```shell
$ git clone https://github.com/yppan/rusty-huffman-unicode/ && cd rusty-huffman-unicode/
```

- Encode(Compress):
```shell
$ cargo run -- -c <FILE>
```

- Decode(Extract): 
```shell
$ cargo run -- -d <FILE>
```
**This will not work now and will be explained below!**

## Future work 

  Due to the lack of the time, now I only focus on the 
experiment of different ways of huffman coding. Therefore 
**Decoder is yet not fully workable** The basic decoder 
concept is the same as my previous project: 
- [rusty-huffman-unicode](https://github.com/yppan/rusty-huffman-unicode/)" 

  You can check detail implementation there if you want.

## Experiment statistic

Baboon
  
| Image(256*256)  | Entropy(bit) | Before(byte) | After(byte) -without header | Header(byte) | Compression Rate |
|-----------------|--------------|--------------|-----------------------------|--------------|------------------|
| baboon_b        | 0.96909      | 65536        | 8197                        | 4            | 87.49%           |
| baboon_halftone | 0.96909      | 65535        | 8197                        | 4            | 87.49%           |
| baboon_raw      | 0.96909      | 65536        | 59939                       | 146          | 8.54%            |

Lena

| Image(256*256)  | Entropy(bit) | Before(byte) | After(byte) -without header | Header(byte) | Compression Rate |
|-----------------|--------------|--------------|-----------------------------|--------------|------------------|
| lena_b          | 0.96968      | 65536        | 8197                        | 4            | 87.49%           |
| baboon_halftone | 0.83681      | 65535        | 8197                        | 4            | 87.49%           |
| baboon_raw      | 7.59536      | 65536        | 62932                       | 214          | 3.97%            |
