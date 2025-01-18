# Deck Project

## Overview

The Deck Project is a simple Rust application that simulates a deck of playing cards. It provides functionality to create a new deck, shuffle the cards, and deal a specified number of cards.

## Features

- **Create a New Deck**: Initializes a deck with standard playing cards.
- **Shuffle the Deck**: Randomizes the order of the cards in the deck.
- **Deal Cards**: Removes a specified number of cards from the deck and returns them.

## Getting Started

### Prerequisites

- Rust (version 1.50 or later) installed on your machine. You can install Rust by following the instructions at [rust-lang.org](https://www.rust-lang.org/tools/install).

### Installation

1. Clone the repository:

   ```bash
   git clone https://github.com/yourusername/deck-project.git
   cd deck-project
   ```
2. Build the project:

   ```bash
   cargo build
   ```

### Usage

To run the application, use the following command:

bash
cargo run

This will create a new deck, shuffle it, and deal three cards, displaying the hand and remaining cards in the console.

## Code Structure

- `src/main.rs`: Contains the main logic for the deck functionality, including the `Deck` struct and the implementation of the `DeckFunctionality` trait.
