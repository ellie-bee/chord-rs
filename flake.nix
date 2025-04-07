{
  inputs = {
    naersk.url = "github:nix-community/naersk/master";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, utils, naersk }:
    utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
        naersk-lib = pkgs.callPackage naersk { };
      in
      {
        defaultPackage = naersk-lib.buildPackage ./.;
        devShell = with pkgs; mkShell {
					env.LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath [ pkgs.xdotool ];
          buildInputs = [
						cargo
						rustc
						rustfmt
						pre-commit
						rustPackages.clippy
						xdotool
						xorg.libX11
  					xorg.libXtst
  					xorg.libXi
					];
					nativeBuildInputs = [
						xdotool
						xorg.libX11
  					xorg.libXtst
  					xorg.libXi
					];
          RUST_SRC_PATH = rustPlatform.rustLibSrc;
        };
      }
    );
}
