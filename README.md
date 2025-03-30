# nix-activate

A simple script to enable nix flake direnv from current directory or external flake repository.

1. check current directory's flake
   - ex. `use flake .`
2. check git directory's flake
   - ex. `use flake /path/to/flakes/github.com/sudosubin/nix-activate`
3. check local-only flake
   - ex. `use flake /path/to/flakes/local/sudosubin/nix-activate`
