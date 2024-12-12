# Dungeon Bus

A **D&D Omnibus** built in Rust, designed for tracking and assisting you throughout any D&D adventure.

🚧 **Work in Progress**  
This project is in the early stages of development. Many features are planned but have not been fully implemented yet. Contributions, ideas, and feedback are welcome!

## Planned Features

- **Initiative Tracking**: Sort the turn order of various monsters and players based on their initiative.
- **Autofill Monster Data**: Automatically fetch monster stats like hit points, armor class, initiative roll, and name from a backend database for quick setup.
- **Dice Roller for Initiative**: Roll for initiative directly within the app and apply modifiers automatically.
- **General Dice Roller**: A versatile dice roller for non-initiative rolls to handle any in-game checks or actions.
- **Website Frontend**: A web-based interface for users to interact with the tracker easily from any device.

## Getting Started

## Process

//data 

Database utilized both Rust, Python, and Hashmap. The data was sourced from https://www.aidedd.org/dnd-filters/monsters.php in which using inspect elements we were able to obtain a txt file with all the monsters in dnd along with their stats. However, the file needed to be converted into a csv so that it could be plugged into a Hashmap that contains a string and Monster struct. Therefore, using python allowed us to open the txt file and read line by line concatenate each string to ensure that only the name, ac, and int remained so that a csv file could be filled with those isolated values. With the data in a readable format within the main.rs we are now able to fill a hashmap with the key as the string name of the monster and a value of a monster type. With this hashmap every time a name is inputted into the code the program will run through each key in the hashmap to find if the name matches any key in the database.

//dice

When the dice function is called it will do the set dnd dexterity to initiative conversion and add that value to the sum of all dice rolls. The number of dice rolls can be determined by the use and the number of sides as well. This is implemented in the project as when a monster is added from the database it will be given a random initiative with its initiative modifier. 

//monster struct

The monster struct is how the data from the database is stored and utilized. The user is able to store a name, int, hp, ac, and notes about a monster. As well with the getters they are able to call those values to display or modify with the setters. With further implementation a user can track hp changes with functions associated with the class.


### Prerequisites

- **Rust** (Install using [rustup](https://rustup.rs/) or follow the directions on the [rust language website](https://www.rust-lang.org/tools/install))

### Installation

1. Clone the repository:

   ```bash
   git clone https://github.com/Lsanson1987/Rusticular-Torsion.git

3. Build using Cargo:

   ```bash
   Cargo build

4. Run using Cargo:

   ```bash
   Cargo run
