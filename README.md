<div align="center">

<h1>goatOS</h1>
A declarative system configuration manager.

</div>

## Example
The `goat` ecosystem is similar to that of NixOS with several important differences.

The following language is starlark. It's essentially embeddable python.

```python
hostname = "goatmachine"

# pkgs is a builtin function provided
# by goat.
packages = pkgs([
  "fastfetch",
  "git"
])

```

## Features

This list is meant to keep track of the status of features we are going to add.

- [ ] Automatic system healing
  - Because `goat` is a wrapper over your ENTIRE operating system,
    it is critical that it checks the vitality of the system. This
    means that several directories will be checked and restored and
    backed up every time `goat` is executed.
  - [X] Filesystem structure check
  - [ ] Restoration of backups on command
  - [ ] Automatic backup of `goat` controlled directories (like /home/user/.config)
- [X] Cache
- [ ] Package management
  - Just like NixOS, it is frowned upon to install packages manually
    as it directly ignores the declarative features of NixOS. We have
    to mask the systems normal package manager behind a warning screen
    and handle all package installations.
  - [X] Modular package manager declarations
    - For custom package manager declarations
  - [X] Package manager detection
  - [ ] Package manager masking

LOTS more is planned but this is just the groundwork for now.

## Authors

This project is created and maintained by me ([Lucas Marta](https://github.com/gloggers99)) and [William Chastain](https://github.com/crazywillbear).
