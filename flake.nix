{
  description = "Barricade Bar: статусбар для niri на GTK3";

  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

  outputs = { self, nixpkgs }:
    let
      systems = [ "x86_64-linux" "aarch64-linux" ];
      forAllSystems = f: nixpkgs.lib.genAttrs systems (system: f nixpkgs.legacyPackages.${system});
    in
    {
      packages = forAllSystems (pkgs: rec {
        barricade-wallpaper = pkgs.callPackage ./default.nix { };
        default = barricade-wallpaper;
      });

      apps = forAllSystems (pkgs: rec {
        barricade-wallpaper = {
          type = "app";
          program = nixpkgs.lib.getExe self.packages.${pkgs.stdenv.hostPlatform.system}.default;
        };
        default = barricade-wallpaper;
      });

      devShells = forAllSystems (pkgs: {
        default = import ./shell.nix { inherit pkgs; };
      });

      overlays.default = final: _prev: {
        barricade-bar = final.callPackage ./default.nix { };
      };

      nixosModules = {
        barricade-bar = ./module.nix;
        default = self.nixosModules.barricade-wallpaper;
      };
    };
}
