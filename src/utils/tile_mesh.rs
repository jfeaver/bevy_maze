use bevy::render::render_resource::PrimitiveTopology;
use bevy::{asset::RenderAssetUsages, prelude::*};

pub struct AtlasConfig {
    pub cols: u16,
    pub rows: u16,
}

// Consider using a builder struct to allow for the option to use y-down coordinate systems
// although... that's not the Bevy way so maybe not?
pub fn build_tile_mesh(
    tiles: impl Iterator<Item = (Vec2, u16)>,
    atlas: &AtlasConfig,
    tile_size: f32,
    meshes: &mut Assets<Mesh>,
) -> Handle<Mesh> {
    let mut mesh = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::RENDER_WORLD,
    );
    let mut positions: Vec<[f32; 3]> = Vec::new();
    let mut uvs: Vec<[f32; 2]> = Vec::new();
    let mut indices: Vec<u16> = Vec::new();

    let mut i: u16 = 0;
    for (translation, sprite_index) in tiles {
        let offset_x = translation.x;
        let offset_y = translation.y;

        let u = (sprite_index % atlas.cols) as f32 / atlas.cols as f32;
        let v = (sprite_index / atlas.cols) as f32 / atlas.rows as f32;
        let du = 1.0 / atlas.cols as f32;
        let dv = 1.0 / atlas.rows as f32;

        // Associate positions on the mesh with coordinates on the texture atlas.
        // Each element of the positions vector corresponds to the same element of the uvs vector.
        // The indices vector is read three elements at a time and then a triangle is drawn by looking
        // up the elements in positions and uvs using the three elements/indices.

        // quad positions (positions on the rendered mesh)
        positions.extend([
            [offset_x, offset_y, 0.0],
            [offset_x + tile_size, offset_y, 0.0],
            [offset_x + tile_size, offset_y + tile_size, 0.0],
            [offset_x, offset_y + tile_size, 0.0],
        ]);

        // UVs into the atlas (vertex coordinates on the texture)
        uvs.extend([[u, v + dv], [u + du, v + dv], [u + du, v], [u, v]]);
        indices.extend([i, i + 1, i + 2, i, i + 2, i + 3]);
        i += 4;
    }

    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh.insert_indices(bevy::mesh::Indices::U16(indices));

    meshes.add(mesh)
}
