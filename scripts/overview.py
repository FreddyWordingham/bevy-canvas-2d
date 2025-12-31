import sys
from pathlib import Path

from typeguard import typechecked


@typechecked
def print_files(root: Path) -> None:
    for path in sorted(root.rglob("*.rs")):
        if path.is_file():
            print(path.as_posix())
            print("```rust")
            try:
                print(path.read_text(encoding="utf-8"))
            except Exception as e:
                print(f"# [Error reading file: {e}]")
            print("```\n")


if __name__ == "__main__":
    if len(sys.argv) != 2:
        print(f"Usage: {sys.argv[0]} <directory>")
        sys.exit(1)
    root = Path(sys.argv[1])
    if not root.is_dir():
        print(f"Error: {root} is not a directory.")
        sys.exit(1)
    print_files(root)
