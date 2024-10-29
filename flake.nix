{
  description = "Foo Bar";
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
  };
  outputs = { self, nixpkgs }:
    let
      supportedSystems = [ "x86_64-linux" ];
      forAllSystems = nixpkgs.lib.genAttrs supportedSystems;
      pkgsFor = nixpkgs.legacyPackages;
    in {
      #
      # Nixos module, consumed by other flakes
      # nixosModules."otter" = { config, ... }: { options = {}; config = {}; };
      #      nixosModules."otter" = import ./module.nix;
      # nixosModules.default = import ./module.nix;
      # Default module
      # nixosModules.default = { config, ... }: { options = {}; config = {}; };
      #
      # homeManagerModules.otter = import ./module.nix;
      homeManagerModules.otter = { config, pkgs, lib, ... }: {
        options.program.otter = {
          enable = lib.mkEnableOption "Enable the Otter program";

          package = lib.mkOption {
            type = lib.types.package;
            default = pkgsFor.callPackage ./default.nix { };
            description = "The otter package to use.";
          };
        };

        config = lib.mkIf config.program.otter.enable {
          home.packages = [ config.program.otter.package ];
        };
      };
      
      homeManagerModules.default = self.homeManagerModules.otter;
      homeManagerModule.default = self.homeManagerModules.otter;
      #
      packages = forAllSystems (system: {
        default = pkgsFor.${system}.callPackage ./default.nix { };
      });
      devShells = forAllSystems (system: {
        default = pkgsFor.${system}.callPackage ./shell.nix { };
      });
    };
}
/*


# otter/module.nix
{ self, config, pkgs, lib, ... }:

{
  options.program.otter = {
    enable = lib.mkEnableOption "Enable the otter program.";
    # Define any other options here if needed
  };

  config = lib.mkIf config.program.otter.enable {
    # Define what to do when `program.otter.enable` is true
    home.packages = [ self.packages.${pkgs.stdenv.hostPlatform.system}.default ];
  };
}
  */
