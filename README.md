# GLAM - Godot Lightweight Addon Manager
✨GLAM✨ is a CLI tool to facilitate addon managing for Godot 4.0 projects. It's lightweight and focused on providing basic addon managament such as installing and applying changes to any addon that can be installed via git. It does so by adding two files to your project: a **.glam** file that tracks each addon repository and their installed commit, and a **.glam.d/** folder that stores the addons repositories, so you can easily update or apply changes to them - after installing or updating an addon, it's files are copied to the Godot project `addons/` folder. The tool also provides a meaningful `help` for all commands.

This is **not** a one-size-fits-all kind of solution. I created it because I wanted an easy way to start a Godot project with addons I use often without having to download them via the Asset Manager, and to develop my own Addons while I use them on other projects (hence `glam apply`). If you are interested in other alternatives, check the **Alternatives** section.

## Installing
The project only works on UNIX (Linux/Mac) machines. You need `git` installed for the commands to work. It works on **Godot 4.0**, but it should work with any Godot project.

**This is a WIP project**, so I'm not too focused on providing releases; right now, the easiest way to install this project is via `cargo install godot-glam` (https://lib.rs/crates/godot-glam). This may change if this project gain traction.

## Example
You can initialize your project (with meaningful `.gitignore` and `.gdignore` files) with:

```
glam init
```

![](init_example.gif)

You can add new addon repositories with:
```
glam add https://github.com/henriquelalves/GodotTIE
```

![](add_example.gif)

If you want to use ✨GLAM✨ with a CI/CD, you can `.gitignore` the `addons/` folder and install all addons listed in the `.glam` file with:
```
glam install
```

![](install_example.gif)

If you are developing an Addon, you can apply changes made in the addon to their `.glam.d` repository with:
```
glam apply
```

![](apply_example.gif)

## Disclaimer
**This project is a WIP!** This is a beta release to anyone interested in using or contributing to this project. It may contain bugs that may ruin your project if you don't make any backups or use version-control wisely.

## TODO's

- [ ] Implement multiple addons per repository

## Alternatives
- https://github.com/imjp94/gd-plug
This is an addon that inspired and works similarly to this CLI tool, but using GDScript (and the Godot executable). I prefered creating the Rust CLI tool though, so I wouldn't need to bootstrap my Godot projects to use it, and to easier extend the tool to my needs.
- https://crates.io/crates/gotpm
Another Rust solution; the GitHub repository seems to be missing, though.
