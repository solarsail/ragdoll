use amethyst::renderer::VertexPosNormal;

use geometry::SQRT3;

pub fn simple_hex_mesh() -> Vec<VertexPosNormal> {
    vec![
        VertexPosNormal { // origin
            pos: [0., 0., 0.],
            normal: [0., 0., 1.],
            tex_coord:[0.5, 0.5]
        },
        VertexPosNormal { // N
            pos: [0., 1., 0.],
            normal: [0., 0., 1.],
            tex_coord:[0.5, 0.0]
        },
        VertexPosNormal { // NW
            pos: [-SQRT3/2., 0.5, 0.],
            normal: [0., 0., 1.],
            tex_coord:[0.5-SQRT3/4., 0.25]
        },
        VertexPosNormal { // origin
            pos: [0., 0., 0.],
            normal: [0., 0., 1.],
            tex_coord:[0.5, 0.5]
        },
        VertexPosNormal { // NW
            pos: [-SQRT3/2., 0.5, 0.],
            normal: [0., 0., 1.],
            tex_coord:[0.5-SQRT3/4., 0.25]
        },
        VertexPosNormal { // SW
            pos: [-SQRT3/2., -0.5, 0.],
            normal: [0., 0., 1.],
            tex_coord:[0.5-SQRT3/4., 0.75]
        },
        VertexPosNormal { // origin
            pos: [0., 0., 0.],
            normal: [0., 0., 1.],
            tex_coord:[0.5, 0.5]
        },
        VertexPosNormal { // SW
            pos: [-SQRT3/2., -0.5, 0.],
            normal: [0., 0., 1.],
            tex_coord:[0.5-SQRT3/4., 0.75]
        },
        VertexPosNormal { // S
            pos: [0., -1., 0.],
            normal: [0., 0., 1.],
            tex_coord:[0.5, 1.0]
        },
        VertexPosNormal { // origin
            pos: [0., 0., 0.],
            normal: [0., 0., 1.],
            tex_coord:[0.5, 0.5]
        },
        VertexPosNormal { // S
            pos: [0., -1., 0.],
            normal: [0., 0., 1.],
            tex_coord:[0.5, 1.0]
        },
        VertexPosNormal { // SE
            pos: [SQRT3/2., -0.5, 0.],
            normal: [0., 0., 1.],
            tex_coord:[0.5+SQRT3/4., 0.75]
        },
        VertexPosNormal {
            pos: [0., 0., 0.],
            normal: [0., 0., 1.],
            tex_coord:[0.5, 0.5]
        },
        VertexPosNormal { // SE
            pos: [SQRT3/2., -0.5, 0.],
            normal: [0., 0., 1.],
            tex_coord:[0.5+SQRT3/4., 0.75]
        },
        VertexPosNormal { // NE
            pos: [SQRT3/2., 0.5, 0.],
            normal: [0., 0., 1.],
            tex_coord:[0.5+SQRT3/4., 0.25]
        },
        VertexPosNormal {
            pos: [0., 0., 0.],
            normal: [0., 0., 1.],
            tex_coord:[0.5, 0.5]
        },
        VertexPosNormal { // NE
            pos: [SQRT3/2., 0.5, 0.],
            normal: [0., 0., 1.],
            tex_coord:[0.5+SQRT3/4., 0.25]
        },
        VertexPosNormal { // N
            pos: [0., 1., 0.],
            normal: [0., 0., 1.],
            tex_coord:[0.5, 0.0]
        },
    ]
}