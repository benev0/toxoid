use crate::bindings::*;
use sokol::{app as sapp, gfx as sg};
use toxoid_render::{Renderer2D, RenderTarget, Sprite};
use toxoid_api::components::{Position, Size, Color, GameConfig};
use toxoid_api::World;
use std::any::Any;

pub struct SokolRenderer2D {
    pass_action: sg::PassAction,
}

pub struct SokolSprite {
    pub width: u32,
    pub height: u32,
    pub image: sg::Image,
}

pub struct SokolRenderTarget {
    pub sprite: Box<dyn Sprite>,
    pub depth_image: sg::Image,
    pub sampler: sg::Sampler,
    pub pass: sg::Pass,
}

impl Sprite for SokolSprite {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn width(&self) -> u32 {
        self.width
    }
    fn height(&self) -> u32 {
        self.height
    }
    fn set_width(&mut self, width: u32) {
        self.width = width;
    }
    fn set_height(&mut self, height: u32) {
        self.height = height;
    }
    /*
    fn drop(&self) {
        sg_destroy_image(&self.image);
    }
    */
}

impl RenderTarget for SokolRenderTarget {
    fn as_any(&self) -> &dyn Any {
        self
    }

    /*
    fn drop(&self) {
        &self.sprite.drop();
        sg_destroy_image(self.depth_image);
        sg_destroy_pass(self.pass);
        sg_destroy_sampler(self.sampler);
    }
    */
}

impl Renderer2D for SokolRenderer2D {
    fn new() -> Self {
        Self {
            pass_action: sg::PassAction::new()
        }
    }

    fn window_size() -> (u32, u32) {
        (sapp::width() as u32, sapp::height() as u32)
    }

    fn begin() {
        // Get the size of the window
        let (window_width, window_height) = (sapp::width(), sapp::height());
        unsafe {
            // Begin recording draw commands for a frame buffer of size (width, height).
            sgp_begin(window_width, window_height);
            // Set frame buffer drawing region to (0,0,width,height).
            sgp_viewport(0, 0, window_width, window_height);
            // Set drawing coordinate space to (left=0, right=width, top=0, bottom=height).
            sgp_project(0.0, window_width as f32, 0.0, window_height as f32);
            // Clear the frame buffer.
            sgp_set_color(0.1, 0.1, 0.1, 1.0);
            sgp_clear();
        }
        
    }

    fn end(&self) {
        // Get the size of the window
        let (window_width, window_height) = (sapp::width(), sapp::height());
        // Begin a render pass.
        sg::begin_default_pass(&self.pass_action, window_width, window_height);

        unsafe { 
            // Dispatch all draw commands to Sokol GFX.
            sgp_flush(); 
            // Finish a draw command queue, clearing it.
            sgp_end();
            
            // The actual sokol-gfx render pass, here we also don't need to care about
            // if the atlas image have already been loaded yet, if the image handles
            // recorded by sokol-spine for rendering are not yet valid, rendering
            // operations will silently be skipped.
            // Render Spine
            let layer_transform = sspine_layer_transform {
                size: sspine_vec2 { 
                    x: window_width as f32, 
                    y: window_height as f32 
                },
                origin: sspine_vec2 { 
                    x: 0., 
                    y: 0. 
                }
            };
            sspine_draw_layer(0, &layer_transform);

            // Render ImGui
            simgui_render();
        }
        // End render pass.
        sg::end_pass();
        // Commit Sokol render.
        sg::commit();
    }

