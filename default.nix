{
  lib,
  rustPlatform,
  fetchFromGitHub,
}:

rustPlatform.buildRustPackage rec {
  pname = "combiner";
  version = "0.1.8";

  src = fetchFromGitHub {
    owner = "Jesalx";
    repo = "combiner";
    rev = "v${version}";
    sha256 = "sha256-h41piGQNXyl4LAOihScTjzT8sWNZzb79SZIxjUr+FrA=";
  };

  cargoLock = {
    lockFile = ./Cargo.lock;
  };

  meta = with lib; {
    description = "A tool for recursively combining text files in a directory";
    homepage = "https://github.com/Jesalx/combiner";
    license = licenses.mit;
    maintainers = with maintainers; [ "jesalx" ];
  };
}
