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
      # homeManagerModules = {
      #   otter = import "${self}/module.nix" {
      #     inherit self;
      #     isNixOSModule = false;
      #   };
      #   default = self.homeManagerModules.otter;
      # };
      # Nixos module, consumed by other flakes
      # nixosModules."otter" = { config, ... }: { options = {}; config = {}; };
      nixosModules."otter" = import ./module.nix;
      nixosModules.default = import ./module.nix;
      # Default module
      # nixosModules.default = { config, ... }: { options = {}; config = {}; };
      #
      homeManagerModules.otter = import ./module.nix;
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

