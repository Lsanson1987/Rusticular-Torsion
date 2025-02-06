# Dungeon Bus

A **D&D Omnibus** built in Rust, designed for tracking and assisting you throughout any D&D adventure.

🏆 **Hall of Fame**  
This project was recognized for its innovation and excellence in the **2024 Fall Semester Hall of Fame**. Learn more about the recognition [here](https://example.com/hall-of-fame).

---

## 🛠️ Version 1  
This is the first release of Dungeon Bus, packed with essential features to streamline your D&D sessions. Stay tuned for future updates with even more functionality!

---

## ✅ Current Features

- **Initiative Tracking**: Sort the turn order of various monsters and players based on their initiative.
- **Autofill Monster Data**: Automatically fetch monster stats like hit points, armor class, initiative roll, and name from a backend database for quick setup.
- **Dice Roller for Initiative**: Roll for initiative directly within the app and apply modifiers automatically.
- **Combat Timer**: Track combat duration to keep your sessions organized and on schedule.
- **Desktop App**: A standalone desktop application for seamless offline use.

---

## 🚀 Planned Features

- **General Dice Roller**: A versatile dice roller for non-initiative rolls to handle any in-game checks or actions.
- **Website Frontend**: A web-based interface for users to interact with the tracker easily from any device.

---

## 🛠️ Getting Started

### Prerequisites

Make sure you have the following installed:

- **Rust**: Install using [rustup](https://rustup.rs/) or follow the directions on the [Rust website](https://www.rust-lang.org/tools/install).

---

### Installation

1. Clone the repository:

   ```bash
   git clone https://github.com/Lsanson1987/Rusticular-Torsion.git
   ```

2. Navigate to the project directory:

   ```bash
   cd Rusticular-Torsion
   ```

3. Build the project using Cargo:

   ```bash
   cargo build
   ```

4. Run the application using Cargo:

   ```bash
   cargo run
   ```

---

## 🔍 Process

### Data Processing

- **Data Source**: The monster data was sourced from [AideDD](https://www.aidedd.org/dnd-filters/monsters.php). Using browser developer tools, a text file containing all monsters and their stats was extracted.
- **Conversion**: A Python script was used to parse the text file, isolate relevant information (e.g., name, armor class, and initiative modifier), and export it as a CSV file.
- **HashMap Integration**: The Rust program reads the CSV file to populate a `HashMap`, where the key is the monster name (a `String`) and the value is a `Monster` struct containing its attributes.

### Dice Rolling

- **Initiative Rolls**: When a monster is added from the database, its initiative is determined by rolling dice and applying its initiative modifier.

### Monster Struct

The `Monster` struct is used to store and manipulate monster data. Attributes include:

- `name` (String): The monster’s name.
- `hp` (u32): The monster’s hit points.
- `ac` (u32): The monster’s armor class.
- `initiative` (i32): The monster’s initiative score.
- `notes` (String): Additional information or notes.

---

## 🙏 Special Thanks

This project wouldn't have been possible without the incredible database provided by [AideDD](https://www.aidedd.org/dnd-filters/monsters.php). Their extensive collection of D&D monster stats allowed us to bring Dungeon Bus to life. Thank you for your contribution to the D&D community!

---

## License

This project is licensed under the MIT License. See the `LICENSE` file for details.
