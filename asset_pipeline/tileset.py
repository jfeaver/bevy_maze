# Update the game's sprite sheet

# Run with:
# python asset_pipeline/tileset.py

import shutil
from pathlib import Path


def copy_file(source: str, destination: str) -> None:
    """Copy a file from source to destination."""
    src_path = Path(source)
    dest_path = Path(destination)

    # Ensure source file exists
    if not src_path.is_file():
        raise FileNotFoundError(f"Source file not found: {src_path}")

    # Ensure destination directory exists
    dest_path.parent.mkdir(parents=True, exist_ok=True)

    shutil.copy2(src_path, dest_path)  # preserves metadata
    print(f"Copied {src_path} → {dest_path}")


if __name__ == "__main__":
    copy_file(
        "/Users/nathan/art/TLoAsh/sprite_sheet.png",
        "/Users/nathan/personal/bevy_maze/assets/textures/tileset.png",
    )
