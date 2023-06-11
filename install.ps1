# PowerShell script for Windows

# Clone the repository
git clone https://github.com/djm30/ttrim.git

# Go into the cloned directory
cd .\ttrim

# Build the project
cargo build --release

# Move the binary to a location of your choice. For example, C:\Program Files
Move-Item -Path .\target\release\ttrim.exe -Destination "C:\Program Files"

# Go up one level in the directory structure
cd ..

# Remove the cloned repository
Remove-Item -Recurse -Force .\ttrim

# Print success message
Write-Output "Installation complete!"
