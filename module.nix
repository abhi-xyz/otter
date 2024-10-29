{
lib,
config,
self,
...
}:
let
  cfg = config.otter;
in
  {
  options.otter = {
    enable = lib.mkEnableOption "the otter program";

    package = lib.mkOption {
      type = lib.types.package;
      default = self.packages.default;
      description = "otter package to use.";
    };
  };
  config = lib.mkIf cfg.enable {
    home.packages = [ cfg.package ];
  };
}
