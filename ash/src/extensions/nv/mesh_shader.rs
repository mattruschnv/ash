#![allow(dead_code)]
use std::ffi::CStr;
use std::mem;
use version::{DeviceV1_0, InstanceV1_0};
use vk;

#[derive(Clone)]
pub struct MeshShader {
    mesh_shader_fn: vk::NvMeshShaderFn,
}

impl MeshShader {
    pub fn new<I: InstanceV1_0, D: DeviceV1_0>(instance: &I, device: &D) -> MeshShader {
        let mesh_shader_fn = vk::NvMeshShaderFn::load(|name| unsafe {
            mem::transmute(instance.get_device_proc_addr(device.handle(), name.as_ptr()))
        });
        MeshShader { mesh_shader_fn }
    }
    pub unsafe fn cmd_draw_mesh_tasks(
        &self,
        command_buffer: vk::CommandBuffer,
        task_count: u32,
        first_task: u32,
    ) {
        self.mesh_shader_fn
            .cmd_draw_mesh_tasks_nv(command_buffer, task_count, first_task);
    }
    pub unsafe fn cmd_draw_mesh_tasks_indirect(
        &self,
        command_buffer: vk::CommandBuffer,
        buffer: vk::Buffer,
        offset: vk::DeviceSize,
        draw_count: u32,
        stride: u32,
    ) {
        self.mesh_shader_fn.cmd_draw_mesh_tasks_indirect_nv(
            command_buffer,
            buffer,
            offset,
            draw_count,
            stride,
        );
    }
    pub unsafe fn cmd_draw_mesh_tasks_indirect_count(
        &self,
        command_buffer: vk::CommandBuffer,
        buffer: vk::Buffer,
        offset: vk::DeviceSize,
        count_buffer: vk::Buffer,
        count_buffer_offset: vk::DeviceSize,
        max_draw_count: u32,
        stride: u32,
    ) {
        self.mesh_shader_fn.cmd_draw_mesh_tasks_indirect_count_nv(
            command_buffer,
            buffer,
            offset,
            count_buffer,
            count_buffer_offset,
            max_draw_count,
            stride,
        );
    }
    pub fn name() -> &'static CStr {
        CStr::from_bytes_with_nul(b"VK_NV_mesh_shader\0").expect("Wrong extension string")
    }
}
