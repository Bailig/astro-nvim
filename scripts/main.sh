
#!/bin/bash

if [[ $(uname -s) != "Darwin" ]]; then
    echo "Only works on macOS"
    exit 1
fi

function is_homebrew_installed {
    if command -v brew >/dev/null 2>&1; then
        return 0
    else
        return 1
    fi
}

function install_homebrew {
    /bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
}

function install_neovim {
    brew install neovim
}

function install_astronvim {
    git clone --depth 1 https://github.com/AstroNvim/AstroNvim ~/.config/nvim
}

function install_astronvim_config {
    git clone --depth 1 git@github.com:Bailig/astronvim-config.git ~/.config/nvim/lua/user
}

function install_nerd_fonts {
    brew tap homebrew/cask-fonts
    brew install --cask font-hack-nerd-font
}

if ! is_homebrew_installed; then
    read -p "Do you want to install Homebrew? (y/n): " choice
    case "$choice" in
        [yY])
            install_homebrew
            ;;
        *)
            exit 0
            ;;
    esac
fi

echo "Homebrew is installed"
echo "Installing neovim..."
install_neovim
echo "Installing astronvim..."
install_astronvim
echo "Installing astronvim config..."
install_astronvim_config
echo "Installing nerd fonts..."
install_nerd_fonts
