# Tiny Hacking Puzzle

This is a small program intended as part of a paper chase.

## Setup

After compiling and running the rust program, type in a username and a password, and click the "Add user" button. The password only accepts digits as input, to make cracking easier later on. The username should therefore add some semantics to the number. Something like "You will need this number in Puzzle 31A", for example.

Send someone the executable, the file database.db, and, depending on hacking experience, probably the hints/ folder. They can now start to abuse the "Search for users" functionality to extract the password hash, and then write a script to bruteforce it.

The solution to the puzzle is contained in puzzle_solution/.
