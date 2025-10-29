{
  description = "A Nix-based development environment";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
  };

  outputs = { self, nixpkgs }:
    let
      system = "x86_64-linux";
      pkgs = import nixpkgs {
        inherit system;
      };

      samCli = pkgs.aws-sam-cli.overrideAttrs (oldAttrs: {
        buildInputs = oldAttrs.buildInputs ++ [
          (pkgs.python3Packages.click.overrideAttrs (oldClickAttrs: rec {
            version = "8.1.8";
            src = pkgs.fetchPypi {
              pname = "click";
              inherit version;
              sha256 = "sha256-7VPJ2JkNg8Kifermjk7jN0c/YzDAQKMdQiXJV00WCWo="; # Corrected SHA256
            };
          }))
        ];
      });

    in {
      devShells.${system} = {
        default = pkgs.mkShell {
          buildInputs = [
            samCli
            # pkgs.cargo
            # pkgs.rustc
          ];
        };
      };
    };
}
