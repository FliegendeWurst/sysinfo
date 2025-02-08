{
  description = "sysinfo - CPU / RAM / Disk buffers / Temperature";

  inputs.nixpkgs.url = "nixpkgs/nixos-24.11";

  outputs = { self, nixpkgs }:
    let

      lib = nixpkgs.lib;

      # System types to support.
      supportedSystems = [ "x86_64-linux" "x86_64-darwin" "aarch64-linux" "aarch64-darwin" ];

      # Helper function to generate an attrset '{ x86_64-linux = f "x86_64-linux"; ... }'.
      forAllSystems = lib.genAttrs supportedSystems;

      # Nixpkgs instantiated for supported system types.
      nixpkgsFor = forAllSystems (system: import nixpkgs { inherit system; });

    in
    {

      # Provide some binary packages for selected system types.
      packages = forAllSystems (system:
        let
          pkgs = nixpkgsFor.${system};
        in
        {
          sysinfo = pkgs.rustPlatform.buildRustPackage rec {
            pname = "sysinfo";
            version = "0-unstable";

            src = ./.;

            cargoLock.lockFile = ./Cargo.lock;

            nativeBuildInputs = with pkgs; [ ];

            buildInputs = with pkgs; [ ];

            meta = with lib; {
              description = "sysinfo";
              homepage = "https://github.com/FliegendeWurst/sysinfo";
              license = licenses.gpl3Plus;
              maintainers = with maintainers; [ fliegendewurst ];
              mainProgram = "sysinfo";
            };
          };
        });

      defaultPackage = forAllSystems (system: self.packages.${system}.wastebin);
    };
}
