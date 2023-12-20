use super::{
    super::geometry::{Size, ToVertices, Transformation2D},
    super::pwindow::WindowHandle,
    base::{BaseStimulus, BaseStimulusPixelShader, ShapeStimulusParams},
};
use bytemuck::{Pod, Zeroable};
use futures_lite::future::block_on;
use std::borrow::Cow;
use wgpu::{Device, ShaderModule};

/// The parameters for the gratings stimulus, these will be used as uniforms
/// and made available to the shader.
#[repr(C)]
#[derive(Debug, Copy, Clone, Pod, Zeroable)]
pub struct GratingsStimulusParams {
    pub phase: f32,
    pub cycle_length: f32, // in pixels
}

// TODO: make this a macro
impl ShapeStimulusParams for GratingsStimulusParams {}

pub struct GratingsShader {
    shader: ShaderModule,
    cycle_length: Size,
    phase: f32,
}

pub type GratingsStimulus<'a, G> =
    BaseStimulus<G, GratingsShader, GratingsStimulusParams>;

impl<G: ToVertices> GratingsStimulus<'_, G> {
    /// Create a new gratings stimulus.
    pub fn new(
        window_handle: &WindowHandle,
        shape: G,
        cycle_length: impl Into<Size>,
        phase: f32,
    ) -> Self {
        let window = block_on(window_handle.get_window());
        let device = &window.device;

        let shader = GratingsShader::new(&device, phase, cycle_length.into());

        let params = GratingsStimulusParams {
            phase,
            cycle_length: 0.0,
        };

        drop(window); // this prevent a deadlock (argh, i'll have to refactor this)

        BaseStimulus::create(window_handle, shader, shape, params, None)
    }

    /// Set the length of a cycle.
    pub fn set_cycle_length(&self, length: impl Into<Size>) {
        block_on(self.pixel_shader.lock()).cycle_length = length.into();
    }

    /// Set the phase.
    pub fn set_phase(&self, phase: f32) {
        block_on(self.pixel_shader.lock()).phase = phase;
    }
}

impl GratingsShader {
    pub fn new(device: &Device, phase: f32, frequency: Size) -> Self {
        let shader: ShaderModule =
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!(
                    "shaders/gratings.wgsl"
                ))),
            });

        Self {
            shader,
            cycle_length: frequency,
            phase: phase,
        }
    }
}

impl BaseStimulusPixelShader<GratingsStimulusParams> for GratingsShader {
    fn prepare(
        &self,
        params: &mut GratingsStimulusParams,
        width_mm: f64,
        viewing_distance_mm: f64,
        width_px: i32,
        height_px: i32,
    ) {
        // update the shader params
        params.cycle_length = self.cycle_length.to_pixels(
            width_mm as f64,
            viewing_distance_mm as f64,
            width_px,
            height_px,
        ) as f32;
    }
    fn get_shader(&self) -> &ShaderModule {
        &self.shader
    }
}