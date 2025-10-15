# Generate and save to file the matrix representing static world content from CSV files

# Run with:
# python asset_pipeline/world_map_gen.py

import csv

# --- CONFIGURATION ---
GROUND_MAP = {
    1: "GroundType::Grass",
    3: "GroundType::DirtH",
    4: "GroundType::DirtV",
}

OBSTRUCTION_MAP = {
    -1: "ObstructionType::None",
    7: "ObstructionType::WallV",
    6: "ObstructionType::WallH",
    15: "ObstructionType::Tower",
    3: "ObstructionType::Rock1",
    4: "ObstructionType::Rock2",
    5: "ObstructionType::Rock3",
}

GROUND_CSV = "/Users/nathan/art/TLoAsh/Tiled/trial_Ground.csv"
OBSTRUCTION_CSV = "/Users/nathan/art/TLoAsh/Tiled/trial_Walls.csv"
OUTPUT_RS = "/Users/nathan/personal/bevy_maze/src/gameplay/environment/world_map_array.rs"


def read_csv(path: str):
    with open(path, newline="") as f:
        return [[int(x) for x in row] for row in csv.reader(f)]


def make_tile_matrix(ground_matrix: list[list[int]], obstruction_matrix: list[list[int]]):
    tiles: list[list[str]] = []
    for gy, row in enumerate(ground_matrix):
        tile_row: list[str] = []
        for gx, gval in enumerate(row):
            oval = obstruction_matrix[gy][gx]
            gstr = GROUND_MAP.get(gval, "GroundType::Grass")
            ostr = OBSTRUCTION_MAP.get(oval, "ObstructionType::None")
            tile_row.append(f"Tile {{ ground: {gstr}, obstruction: {ostr} }}")
        tiles.append(tile_row)
    return tiles


def format_rust_matrix(tile_matrix: list[list[str]]):
    rows: list[str] = []
    for row in tile_matrix:
        rows.append("    [" + ", ".join(row) + "]")
    return "[\n" + ",\n".join(rows) + "\n];"


def main():
    ground = read_csv(GROUND_CSV)
    obstruction = read_csv(OBSTRUCTION_CSV)

    if len(ground) != len(obstruction) or any(
        len(gr) != len(orow) for gr, orow in zip(ground, obstruction)
    ):
        raise ValueError("CSV matrices must be the same size.")

    tiles = make_tile_matrix(ground, obstruction)
    rust_matrix = format_rust_matrix(tiles)

    rust_output = (
        "// Auto-generated tile map\n"
        "use super::{GroundType, ObstructionType, Tile};\n\n"
        "pub const TILE_MAP: [[Tile; "
        + str(len(ground))
        + "]; "
        + str(len(ground[0]))
        + "] = "
        + rust_matrix
        + "\n"
    )

    with open(OUTPUT_RS, "w") as f:
        f.write(rust_output)

    print(f"Wrote Rust matrix to {OUTPUT_RS}")


if __name__ == "__main__":
    main()
