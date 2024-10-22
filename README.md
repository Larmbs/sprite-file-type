This is just a fun project where I create a new file type used for quickly creating spritesheets

It uses the image crate under the hood for maximum compatibility

File structure of .sprite file type
Uses Little Endian Format for u32
```             
                                           Header | Total Bytes
- u8          : Version Number            1 Byte  | 13
- u32         : Width of Image            4 Bytes |
- u32         : Height of Image           4 Bytes |
- u32         : Number of Hashmap Entries 4 Bytes |
- [
    - For each pixel (Width * Height):    
        - u8      : Red Channel
        - u8      : Green Channel
        - u8      : Blue Channel
        - u8      : Alpha Channel
  ]
- [
    - For each hashmap entry:
        - u32      : String Length (in bytes)
        - [
            - For each byte in string (String Length):
                - u8 : String Byte
          ]
        - u32      : X Coordinate
        - u32      : Y Coordinate
        - u32      : Width
        - u32      : Height
  ]
```
