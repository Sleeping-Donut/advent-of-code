{
	description = "Flake for advent of code dev shells";

	inputs = {
		nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
	};

	outputs = { self, nixpkgs }: let
		allSystems = [
			"aarch64-darwin"
		];
		forAllSystems = f: nixpkgs.lib.genAttrs allSystems (system: f {
			pkgs = import nixpkgs { inherit system; };
		});

	in {
		devShells = forAllSystems ({ pkgs }: {
			python = pkgs.mkShell {
				packages = with pkgs; [ python3Minimal uv ];
			};
			rust = pkgs.mkShell {
				nativeBuildInputs = with pkgs; [
					rustc cargo gcc rustfmt clippy rust-analyzer
				];
			};
		});
	};
}
