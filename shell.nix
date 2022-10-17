{ pkgs ? import <nixpkgs> {}
}:

pkgs.mkShell {
  buildInputs = with pkgs; [
    go
    rustc
    rustfmt
    cargo
    clippy
    clang
    mold
    sqlx-cli
    pkg-config
    openssl.dev
    nodejs
    tmux
    docker
    docker-compose
  ];
  RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
  RUST_BACKTRACE = 1;

 #OPENSSL_DEV=openssl.dev;
  shellHook = ''
    echo 'Shell started...'
  '';
}