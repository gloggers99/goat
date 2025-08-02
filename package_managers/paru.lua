binary_name = "paru"

install_command = "paru -S {}"
full_system_update_command = "paru -Syu"
list_explicit_packages = "paru -Qe | cut -d ' ' -f1"