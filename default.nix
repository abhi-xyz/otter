{ pkgs ? import <nixpkgs> { } }:
let 
  rustPlatform = pkgs.rustPlatform;
  manifest = (pkgs.lib.importTOML ./Cargo.toml).package;
in
  rustPlatform.buildRustPackage rec {
    pname = manifest.name;
    version = manifest.version;
    cargoLock.lockFile = ./Cargo.lock;
    src = pkgs.lib.cleanSource ./.;

    #     installPhase = ''
    #      mkdir -p $out/bin
    #      cp -r target/x86_64-unknown-linux-gnu/release/ $out/bin/
    #      '';
    #
    doCheck = false;

    meta = with pkgs.lib; {
      description = manifest.description;
      #       homepage = "https://github.com/abhi-xyz/brightness";
      #       changelog = "https://github.com/abhi-xyz/brightness/releases";
      license = licenses.mit;
      maintainers = with maintainers; [ Abhinandh S ];
      platforms = platforms.linux; # Specify supported platforms
      mainProgram = manifest.name;
    };

  }
