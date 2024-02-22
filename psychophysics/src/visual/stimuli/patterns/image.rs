// Copyright (c) 2024 Marc Pabst
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
use image::{DynamicImage, GenericImageView};

use super::super::pattern_stimulus::FillPattern;
use crate::{
    prelude::PsychophysicsError,
    utils::AtomicExt,
    visual::{
        color::{ColorFormat, IntoRawRgba, RawRgba},
        geometry::{Size, SizeVector2D, ToPixels},
        Window,
    },
};

#[derive(Clone, Debug)]
pub struct Image {
    image: image::DynamicImage,
}

impl Image {
    pub fn new(image: image::DynamicImage) -> Self {
        Self { image }
    }

    pub fn new_from_path(path: &str) -> Result<Self, image::ImageError> {
        println!("Loading image from path: {}", path);
        let image = image::open(path)?;
        Ok(Self { image })
    }
}

impl FillPattern for Image {
    fn texture_extent(&self, _window: &Window) -> Option<wgpu::Extent3d> {
        let (width, height) = self.image.dimensions();
        Some(wgpu::Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        })
    }

    fn texture_data(&self, _window: &Window) -> Option<Vec<u8>> {
        Some(self.image.to_rgba8().to_vec())
    }

    fn updated_texture_data(&self, _window: &Window) -> Option<Vec<u8>> {
        return None;
    }

    fn uniform_buffer_data(&self, _window: &Window) -> Option<Vec<u8>> {
        Some(vec![0; 32])
    }

    fn fragment_shader_code(&self, _window: &Window) -> String {
        "
        struct VertexOutput {
            @location(0) position: vec2<f32>,
            @location(1) tex_coords: vec2<f32>,
        };

        @group(0) @binding(1)
        var texture: texture_2d<f32>;

        @group(0) @binding(2)
        var texture_sampler: sampler;

        @fragment
        fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
            return vec4<f32>(textureSample(texture, texture_sampler, in.tex_coords).xyz, 0.5);
            //return textureSample(texture, texture_sampler, in.tex_coords);
        }
        "
        .to_string()
    }
}
