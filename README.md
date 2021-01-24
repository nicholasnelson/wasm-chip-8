# CHIP-8


## Hello World
```
        0xF1, 0x29, // Load address for sprite "1" to I
        0xD0, 0x15, // Draw "1" to v0,v1
        0xF2, 0x29, // Load address for sprite "2" to I
        0x60, 0x06, // Set v0 to 6
        0xD0, 0x15, // Draw "2" to v0,v1
        0xF3, 0x29, // Load address for sprite "3" to I
        0x60, 0x0C, // Set v0 to 12
        0xD0, 0x15, // Draw "1" to v0,v1
```