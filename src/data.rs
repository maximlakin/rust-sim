use crate::mesh;

const SIDE: f32 = 0.48;

pub const GROUND_VERTICES: &[mesh::Vertex] = &[
    mesh::Vertex {
        position:   [-SIDE, SIDE, 0.0],
    },
    mesh::Vertex {
        position:   [-SIDE, -SIDE, 0.0],
    },
    mesh::Vertex {
        position:   [SIDE, -SIDE, 0.0],
    },
    mesh::Vertex {
        position:   [SIDE, SIDE, 0.0],
    },
];

pub const BOX_VERTICES: &[mesh::Vertex2] = &[
    mesh::Vertex2 { position: [-2.0, -2.0, 0.1]},
    mesh::Vertex2 { position: [-2.0,  2.0, 0.1]},

    mesh::Vertex2 { position: [-2.0,  2.0, 0.1]},
    mesh::Vertex2 { position: [ 2.0,  2.0, 0.1]},

    mesh::Vertex2 { position: [ 2.0,  2.0, 0.1]},
    mesh::Vertex2 { position: [ 2.0, -2.0, 0.1]},

    mesh::Vertex2 { position: [ 2.0, -2.0, 0.1]},
    mesh::Vertex2 { position: [-2.0, -2.0, 0.1]},
// ------------------
    mesh::Vertex2 { position: [-2.0, -2.0, 0.1]},
    mesh::Vertex2 { position: [-2.0, -2.0, 1.1]},

    mesh::Vertex2 { position: [-2.0,  2.0, 0.1]},
    mesh::Vertex2 { position: [-2.0,  2.0, 1.1]},

    mesh::Vertex2 { position: [ 2.0,  2.0, 0.1]},
    mesh::Vertex2 { position: [ 2.0,  2.0, 1.1]},

    mesh::Vertex2 { position: [ 2.0, -2.0, 0.1]},
    mesh::Vertex2 { position: [ 2.0, -2.0, 1.1]},
// ------------------
    mesh::Vertex2 { position: [-2.0, -2.0, 1.1]},
    mesh::Vertex2 { position: [-2.0,  2.0, 1.1]},

    mesh::Vertex2 { position: [-2.0,  2.0, 1.1]},
    mesh::Vertex2 { position: [ 2.0,  2.0, 1.1]},

    mesh::Vertex2 { position: [ 2.0,  2.0, 1.1]},
    mesh::Vertex2 { position: [ 2.0, -2.0, 1.1]},
    
    mesh::Vertex2 { position: [ 2.0, -2.0, 1.1]},
    mesh::Vertex2 { position: [-2.0, -2.0, 1.1]},
];

pub const INDICES: &[u16] = &[0, 1, 3, 1, 2, 3];

#[rustfmt::skip]
pub const OPENGL_TO_WGPU_MATRIX: cgmath::Matrix4<f32> = cgmath::Matrix4::new(
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.5,
    0.0, 0.0, 0.0, 1.0,
);

pub const NUM_INSTANCES_PER_ROW: u32 = 20;

pub const INSTANCE_DISPLACEMENT: cgmath::Vector3<f32> = cgmath::Vector3::new(
    (NUM_INSTANCES_PER_ROW as f32 - 1.0) * 0.5,    
    (NUM_INSTANCES_PER_ROW as f32 - 1.0) * 0.5,
    0.0,
);