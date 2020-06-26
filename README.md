# Factorio CLI Mod Manager

[![Coverage Status](https://coveralls.io/repos/github/Scarjit/Factorio-CLI-Mod-manger/badge.svg?branch=master)](https://coveralls.io/github/Scarjit/Factorio-CLI-Mod-manger?branch=master)
[![Build Status](https://travis-ci.org/Scarjit/Factorio-CLI-Mod-manger.svg?branch=master)](https://travis-ci.org/Scarjit/Factorio-CLI-Mod-manger)
## Getting started
 - Install [rust & cargo](https://www.rust-lang.org/learn/get-started)
 - Use rustup to download a newish nightly version (tested on ``nightly-2020-06-18-x86_64-pc-windows-gnu``)
 - Clone this repository (``git clone https://github.com/Scarjit/Factorio-CLI-Mod-manger``)
 - Run it using: ``cargo run --release --package factorio_mod_manager --bin factorio_mod_manager --  --help``
 
## Example usage:

### With CLI gui
 ```
Running `target\release\factorio_mod_manager.exe -u USERNAME -p PASSWORD -s D:\RustProjects\factorio_mod_manager\fctrserver\serverfiles install`

Creating api-token
Downloading mod list ...
Type mod to install. Insert 'q' to quit or 'i' to install selected.
flare stack
Found multiple options:
+----+-----------------+-------------+-----------+----------------------+
| ID | Name            | Title       | Downloads | Levenshtein distance |
+----+-----------------+-------------+-----------+----------------------+
| 0  | Flare Stack     | Flare Stack | 110088    | 2                    |
+----+-----------------+-------------+-----------+----------------------+
| 1  | Flare Stack 015 | Flare Stack | 1059      | 2                    |
+----+-----------------+-------------+-----------+----------------------+
| 2  | flame-tank      | Flame tank  | 1216      | 4                    |
+----+-----------------+-------------+-----------+----------------------+
Select mod to install, insert 'n' to select none
0
Installing Flare Stack
Type mod to install. Insert 'q' to quit or 'i' to install selected.
i
Dependency parsing for: 
+-------------+
| Flare Stack |
+-------------+
Parsed dependency graph:
+-----------------------+--------------------------------------------------+
| Flare Stack_2.2.6.zip | /download/Flare%20Stack/5e81d69b97ca9c000c97e815 |
+-----------------------+--------------------------------------------------+
Enter 'd' to start downloading, else aborting
d
Downloading Flare Stack_2.2.6.zip from https://mods.factorio.com//download/Flare%20Stack/5e81d69b97ca9c000c97e815?username=Scarjit&token=XXXXXXXXXXXXXXXXXXXXXXXXXX
Finished downloading Flare Stack_2.2.6.zip !
Finished downloading !
```

### Command line only
```
Running `target\debug\factorio_mod_manager.exe -u USERNAME -p PASSWORD -s D:\RustProjects\factorio_mod_manager\fctrserver\serverfiles install -m bobores`
Creating api-token
Downloading mod list ...
Dependency parsing for: 
+---------+
| bobores |
+---------+
Parsed dependency graph:
+-----------------------+-----------------------------------------------+
| boblibrary_0.18.9.zip | /download/boblibrary/5e9daef69e85eb000cee2549 |
+-----------------------+-----------------------------------------------+
| bobores_0.18.3.zip    | /download/bobores/5ec96a823cf12a000b34fcb0    |
+-----------------------+-----------------------------------------------+
boblibrary_0.18.9.zip already downloaded !
bobores_0.18.3.zip already downloaded !
Finished downloading !
```

CLI only will not recommend other mods, if the mod is written incorrectly !
