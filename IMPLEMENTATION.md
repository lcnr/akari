# Implementation

How do/should certain parts of this game work?

## Environment

The world consists of 16x16 chunks.
The 9 chunks surrounding the player are always loaded.
A registry of existing chunks is always loaded as a `Hashmap<(u16, u16), String>`.
The chunk data is only loaded once the player is close to it. 

The camera follows the player unless he is close to the border of a nonexisting chunk, 
in which case the camera staysin a position where only the existing chunks an be seen.