    fn create_render_target(width: u32, height: u32) -> Box<dyn RenderTarget> {
        // Create framebuffer image
        // This is the color buffer of your framebuffer. When you render something onto this framebuffer, the color information is stored in this image. If you're blitting a sprite onto the framebuffer, the sprite's pixels will be written into this image.
        let image_desc = sg::ImageDesc {
            render_target: true,
            width: width as i32,
            height: height as i32,
            ..Default::default()
        };
        let image = sg::make_image(&image_desc);

        // Create framebuffer depth stencil
        // This is the depth buffer of your framebuffer. It's used for depth testing, which is a common technique in 3D rendering to determine which objects are in front of others.
        // Depth is for depth testing, DepthStencil is for both depth and stencil testing. Stencil testing is a technique to mask out certain parts of the framebuffer. It's often used for special effects like outlining objects, mirrors, decals, etc.
        let depth_image_desc = sg::ImageDesc {
            render_target: true,
            width: width as i32,
            height: height as i32,
            pixel_format: sg::PixelFormat::DepthStencil,
            ..Default::default()
        };
        let depth_image = sg::make_image(&depth_image_desc);

        // Create linear sampler
        // This is a sampler object. It's used to sample the color buffer when you blit it onto the screen. It's also used to sample textures when you render them onto the framebuffer.
        let sampler_desc = sg::SamplerDesc {
            min_filter: sg::Filter::Linear,
            mag_filter: sg::Filter::Linear,
            wrap_u: sg::Wrap::ClampToEdge,
            wrap_v: sg::Wrap::ClampToEdge,
            ..Default::default()
        };
        let sampler = sg::make_sampler(&sampler_desc);

        // Create framebuffer pass
        // This is the framebuffer pass. It's used to render onto the framebuffer. You can only render onto a framebuffer using a framebuffer pass.
        // This is the rendering pass that uses image and depth_image as its color and depth-stencil attachments, respectively. When you want to render to the framebuffer, you'll start this pass, issue your rendering commands, and then end the pass.
        let mut pass_desc = sg::PassDesc::default();
        pass_desc.color_attachments[0].image = image;
        pass_desc.depth_stencil_attachment.image = depth_image;
        let fb_pass = sg::make_pass(&pass_desc);

        // TODO: Error handling
        // let state_1 = sg::query_image_state(image);
        // let state_2 = sg::query_image_state(depth_image);
        // let state_3 = sg::query_pass_state(fb_pass);
        // let state_4 = sg::query_sampler_state(sampler);

        // println!("Image state: {:?}", state_1);
        // println!("Depth image state: {:?}", state_2);
        // println!("Pass state: {:?}", state_3);
        // println!("Sampler state: {:?}", state_4);
        
        Box::new(SokolRenderTarget {
            sprite: Box::new(SokolSprite {
                width,
                height,
                image: sg::Image { id: image.id },
            }),
            depth_image: sg::Image { id: depth_image.id },
            sampler: sg::Sampler { id: sampler.id },
            pass: sg::Pass { id: fb_pass.id },
        })
    }

    fn create_sprite(filename: &str) -> Box<dyn Sprite> {
        // let desc = unsafe { sg_query_image_desc(image) };
        // let (width, height) = (desc.width, desc.height);

        // Box::new(SokolSprite {
        //     width: width as u32,
        //     height: height as u32,
        //     image: sg::Image { id: image.id },
        // })
        unimplemented!()
    }

    fn blit_sprite(source: &Box<dyn Sprite>, sx: f32, sy: f32, sw: f32, sh: f32, destination: &Box<dyn RenderTarget>, dx: f32, dy: f32) {
        unsafe {      
            sgp_begin(sw as i32, sh as i32);
            sgp_project(0., sw, sh, 0.);
            sgp_set_color(0., 0., 0., 0.);
            sgp_clear();
            sgp_reset_color();
            sgp_set_blend_mode(sgp_blend_mode_SGP_BLENDMODE_BLEND);
            let sokol_source = source.as_any().downcast_ref::<SokolSprite>().unwrap();
            let sokol_destination = destination.as_any().downcast_ref::<SokolRenderTarget>().unwrap();
        
            // Set the source image
            sgp_set_image(0, sg_image { id: sokol_source.image.id });
        
            // Draw the source sprite onto the destination sprite
            let src_rect = sgp_rect { x: sx, y: sy, w: sw, h: sh };
            let dest_rect = sgp_rect { x: dx, y: dy, w: sw, h: sh };
            sgp_draw_textured_rect(0, dest_rect, src_rect);

            // Set the framebuffer as the current render target
            let pass_action = sg::PassAction::default();
            sg::begin_pass(sokol_destination.pass, &pass_action);

            sgp_flush();
            sgp_end();
        
            // End the pass to apply the drawing commands to the framebuffer
            sg::end_pass();
        }
    }

    fn resize_sprite(sprite: &Box<dyn Sprite>, width: u32, height: u32) {
        // let sokol_sprite = sprite.as_any().downcast_ref::<SokolSprite>().unwrap();
        // let old_image = sokol_sprite.image;

        // // Create a new image with the desired dimensions
        // let new_image_desc = sg_image_desc {
        //     width: width as i32,
        //     height: height as i32,
        //     // Copy other parameters from the old image
        //     ..unsafe { sg_query_image_desc(old_image) }
        // };
        // let _new_image = unsafe { sg_make_image(&new_image_desc) };

        // // TODO: Copy old image data into the new image

        // // Replace the old image with the new one
        // // sokol_sprite.image = new_image;

        // // Destroy the old image
        // unsafe { sg_destroy_image(old_image) };
        unimplemented!()
    }

