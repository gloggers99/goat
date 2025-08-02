<div align="center">

```lua

                                               # ###       #######    
                                             /  /###     /       ###  
                                   #        /  /  ###   /         ##  
                                  ##       /  ##   ###  ##        #   
                                  ##      /  ###    ###  ###          
    /###      /###     /###     ######## ##   ##     ## ## ###        
   /  ###  / / ###  / / ###  / ########  ##   ##     ##  ### ###      
  /    ###/ /   ###/ /   ###/     ##     ##   ##     ##    ### ###    
 ##     ## ##    ## ##    ##      ##     ##   ##     ##      ### /##  
 ##     ## ##    ## ##    ##      ##     ##   ##     ##        #/ /## 
 ##     ## ##    ## ##    ##      ##      ##  ##     ##         #/ ## 
 ##     ## ##    ## ##    ##      ##       ## #      /           # /  
 ##     ## ##    ## ##    /#      ##        ###     /  /##        /   
  ########  ######   ####/ ##     ##         ######/  /  ########/    
    ### ###  ####     ###   ##     ##          ###   /     #####      
         ###                                         |                
   ####   ###                                         \)              
 /######  /#                                                          
/     ###/                                                            
by Lucas Marta and William Chastain
```

A meta-distribution with a declarative system configuration.

</div>

<!-- TOC -->
  * [What](#what)
  * [Why](#why)
  * [Example](#example)
  * [Features](#features)
  * [Contributing](#contributing)
  * [Authors](#authors)
<!-- TOC -->

## What

`goat` is a declarative system configuration manager, similar to NixOS. 
However `goat` is a sort of "meta-distribution". This means you can combine 
it with your current operating system!

Q: "What distributions are supported?"  
A: EVERY Linux based operating system should be supported*

Q: "Do I need to have a fresh distro install?"  
A: Nope. During `goat`'s installation it will compile your current 
distro settings like your manually installed packages, running services, users, and more.


\* non-supported distros need a package manager configuration interface created.
This is a very simple process.

## Why

`goat` and its tooling is what I originally wanted to see from NixOS. The 
declarative configuration file was the best idea I've seen from any Linux
distro yet. At this point 99% of distros are the same thing with a different 
package manager (pacman, portage, apt, dnf, etc.) or preinstalled DE's. The 
only difference is preference, unlike NixOS which had a genuinely creative 
difference from everything else.

However, when my configuration reached a certain point it was just tiring to
deal with. I don't want to figure out how to use the experimental flakes, or
wait upwards of 5 minutes to rebuild my system! Or look through several websites
to find out a specific config option or use the nix repl... 

The point of this "essay" isn't to de-value NixOS but to highlight the fantastic
idea of a declarative system configuration. That is exactly what `goat` attempts
to replicate without all the baggage and inconvenience of NixOS. (sorry NixOS
lovers)

~ Lucas Marta

## Example
The `goat` ecosystem is similar to that of NixOS with several important differences.

The following is lua.

```lua
-- This is the default hostname 
-- if no hostname global is set.
hostname = "goatOS"

packages = {
  "fastfetch",
  "git"
}
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

## Contributing

We are very strict (not really) on our Rust code:

- NO `.unwrap()`'s on any pull requests. The only time you should be using
unwrap is for testing code.
- Use `anyhow::Result` for result types. use `.map_err(...)` for conflicting
types.

That's pretty much it. Just keep your code clean and try to leave docstrings
as much as possible.

## Authors

This project is created and maintained by me ([Lucas Marta](https://github.com/gloggers99)) and [William Chastain](https://github.com/crazywillbear).
