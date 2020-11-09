# rusty-dpcm-huff

  
## About 

The code is for experimenting DPCM huffman compression with some raw data images.  
(ie. Baboon and lena in raw,halftone and binary color)

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

Due to the lack of the time, now I only focus on the experiment of different ways of huffman coding. Therefore **Decoder is yet not fully workable** 

The basic decoder concept is the same as my previous project: [rusty-huffman-unicode](https://github.com/yppan/rusty-huffman-unicode/)" You can check detail implementation there if you want.  

## Experiment statistic

Baboon 
  
![Baboon](https://github.com/yppan/rusty-classic-huffman-for-img/blob/main/Data/PNG/baboon.png)

| Image(256*256)  | Entropy(bit) | Before(byte) | After(byte) -without header | Header(byte) | Compression Rate |
|-----------------|--------------|--------------|-----------------------------|--------------|------------------|
| baboon_b        | 0.78913      | 65536        | 9498                        | 6            | 85.51%           |
| baboon_halftone | 1.55381      | 65536        | 12842                       | 6            | 80.40%           |
| baboon_raw      | 6.36163      | 65536        | 52788                       | 208          | 19.45%           |

Lena 
  
![Lena](https://github.com/yppan/rusty-classic-huffman-for-img/blob/main/Data/PNG/lena.png)

| Image(256*256)  | Entropy(bit) | Before(byte) | After(byte) -without header | Header(byte) | Compression Rate |
|-----------------|--------------|--------------|-----------------------------|--------------|------------------|
| lena_b          | 0.47269      | 65536        | 8838                        | 6            | 86.51%           |
| baboon_halftone | 1.34307      | 65536        | 11337                       | 6            | 82.70%           |
| baboon_raw      | 5.87696      | 65536        | 48945                       | 212          | 25.31%            |
