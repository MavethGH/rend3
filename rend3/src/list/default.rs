use crate::list::{ImageInputReference, ShaderStage};
use crate::{
    list::{
        ImageOutput, ImageOutputReference, ImageReference, ImageResolution, ImageResourceDescriptor, RenderList,
        RenderOpDescriptor, RenderOpInputType, RenderPassDescriptor, RenderPassSetDescriptor, RenderPassSetRunRate,
        ResourceBinding, ShaderSource, ShaderSourceType, SourceShaderDescriptor,
    },
    renderer::MAX_MATERIALS,
};
use glam::Vec2;
use wgpu::TextureFormat;

pub fn default_render_list() -> RenderList {
    let mut list = RenderList::new();

    list.create_shader(
        "depth vert",
        ShaderSource::Glsl(SourceShaderDescriptor {
            source: ShaderSourceType::File("rend3/shaders/depth.vert".to_string()),
            stage: ShaderStage::Vertex,
            includes: vec![],
            defines: vec![(String::from("MATERIAL_COUNT"), Some(MAX_MATERIALS.to_string()))],
        }),
    );

    list.create_shader(
        "depth frag",
        ShaderSource::Glsl(SourceShaderDescriptor {
            source: ShaderSourceType::File("rend3/shaders/depth.frag".to_string()),
            stage: ShaderStage::Fragment,
            includes: vec![],
            defines: vec![(String::from("MATERIAL_COUNT"), Some(MAX_MATERIALS.to_string()))],
        }),
    );

    list.start_render_pass_set(RenderPassSetDescriptor {
        run_rate: RenderPassSetRunRate::PerShadow,
    });

    list.add_render_pass(RenderPassDescriptor {
        outputs: vec![],
        depth: Some(ImageOutputReference::OutputImage),
    });

    list.add_render_op(RenderOpDescriptor {
        input: RenderOpInputType::Models3D,
        vertex: String::from("depth vert"),
        fragment: Some(String::from("depth frag")),
        bindings: vec![
            ResourceBinding::GeneralData,
            ResourceBinding::ObjectData,
            ResourceBinding::GPU2DTextures,
            ResourceBinding::CameraData,
        ],
    });

    let internal_renderbuffer_name = "color renderbuffer";

    list.create_image(
        internal_renderbuffer_name,
        ImageResourceDescriptor {
            resolution: ImageResolution::Relative(ImageReference::OutputImage, Vec2::splat(1.0)),
            format: TextureFormat::Rgba16Float,
            samples: 1,
        },
    );

    list.create_image(
        "normal buffer",
        ImageResourceDescriptor {
            resolution: ImageResolution::Relative(
                ImageReference::Custom(internal_renderbuffer_name.to_string()),
                Vec2::splat(1.0),
            ),
            format: TextureFormat::Rgba16Float,
            samples: 1,
        },
    );

    list.create_image(
        "depth_buffer",
        ImageResourceDescriptor {
            resolution: ImageResolution::Relative(
                ImageReference::Custom(internal_renderbuffer_name.to_string()),
                Vec2::splat(1.0),
            ),
            format: TextureFormat::Depth32Float,
            samples: 1,
        },
    );

    list.start_render_pass_set(RenderPassSetDescriptor {
        run_rate: RenderPassSetRunRate::Once,
    });

    list.add_render_pass(RenderPassDescriptor {
        outputs: vec![
            ImageOutput {
                output: ImageOutputReference::Custom(internal_renderbuffer_name.to_owned()),
                resolve_target: None,
            },
            ImageOutput {
                output: ImageOutputReference::Custom(String::from("normal buffer")),
                resolve_target: None,
            },
        ],
        depth: Some(ImageOutputReference::Custom(String::from("depth buffer"))),
    });

    list.add_render_op(RenderOpDescriptor {
        input: RenderOpInputType::Models3D,
        vertex: String::from("depth vert"),
        fragment: Some(String::from("depth frag")),
        bindings: vec![
            ResourceBinding::GeneralData,
            ResourceBinding::ObjectData,
            ResourceBinding::GPU2DTextures,
            ResourceBinding::CameraData,
        ],
    });

    list.create_shader(
        "skybox vert",
        ShaderSource::Glsl(SourceShaderDescriptor {
            source: ShaderSourceType::File("rend3/shaders/skybox.vert".to_string()),
            stage: ShaderStage::Vertex,
            includes: vec![],
            defines: vec![],
        }),
    );

    list.create_shader(
        "skybox vert",
        ShaderSource::Glsl(SourceShaderDescriptor {
            source: ShaderSourceType::File("rend3/shaders/skybox.frag".to_string()),
            stage: ShaderStage::Fragment,
            includes: vec![],
            defines: vec![],
        }),
    );

    list.add_render_op(RenderOpDescriptor {
        input: RenderOpInputType::FullscreenTriangle,
        vertex: String::from("skybox vert"),
        fragment: Some(String::from("skybox frag")),
        bindings: vec![
            ResourceBinding::GeneralData,
            ResourceBinding::SkyboxTexture,
            ResourceBinding::CameraData,
        ],
    });

    list.create_shader(
        "opaque vert",
        ShaderSource::Glsl(SourceShaderDescriptor {
            source: ShaderSourceType::File("rend3/shaders/opaque.vert".to_string()),
            stage: ShaderStage::Vertex,
            includes: vec![],
            defines: vec![(String::from("MATERIAL_COUNT"), Some(MAX_MATERIALS.to_string()))],
        }),
    );

    list.create_shader(
        "opaque frag",
        ShaderSource::Glsl(SourceShaderDescriptor {
            source: ShaderSourceType::File("rend3/shaders/opaque.frag".to_string()),
            stage: ShaderStage::Fragment,
            includes: vec![],
            defines: vec![(String::from("MATERIAL_COUNT"), Some(MAX_MATERIALS.to_string()))],
        }),
    );

    list.add_render_op(RenderOpDescriptor {
        input: RenderOpInputType::Models3D,
        vertex: String::from("opaque vert"),
        fragment: Some(String::from("opaque frag")),
        bindings: vec![
            ResourceBinding::GeneralData,
            ResourceBinding::ObjectData,
            ResourceBinding::GPU2DTextures,
            ResourceBinding::CameraData,
        ],
    });

    list.add_render_pass(RenderPassDescriptor {
        outputs: vec![ImageOutput {
            output: ImageOutputReference::OutputImage,
            resolve_target: None,
        }],
        depth: None,
    });

    list.create_shader(
        "blit vert",
        ShaderSource::Glsl(SourceShaderDescriptor {
            source: ShaderSourceType::File("rend3/shaders/blit.vert".to_string()),
            stage: ShaderStage::Vertex,
            includes: vec![],
            defines: vec![],
        }),
    );

    list.create_shader(
        "blit vert",
        ShaderSource::Glsl(SourceShaderDescriptor {
            source: ShaderSourceType::File("rend3/shaders/blit.frag".to_string()),
            stage: ShaderStage::Fragment,
            includes: vec![],
            defines: vec![],
        }),
    );

    list.add_render_op(RenderOpDescriptor {
        input: RenderOpInputType::FullscreenTriangle,
        vertex: String::from("blit vert"),
        fragment: Some(String::from("blit frag")),
        bindings: vec![
            ResourceBinding::GeneralData,
            ResourceBinding::Custom2DTexture(vec![ImageInputReference::Custom(
                internal_renderbuffer_name.to_string(),
            )]),
        ],
    });

    list
}
