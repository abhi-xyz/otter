{
lib,
config,
pkgs,
self,
...
}:
let
  cfg = config.otter;
  defaultPkg = self.packages.${pkgs.hostPlatform.system}.default;
in
  {
  options.otter = {
    enable = lib.mkEnableOption "the otter program";

    package = lib.mkOption {
      type = lib.types.package;
      default = defaultPkg;
      description = "otter package to use.";
    };
  };
  config = lib.mkIf cfg.enable {
    home.packages = [ cfg.package ];
  };
}
