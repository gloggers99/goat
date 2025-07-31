binary_name = "pacman"

install_command = "pacman -S {}"
full_system_update_command = "pacman -Syu"
list_explicit_packages = "pacman -Qe | cut -d ' ' -f1"