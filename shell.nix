let
    nixpkgs = import <nixpkgs> {
        config.allowUnfree = false;
        overlays = [ ];
    };
in
    with nixpkgs;
    stdenv.mkDerivation rec {
        name = "bitcoin-seed-tool";
        env = buildEnv { name = name; paths = buildInputs; };
        buildInputs = [
          cargo rustc rustracer
        ];

        shellHook = ''
          export NIX_SHELL_ENV=${name}


        '';
    }
