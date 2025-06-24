# Adversarial Multi-Agent Path Finding

A Rust implementation of Adversarial Multi-Agent Path Finding (ACPF) algorithms and environment for comparing different AI approaches.

## Overview

This project provides a framework for solving Multi-Agent Path Finding problems, where multiple agents need to navigate from their starting positions to goal positions while avoiding obstacles and collisions. The project currently includes:

- A grid-based MAPF environment
- Multiple AI implementations with varying levels of sophistication
- Utilities for evaluating and comparing AI performance

## Installation

### Prerequisites

- Rust 1.81.0 or later
- Cargo (Rust's package manager)

### Building the Project

```bash
git clone git@gitlab.fit.cvut.cz:hajekric/acpf.git
cd acpf
cargo build
```

## Usage

Run the main example:

```bash
cargo run
```

This will execute a sample ACPF scenario with multiple AI agents.

## Project Structure

### Core Components

- [Main Entry Point](src/main.rs) - The application entry point that sets up and runs a sample MAPF scenario
- [ACPF Environment](src/mapf/environment.rs) - Defines the grid-based environment where agents operate
- [ACPF State](src/mapf/state.rs) - Represents the state of the environment at a given point in time
- [ACPF Actions](src/mapf/action.rs) - Defines the possible actions agents can take

### AI Implementations

- [AI Trait](src/ai/mod.rs) - The interface that all AI implementations must follow
- [Random AI](src/ai/random_ai.rs) - A simple AI that makes random moves (baseline)
- [Greedy AI](src/ai/greedy.rs) - An AI that uses a distance heuristic to move toward goals
- [MCTS AI](src/ai/mcts.rs) - A sophisticated AI using Monte Carlo Tree Search for decision making

### Utilities

- [Evaluation Loops](src/loops.rs) - Functions for evaluating and comparing AI performance

## Maps

Maps in the `maps/` have the following structure:

- `.` - Empty space
- `#` - Obstacle
- `1-9` - Starting position for player 1-9
- `A-Z` - Goal position (A for player 1, B for player 2, etc.)

## TODO

ACPF and MAPF are so far in this project used interchably, to mean exclusively ACPF. TODO: fix.