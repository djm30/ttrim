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

# Trying to update path variable
$existingPath = [Environment]::GetEnvironmentVariable("Path", "User")
$folderPath = Join-Path $env:LOCALAPPDATA "ttrim"

if ($existingPath -split ";" -contains $folderPath) {
    Write-Host "Folder path already exists in the User Environment Path variable."
    exit
}

$newPath = "$existingPath;$folderPath"
[Environment]::SetEnvironmentVariable("Path", $newPath, "User")


# Go up one level in the directory structure
cd ..

# Remove the cloned repository
Remove-Item -Recurse -Force .\ttrim

# Print success message
Write-Output "Installation complete!"
