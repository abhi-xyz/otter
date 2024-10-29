inputs:
# otter/module.nix
{ config, pkgs, lib, ... }:

{
  options.program.otter = {
    enable = lib.mkEnableOption "Enable the otter program.";
    # Define any other options here if needed
  };

  config = lib.mkIf config.program.otter.enable {
    # Define what to do when `program.otter.enable` is true
    home.packages = [ inputs.self.packages.${pkgs.stdenv.hostPlatform.system}.default ];
  };
}

