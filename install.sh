#!/bin/bash

# Clone the repository
git clone https://github.com/djm30/ttrim.git

# Go into the cloned directory
cd ttrim

# Build the project
echo "Building the project..."
cargo build --release

# Move the binary to /usr/local/bin
echo "Moving the binary to /usr/local/bin..."
sudo mv target/release/ttrim /usr/local/bin

# Go up one level in the directory structure
cd ..

# Remove the cloned repository
echo "Deleting the cloned repository..."
rm -rf ttrim

# Print success message
echo "Installation complete!"
