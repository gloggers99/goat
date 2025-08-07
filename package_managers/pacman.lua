binary_name = "pacman"

-- Prefer paru on arch based systems.
-- Extend with more later, paru seems to be the most refined AUR helper available.
if goat.program_exists("paru") then
    binary_name = "paru"
end

install_command = binary_name .. " -S --noconfirm {}"
remove_command = binary_name .. " -Rns --noconfirm {}"
full_system_update_command = binary_name .. " -Syu --noconfirm"
list_explicit_packages_command = binary_name .. " -Qe | cut -d ' ' -f1"
list_all_packages_command = binary_name .. " -Q | cut -d ' ' -f1"

core_packages = {
    "base"
}