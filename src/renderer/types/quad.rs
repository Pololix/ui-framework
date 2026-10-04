use crate::types::{Color, Rect};

#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Quad {
    pub position: [f32; 2],
    pub size: [f32; 2],
    pub color: [f32; 4],
}

impl Quad {
    const ATTRIBUTES: [wgpu::VertexAttribute; 3] = wgpu::vertex_attr_array![
        0 => Float32x2, // position
        1 => Float32x2, // size
        2 => Float32x4, // color
    ];

    pub fn instance_buffer_layout() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Self>() as u64,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &Self::ATTRIBUTES,
        }
    }

    pub fn instance_buffer(device: &wgpu::Device, capacity: usize) -> wgpu::Buffer {
        let size = capacity * std::mem::size_of::<Self>();

        device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Instance buffer"),
            size: size as u64,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        })
    }

    pub fn from_rect(rect: Rect, color: Color) -> Self {
        Self {
            position: [rect.x, rect.y],
            size: [rect.width, rect.height],
            color: [color.r, color.g, color.b, color.a],
        }
    }

    pub fn as_rect(&self) -> Rect {
        Rect {
            x: self.position[0],
            y: self.position[1],
            width: self.size[0],
            height: self.size[1],
        }
    }
}
