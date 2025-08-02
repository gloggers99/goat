-- All arch based systems will (hopefully) end up here.

binary_name = "pacman"

install_command = "pacman -S {}"
full_system_update_command = "pacman -Syu"
list_explicit_packages = "pacman -Qe | cut -d ' ' -f1"

-- if paru is installed lets use that instead.
--
-- program_exists() is a function added by goat similar
-- to how neovim has the `vim` functions
if goat.program_exists("paru") then
    binary_name = "paru"

    install_command = "paru -S {}"
    full_system_update_command = "paru -Syu"
    list_explicit_packages = "paru -Qe | cut -d ' ' -f1"
end

core_packages = {
    "base"
}