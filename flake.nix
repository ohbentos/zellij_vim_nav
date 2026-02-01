{
  description = "Zellij vim-nav plugin for seamless vim/pane navigation";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, rust-overlay }:
    let
      supportedSystems = [ "aarch64-darwin" "x86_64-darwin" "x86_64-linux" "aarch64-linux" ];
      forAllSystems = nixpkgs.lib.genAttrs supportedSystems;
    in
    {
      packages = forAllSystems (system:
        let
          pkgs = import nixpkgs {
            inherit system;
            overlays = [ rust-overlay.overlays.default ];
          };

          rustToolchain = pkgs.rust-bin.stable.latest.default.override {
            targets = [ "wasm32-wasip1" ];
          };

          rustPlatform = pkgs.makeRustPlatform {
            cargo = rustToolchain;
            rustc = rustToolchain;
          };
        in
        {
          default = rustPlatform.buildRustPackage {
            pname = "zellij-vim-nav";
            version = "0.1.0";
            src = ./.;

            cargoHash = "sha256-fqT+fOgigotqEiv84u2Y1TsCILr47M/KyJKWSveZ060=";

            buildPhase = ''
              cargo build --release --target wasm32-wasip1
            '';

            installPhase = ''
              mkdir -p $out/lib
              cp target/wasm32-wasip1/release/vim_nav.wasm $out/lib/
            '';

            doCheck = false;
          };
        }
      );
    };
}
