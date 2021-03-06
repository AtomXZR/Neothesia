use wgpu::vertex_attr_array;

use zerocopy::AsBytes;

#[repr(C)]
#[derive(Debug, Copy, Clone, AsBytes, PartialEq)]
pub struct RectangleInstance {
    pub position: [f32; 2],
    pub size: [f32; 2],
    pub color: [f32; 4],
}
impl RectangleInstance {
    pub fn attributes() -> [wgpu::VertexAttributeDescriptor; 3] {
        vertex_attr_array!(1 => Float2,2 => Float2,3 => Float4)
    }
    pub fn desc(attributes: &[wgpu::VertexAttributeDescriptor]) -> wgpu::VertexBufferDescriptor {
        wgpu::VertexBufferDescriptor {
            stride: std::mem::size_of::<RectangleInstance>() as wgpu::BufferAddress,
            step_mode: wgpu::InputStepMode::Instance,
            attributes,
        }
    }
}