    fn draw_sprite(sprite: &Box<dyn Sprite>, x: f32, y: f32) {
        unsafe {
            sgp_reset_color();
            sgp_set_blend_mode(sgp_blend_mode_SGP_BLENDMODE_BLEND);
            let game_config = World::get_singleton::<GameConfig>();
            let (window_width, _) = SokolRenderer2D::window_size();
            let scale_factor = window_width as f32 / game_config.get_resolution_width() as f32;
            let dest_rect = sgp_rect { 
                x: (x * scale_factor).round(), 
                y: (y * scale_factor).round(), 
                w: (sprite.width() as f32 * scale_factor).round(), 
                h: (sprite.height() as f32 * scale_factor).round()
            };
            let src_rect = sgp_rect { 
                x: 0., 
                y: 0., 
                w: sprite.width() as f32, 
                h: sprite.height() as f32 
            };
            let sokol_sprite = sprite.as_any().downcast_ref::<SokolSprite>().unwrap();
            sgp_set_image(0, sg_image { id: sokol_sprite.image.id });
            sgp_draw_textured_rect(0, dest_rect, src_rect);
        }
    }

    fn draw_filled_rect(pos: Position, size: Size, color: Color) {
        unsafe {
            let game_config = World::get_singleton::<GameConfig>();
            let (window_width, _) = SokolRenderer2D::window_size();
            let scale_factor = window_width as f32 / game_config.get_resolution_width() as f32;
            sgp_reset_color();
            sgp_set_color(color.get_r() as f32, color.get_g() as f32, color.get_b() as f32, color.get_a() as f32);
            sgp_draw_filled_rect(pos.get_x() as f32 * scale_factor, pos.get_y() as f32 * scale_factor, size.get_width() as f32 * scale_factor, size.get_height() as f32 * scale_factor);
        }
    }

    fn draw_line(ax: f32, ay: f32, bx: f32, by: f32) {
        unsafe {
            let game_config = World::get_singleton::<GameConfig>();
            let (window_width, _) = SokolRenderer2D::window_size();
            let scale_factor = window_width as f32 / game_config.get_resolution_width() as f32;
            sgp_draw_line(ax * scale_factor, ay * scale_factor, bx * scale_factor, by * scale_factor);
        }
    }

    fn clear_sprite(sprite: &Box<dyn RenderTarget>, x: i32, y: i32, width: i32, height: i32) {
        let sokol_render_target = sprite.as_any().downcast_ref::<SokolRenderTarget>().unwrap();
    
        unsafe {
            // The sgp_scissor function sets a scissor rectangle in the viewport. The scissor test is a per-sample operation performed after the fragment shader. It discards the fragment if the fragment's position lies outside the scissor rectangle. In other words, it restricts drawing to a certain rectangular area of the screen.
            // You need to call sgp_begin before you can set a scissor rectangle with sgp_scissor, and you need to call sgp_end when you're done.
            sgp_begin(width, height);
            // The sgp_project function sets the coordinate space boundary in the current viewport. It's used to define the 2D projection matrix for the rendering context. The parameters left, right, top, and bottom define the boundaries of the coordinate space. This function is typically used when you want to set up a specific 2D coordinate system for your rendering context.
            sgp_project(0., width as f32, height as f32, 0.);

            // Set the framebuffer as the current render target
            let pass_action = sg::PassAction {
                colors: [sg::ColorAttachmentAction {
                    load_action: sg::LoadAction::Load,
                    store_action: sg::StoreAction::Store,
                    clear_value: sg::Color::new(),
                }; sg::MAX_COLOR_ATTACHMENTS],
                depth: sg::DepthAttachmentAction {
                    load_action: sg::LoadAction::Load,
                    store_action: sg::StoreAction::Store,
                    clear_value: 0.0,
                },
                stencil: sg::StencilAttachmentAction {
                    load_action: sg::LoadAction::Load,
                    store_action: sg::StoreAction::Store,
                    clear_value: 0,
                },
                ..Default::default()
            };
            sg::begin_pass(sokol_render_target.pass, &pass_action);
    
            // Set a scissor rectangle to the desired area
            sgp_scissor(x, y, width, height);
    
            // Set the color to the clear color
            sgp_set_color(0., 0., 0., 0.); // Replace with your clear color
    
            // Draw a rectangle over the scissor area
            sgp_draw_filled_rect(x as f32, y as f32, width as f32, height as f32);
    
            // Reset the scissor rectangle to default
            sgp_reset_scissor();

            // Flush the draw commands to the 
            // The sgp_flush function dispatches the current Sokol GFX draw commands. It's used to ensure that all the draw commands that have been issued up to this point are sent to the GPU for rendering. This function doesn't end the current draw command queue, so you can continue issuing draw commands after calling sgp_flush.
            sgp_flush();

            // Finish the draw command queue, clearing it
            sgp_end();
    
            // End the pass to apply the drawing commands to the framebuffer
            sg::end_pass();
        }
    }

    fn clear_canvas(x: i32, y: i32, width: i32, height: i32) {
        unsafe {
            sgp_scissor(x, y, width, height);
            sgp_clear();
            sgp_reset_scissor();
        }
    }
}