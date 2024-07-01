use std::marker::PhantomData;

use egui::FontDefinitions;
use egui_demo_lib::DemoWindows;
use egui_wgpu_backend::RenderPass;
use egui_winit_platform::{Platform, PlatformDescriptor};

use crate::{app::App, context::Context};


pub struct Ui{
    //egui stuff
    pub platform: Platform,
    pub ui_render_pass: RenderPass,
    pub egui_demo_app: DemoWindows,
}

impl Ui{
    pub fn new(ctx: &Context,scale_factor: f64) -> Self{
        let platform = Platform::new(PlatformDescriptor{
            physical_width: ctx.size.width,
            physical_height: ctx.size.height,
            scale_factor,
            font_definitions: FontDefinitions::default(),
            style: Default::default(),
        });

        println!("ui scale factor: {}",scale_factor);
    
        let ui_render_pass = RenderPass::new(&ctx.device,ctx.config.format,1);
            
        let egui_demo_app = egui_demo_lib::DemoWindows::default();

        Self{
            platform,
            ui_render_pass,
            egui_demo_app,
        }
    }
}