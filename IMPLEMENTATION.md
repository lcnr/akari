# Implementation

How do/should certain parts of this game work?

## Environment

The world consists of 16x16 chunks.
The 9 chunks surrounding the player are always loaded.
A registry of existing chunks is stored as a `Hashmap<(i32, i32), String>`.
Chunk data is only loaded once the player is close to it. 

The camera follows the player unless he is close to the border of a nonexisting chunk, 
in which case the camera stays in a position where only the existing chunks an be seen.
This is currently solved by simply given both cameras and non existing chunks a hitbox.
