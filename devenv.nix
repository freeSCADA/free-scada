{pkgs, ...}: {
  packages = with pkgs; [
    git
    bacon
    cargo-watch
  ];

  languages = {
    rust = {
      enable = true;
      channel = "stable";
    };
  };
}
