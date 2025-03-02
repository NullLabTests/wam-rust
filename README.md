# Warren Abstract Machine (WAM) in Rust

A Rust implementation of the Warren Abstract Machine, featuring a graphical user interface for step-by-step execution visualization.

## Features

- Basic WAM instruction set implementation
- Register-based architecture
- Step-by-step execution visualization
- Interactive GUI for demonstration
- Error handling using Rust's type system

## Building and Running

1. Make sure you have Rust and Cargo installed
2. Clone the repository
3. Run the following commands:

```bash
cargo build
cargo run
```

## GUI Usage

1. Click "Load Program" to initialize the WAM with a sample program
2. Click "Step" to execute each instruction one at a time
3. Watch the execution progress in the log window
4. Current instruction is highlighted with ">" in the program display

## Implementation Details

The implementation includes:
- Term representation (variables, constants, structures)
- Register handling (permanent and temporary)
- Basic WAM instructions
- Memory areas (heap, trail, environments)
- Instruction execution engine

## Contributing

Feel free to contribute by:
1. Opening issues for bugs or feature requests
2. Submitting pull requests
3. Improving documentation

## License

MIT License
