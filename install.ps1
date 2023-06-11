# PowerShell script for Windows

# Clone the repository
git clone https://github.com/djm30/ttrim.git

# Go into the cloned directory
cd .\ttrim

# Build the project
cargo build --release

# Define the installation path
$installPath = "$env:LOCALAPPDATA\ttrim"

# Create the installation directory if it doesn't exist
if (!(Test-Path -Path $installPath -PathType Container)) {
    New-Item -Path $installPath -ItemType Directory
}

# Move the binary to the installation path
Move-Item -Path .\target\release\ttrim.exe -Destination "$installPath\ttrim.exe"

# Add the installation path to the system PATH if it's not already there
$oldPath = [System.Environment]::GetEnvironmentVariable('Path', 'User')
if ($oldPath -notlike "*$installPath*") {
    $newPath = $oldPath + ";" + $installPath
    [System.Environment]::SetEnvironmentVariable('Path', $newPath, 'User')
}

# Go up one level in the directory structure
cd ..

# Remove the cloned repository
Remove-Item -Recurse -Force .\ttrim

# Print success message
Write-Output "Installation complete!"
