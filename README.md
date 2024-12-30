# Arbitary Depth Tree Data Structure
The above implementation is Rust code to build a tree of arbitrary depth where the number of branches per level equals the integer sequence in the expansion of the irrational number Pi.

**Problem Statement**
Consider the number Pi (3.14159265...) and exclude the first number 3

● The tree generation algorithm will have
🌕 1 node in the first level with ID:1
🌕 4 nodes in the second level : IDs: [2,3,4,5]
🌕 1 node in the third level, and so on

● In every node that belongs to the same layer, have an integer that represents the
identifier for the node which increases linearly as the nodes are being added

● Final output should traverse the tree to retrieve the integers per node

● Dump each of the nodes (serialized) to the disk such that you can retrieve child nodes if
a parent node is picked up. Serialization implies you don’t create human readable data,
rather program readable data in the same data structure it’s initialized before dumping.

**Installation**
1. ```git clone https://github.com/Daksh-10/RustedTree.git```
2. Change directory to RustedTree ```cd RustedTree```
3. To build the program ```cargo build```
4. To run the program ```cargo run```

**Visualize**

![image](https://github.com/user-attachments/assets/fcc71b07-95ae-4668-be99-136589d39c08)

**Output**

![image](https://github.com/user-attachments/assets/f2185923-d7f9-4668-ad1a-bd70d7747e2f)
