File structure of .rssheet file type (Raw Sprite Sheet) - Bytes storage
Uses Little Endian Format for u32

```
// Image Data Header (ByteSize(9))
- u8  : Image Data Version Number
- u32 : Width of Image
- u32 : Height of Image
// Image Data (Width * Height * 4)
- [;;For each pixel (Width * Height):
  - u8  : Red Channel
  - u8  : Green Channel
  - u8  : Blue Channel
  - u8  : Alpha Channel
  ]

// SpriteSheet MetaData Header (ByteSize(5))
- u8  : SpriteSheet MetaData Version Number
- u32 : Number of Hashmap Entries
// SpriteSheet MetaData (ByteSize(StringByteLength))
- [;;For each hashmap entry:
  - u32 : String Byte Length
  // String Data (ByteSize(StringByteLength))
  - [;;For each byte in string (String Length):
    - u8 : String Byte
    ]

  - u32 : X Coordinate
  - u32 : Y Coordinate
  - u32 : Width
  - u32 : Height
  ]
```

File Structure of .ssheetmeta file type (Sprite Sheet Meta Data) - Raw bytes storing mappings
Uses Little Endian Format for u32

```
// SpriteSheet MetaData Header (ByteSize(5))
- u8  : Version Number
- u32 : Number of Hashmap Entries
// Hashmap Data
- [;;For each hashmap entry:
  - u32 : String Byte Length
  // String Data (ByteSize(StringByteLength))
  - [;;For each byte in string (String Length):
    - u8 : String Byte
    ]

  - u32 : X Coordinate
  - u32 : Y Coordinate
  - u32 : Width
  - u32 : Height
  ]
```
