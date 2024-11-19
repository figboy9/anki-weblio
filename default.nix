{
  rustPlatform,
  openssl,
  pkg-config,
}:
rustPlatform.buildRustPackage {
  pname = "anki-weblio";
  version = "0.1.0";

  buildInputs = [ openssl ];
  nativeBuildInputs = [ pkg-config ];

  src = ./.;
  cargoLock.lockFile = ./Cargo.lock;
}
