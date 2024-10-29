{
  description = "Foo Bar";
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
  };
  outputs =
    { self, nixpkgs }:
    let
      supportedSystems = [ "x86_64-linux" ];
      forAllSystems = nixpkgs.lib.genAttrs supportedSystems;
      pkgsFor = nixpkgs.legacyPackages;
      system = "x86_64-linux";
      pkgs = import nixpkgs { inherit system; };
    in
    {
      homeManagerModules.otter =
        {
          config,
          pkgs,
          lib,
          ...
        }:
        let
          tomlFormat = pkgs.formats.toml { };
        in
        {
          options.program.otter = {
            enable = lib.mkEnableOption "Enable the Otter program";

            package = lib.mkOption {
              type = lib.types.package;
              default = pkgs.callPackage ./default.nix { };
              description = "The otter package to use.";
            };

            settings = lib.mkOption {
              type = tomlFormat.type;
              default = { };
              example = lib.literalExpression ''
                                [directories]
                images = { path = "/home/abhi/pics/pictures/images", tree = false }
                documents = { path = "/home/abhi/docs/lib", tree = false }
                videos = { path = "/home/abhi/videos", tree = true }
                archives = { path = "/home/abhi/archive", tree = false }

                [input_dirs]
                dirs = [
                  "/home/abhi/videos",
                  "/home/abhi/downloads"
                ]
              '';
              description = ''
                Configuration written to {file}`$XDG_CONFIG_HOME/oculante/config.json`.
              '';
            };

          };

          config = lib.mkIf config.program.otter.enable {
            home.packages = [ config.program.otter.package ];

            xdg.configFile."otter/config.toml" = lib.mkIf (config.program.otter.settings != { }) {
              source = tomlFormat.generate "config.toml" config.program.otter.settings;
            };
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
