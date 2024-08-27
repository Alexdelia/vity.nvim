# vity

<!-- logo -->
<p align="center">
	<img alt="vity logo" src="https://raw.githubusercontent.com/Alexdelia/vity/main/icon.png" width="42%">
</p>

<!-- badges -->
<p align="center">
	<a href="https://github.com/Alexdelia/vity.nvim">
		<img align="center"
			height="32vw"
			alt="github hits"
			src="https://img.shields.io/endpoint?color=d5397b&logo=github&style=for-the-badge&url=https%3A%2F%2Fhits.dwyl.com%2FAlexdelia%2Fvitynvim.json"
		/>
	</a>
	<a href="https://rustacean.net/">
		<img align="center"
			height="38vw"
			alt="rust ferris"
			src="https://rustacean.net/favicon.png"
		/>
	</a>
	<a href="https://builtwithnix.org">
		<img align="center"
			height="34vw"
			alt="built with nix"
			src="https://builtwithnix.org/badge.svg"
		/>
	</a>
</p>

<!-- description -->
<h2 align="center">
	<span style="color: #57A143">neovim</span>
	<b style="color: #E34A87">colorscheme</b>
	<small>focused on</small>
	<i style="color: #19a1e6">visibility</i>
	<small>and</small>
	<i style="color: #19a1e6">readability</i>
</h2>

<br>

## Install

### with `nix`

```nix
# flake.nix
{
  inputs = {
    vity.url = "github:Alexdelia/vity.nvim";
  };
}
```

```nix
# home.nix
{
  inputs,
  pkgs,
  ...
}: {
  programs.neovim = {
    plugins = [
      (pkgs.vimUtils.buildVimPlugin {
        name = "vity";
        src = inputs.vity.packages.${pkgs.system}.default

        config = "colorscheme vity";
      })
    ];
  };
}
```

### manually with `cargo`

```nushell
# compile the plugin in a `.so` file
cargo build --release

# place the `.so` file in the `lua` directory to be detected as a plugin by `neovim`
mkdir -p ~/.config/nvim/lua

mv target/release/libvity.so ~/.config/nvim/lua/vity.so

# *optional* make `:colorscheme vity` work by adding an entry in the `colors` directory
mkdir $out/colors
echo "require('vity').load()" > ~/.config/nvim/colors/vity.lua
```

## Usage

### in the `neovim` command line

```vim
:colorscheme vity
```

### in the `init.lua` file

```lua
-- init.lua
require('vity').load()
require('vity').setup() -- alias of `load()`
require('vity').colorscheme() -- alias of `load()`
```
