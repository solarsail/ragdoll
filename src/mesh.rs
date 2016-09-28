use amethyst::renderer::VertexPosNormal;

use geometry::SQRT3;

pub fn simple_hex_mesh() -> Vec<VertexPosNormal> {
    vec![
        VertexPosNormal {
            pos: [0., 0., 0.],
            normal: [0., 0., 1.],
            tex_coord:[0.5, 0.5]
        },
        VertexPosNormal {
            pos: [0., 1., 0.],
            normal: [0., 0., 1.],
            tex_coord:[0.5, 0.0]
        },
        VertexPosNormal {
            pos: [SQRT3/2., 0.5, 0.],
            normal: [0., 0., 1.],
            tex_coord:[0.5+SQRT3/4., 0.25]
        },
        VertexPosNormal {
            pos: [0., 0., 0.],
            normal: [0., 0., 1.],
            tex_coord:[0.5, 0.5]
        },
        VertexPosNormal {
            pos: [SQRT3/2., 0.5, 0.],
            normal: [0., 0., 1.],
            tex_coord:[0.5+SQRT3/4., 0.25]
        },
        VertexPosNormal {
            pos: [SQRT3/2., -0.5, 0.],
            normal: [0., 0., 1.],
            tex_coord:[0.5+SQRT3/4., 0.75]
        },
        VertexPosNormal {
            pos: [0., 0., 0.],
            normal: [0., 0., 1.],
            tex_coord:[0.5, 0.5]
        },
        VertexPosNormal {
            pos: [SQRT3/2., -0.5, 0.],
            normal: [0., 0., 1.],
            tex_coord:[0.5+SQRT3/4., 0.75]
        },
        VertexPosNormal {
            pos: [0., -1., 0.],
            normal: [0., 0., 1.],
            tex_coord:[0.5, 1.0]
        },
        VertexPosNormal {
            pos: [0., 0., 0.],
            normal: [0., 0., 1.],
            tex_coord:[0.5, 0.5]
        },
        VertexPosNormal {
            pos: [0., -1., 0.],
            normal: [0., 0., 1.],
            tex_coord:[0.5, 1.0]
        },
        VertexPosNormal {
            pos: [-SQRT3/2., -0.5, 0.],
            normal: [0., 0., 1.],
            tex_coord:[0.5-SQRT3/4., 0.75]
        },
        VertexPosNormal {
            pos: [0., 0., 0.],
            normal: [0., 0., 1.],
            tex_coord:[0.5, 0.5]
        },
        VertexPosNormal {
            pos: [-SQRT3/2., -0.5, 0.],
            normal: [0., 0., 1.],
            tex_coord:[0.5-SQRT3/4., 0.75]
        },
        VertexPosNormal {
            pos: [-SQRT3/2., 0.5, 0.],
            normal: [0., 0., 1.],
            tex_coord:[0.5-SQRT3/4., 0.25]
        },
        VertexPosNormal {
            pos: [0., 0., 0.],
            normal: [0., 0., 1.],
            tex_coord:[0.5, 0.5]
        },
        VertexPosNormal {
            pos: [-SQRT3/2., 0.5, 0.],
            normal: [0., 0., 1.],
            tex_coord:[0.5-SQRT3/4., 0.25]
        },
        VertexPosNormal {
            pos: [0., 1., 0.],
            normal: [0., 0., 1.],
            tex_coord:[0.5, 0.0]
        },
    ]
}