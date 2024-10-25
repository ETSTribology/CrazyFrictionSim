#!/bin/bash

# Exit the script if any command fails
set -e

# Function to check if a command exists
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

echo "Starting installation of Rust, Cargo, Nix, Fish shell, CUDA 12.6, OpenMPI, and other utilities..."

# Step 1: Install Rust and Cargo using rustup
if command_exists rustc && command_exists cargo; then
    echo "Rust and Cargo are already installed!"
else
    echo "Installing Rust and Cargo..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source $HOME/.cargo/env
    echo "Rust and Cargo installed successfully!"
fi

# Add Cargo to the profile (bash and fish)
if ! grep -q 'source $HOME/.cargo/env' ~/.bashrc; then
    echo 'source $HOME/.cargo/env' >> ~/.bashrc
fi

if command_exists fish; then
    if ! grep -q 'source $HOME/.cargo/env' ~/.config/fish/config.fish; then
        echo 'source $HOME/.cargo/env' >> ~/.config/fish/config.fish
    fi
fi

# Step 2: Install Nix
if command_exists nix; then
    echo "Nix is already installed!"
else
    echo "Installing Nix..."
    curl -L https://nixos.org/nix/install | sh

    # Source the Nix profile
    . ~/.nix-profile/etc/profile.d/nix.sh

    echo "Nix installed successfully!"
fi

# Step 3: Install Fish shell
if command_exists fish; then
    echo "Fish shell is already installed!"
else
    echo "Installing Fish shell..."
    # Install Fish shell depending on the OS
    if [[ "$OSTYPE" == "linux-gnu"* ]]; then
        sudo apt-get update
        sudo apt-get install -y fish
    elif [[ "$OSTYPE" == "darwin"* ]]; then
        brew install fish
    else
        echo "Unsupported OS. Please install Fish manually."
        exit 1
    fi
    echo "Fish shell installed successfully!"
fi

# Step 4: Set Fish as the default shell (optional)
read -p "Do you want to set Fish as your default shell? (y/n): " set_fish
if [ "$set_fish" = "y" ]; then
    echo "Setting Fish as your default shell..."
    chsh -s $(which fish)
    echo "Fish shell is now your default shell."
else
    echo "Fish shell installation is complete, but not set as default."
fi

# Step 5: Install CUDA 12.6
if command_exists nvcc; then
    echo "CUDA is already installed!"
else
    echo "Installing CUDA 12.6..."
    if [[ "$OSTYPE" == "linux-gnu"* ]]; then
        # Adding NVIDIA package repositories for Ubuntu or other Debian-based distros
        sudo apt-get update && sudo apt-get install -y gnupg
        wget https://developer.download.nvidia.com/compute/cuda/repos/ubuntu2004/x86_64/cuda-ubuntu2004.pin
        sudo mv cuda-ubuntu2004.pin /etc/apt/preferences.d/cuda-repository-pin-600
        sudo apt-key adv --fetch-keys https://developer.download.nvidia.com/compute/cuda/repos/ubuntu2004/x86_64/3bf863cc.pub
        sudo add-apt-repository "deb https://developer.download.nvidia.com/compute/cuda/repos/ubuntu2004/x86_64/ /"
        sudo apt-get update
        sudo apt-get -y install cuda-12-6
        
        # Set environment variables for CUDA
        echo "export PATH=/usr/local/cuda-12.6/bin${PATH:+:${PATH}}" >> ~/.bashrc
        echo "export LD_LIBRARY_PATH=/usr/local/cuda-12.6/lib64${LD_LIBRARY_PATH:+:${LD_LIBRARY_PATH}}" >> ~/.bashrc
        source ~/.bashrc

    elif [[ "$OSTYPE" == "darwin"* ]]; then
        echo "CUDA installation on macOS is not officially supported. You may need to use alternatives like ROCm or Metal for GPU computing."
        exit 1
    else
        echo "Unsupported OS. Please install CUDA manually."
        exit 1
    fi
    echo "CUDA 12.6 installed successfully!"
fi

# Step 6: Install OpenMPI
if command_exists mpicc; then
    echo "OpenMPI is already installed!"
else
    echo "Installing OpenMPI..."
    if [[ "$OSTYPE" == "linux-gnu"* ]]; then
        sudo apt-get update
        sudo apt-get install -y openmpi-bin openmpi-common libopenmpi-dev
    elif [[ "$OSTYPE" == "darwin"* ]]; then
        brew install open-mpi
    else
        echo "Unsupported OS. Please install OpenMPI manually."
        exit 1
    fi
    echo "OpenMPI installed successfully!"
fi

# Step 7: Install Vim, curl, wget, nmap, and Docker
echo "Installing Vim, curl, wget, nmap, and Docker..."

if command_exists vim; then
    echo "Vim is already installed!"
else
    sudo apt-get install -y vim
fi

if command_exists curl; then
    echo "curl is already installed!"
else
    sudo apt-get install -y curl
fi

if command_exists wget; then
    echo "wget is already installed!"
else
    sudo apt-get install -y wget
fi

if command_exists nmap; then
    echo "nmap is already installed!"
else
    sudo apt-get install -y nmap
fi

if command_exists docker; then
    echo "Docker is already installed!"
else
    echo "Installing Docker..."
    sudo apt-get update
    sudo apt-get install -y apt-transport-https ca-certificates curl software-properties-common
    curl -fsSL https://download.docker.com/linux/ubuntu/gpg | sudo apt-key add -
    sudo add-apt-repository "deb [arch=amd64] https://download.docker.com/linux/ubuntu $(lsb_release -cs) stable"
    sudo apt-get update
    sudo apt-get install -y docker-ce
    sudo usermod -aG docker ${USER}
    echo "Docker installed successfully! You may need to log out and log back in for Docker to work."
fi

# Step 8: Verify installations
echo "Verifying installations..."

if command_exists rustc; then
    echo "Rust version: $(rustc --version)"
else
    echo "Rust installation failed."
fi

if command_exists cargo; then
    echo "Cargo version: $(cargo --version)"
else
    echo "Cargo installation failed."
fi

if command_exists nix; then
    echo "Nix version: $(nix --version)"
else
    echo "Nix installation failed."
fi

if command_exists fish; then
    echo "Fish version: $(fish --version)"
else
    echo "Fish installation failed."
fi

if command_exists nvcc; then
    echo "CUDA version: $(nvcc --version)"
else
    echo "CUDA installation failed."
fi

if command_exists mpicc; then
    echo "OpenMPI version: $(mpicc --version)"
else
    echo "OpenMPI installation failed."
fi

if command_exists vim; then
    echo "Vim installed successfully."
else
    echo "Vim installation failed."
fi

if command_exists curl; then
    echo "curl installed successfully."
else
    echo "curl installation failed."
fi

if command_exists wget; then
    echo "wget installed successfully."
else
    echo "wget installation failed."
fi

if command_exists nmap; then
    echo "nmap installed successfully."
else
    echo "nmap installation failed."
fi

if command_exists docker; then
    echo "Docker installed successfully."
else
    echo "Docker installation failed."
fi

echo "All installations completed successfully!"
