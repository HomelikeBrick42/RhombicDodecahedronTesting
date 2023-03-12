use bevy::prelude::*;

pub fn rhombic_dodecahedron() -> Mesh {
    let mut mesh = Mesh::new(bevy::render::render_resource::PrimitiveTopology::TriangleList);
    let mut vertices = vec![];
    for rotation in [
        Quat::from_rotation_y(0.0f32.to_radians()),
        Quat::from_rotation_y(90.0f32.to_radians()),
        Quat::from_rotation_y(-90.0f32.to_radians()),
        Quat::from_rotation_y(-180.0f32.to_radians()),
        Quat::from_rotation_x(90.0f32.to_radians()),
        Quat::from_rotation_x(-90.0f32.to_radians()),
    ] {
        vertices.extend(
            [
                // Top
                [0.5, 0.5, 0.5],
                [-0.5, 0.5, 0.5],
                [0.0, 0.0, 1.0],
                // Bottom
                [0.5, -0.5, 0.5],
                [0.0, 0.0, 1.0],
                [-0.5, -0.5, 0.5],
                // Left
                [-0.5, 0.5, 0.5],
                [-0.5, -0.5, 0.5],
                [0.0, 0.0, 1.0],
                // Right
                [0.5, 0.5, 0.5],
                [0.0, 0.0, 1.0],
                [0.5, -0.5, 0.5],
            ]
            .into_iter()
            .map(|[x, y, z]| Transform::from_rotation(rotation) * Vec3 { x, y, z }),
        );
    }
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
    mesh.compute_flat_normals();
    mesh
}
