{
  description = "Zola with plugins";

  inputs = {
    utils.url = "github:numtide/flake-utils";
    naersk.url = "github:nmattia/naersk";
  };

  outputs = { self, nixpkgs, utils, naersk }:
  utils.lib.eachDefaultSystem (system: let
    pkgs = nixpkgs.legacyPackages."${system}";
    naersk-lib = naersk.lib."${system}";
  in rec {
    packages.pepel = naersk-lib.buildPackage {
      pname = "pepel";
      root = ./.;
    };
    defaultPackage = packages.pepel;

    apps.pepel = utils.lib.mkApp {
      drv = packages.pepel;
    };
    defaultApp = apps.pepel;

    devShell = pkgs.mkShell {
      nativeBuildInputs = with pkgs; [ rustc cargo clippy rustfmt rust-analyzer ];
    };
  });
}
