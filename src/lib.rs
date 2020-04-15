use std::sync::Arc;

use vulkano::device::Device;
use vulkano::device::DeviceExtensions;
use vulkano::device::Features;
//use vulkano::device::Queue;

use vulkano::instance::Instance;
use vulkano::instance::PhysicalDevice;

//use vulkano::swapchain::Surface;

use vulkano_win::VkSurfaceBuild;
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder
};

pub struct Context {
    instance: Arc<Instance>
}

impl Context {
    pub fn new() -> Context {
        // Setup Vulkano instance
        let instance = {
            // We need extensions to render in a Window
            let extensions = vulkano_win::required_extensions();
            Instance::new(None, &extensions, None).expect("failed to create Vulkan instance")
        };

        println!("Found an instance");

        // Find a device that supports Vulcan
        // Currently only one such device is expected, so pick the first
        let physical = PhysicalDevice::enumerate(&instance)
            .next()
            .expect("no physical device available");
        println!("Found a physical device");

        // Pick a connection (queue) into our physical device, which supports graphics
        let queue_family = physical
            .queue_families()
            .find(|&q| q.supports_graphics())
            .expect("couldn't find a graphical queue family");
        println!("Found a queue family");

        // Get a Device, and the queues into it
        let (_device, mut queues) = {
            Device::new(
                physical,
                &Features::none(),
                &DeviceExtensions::none(),
                [(queue_family, 0.5)].iter().cloned(),
            )
            .expect("failed to create device")
        };
        println!("Found a device");

        // However (and the reasons are unclear) we only need the first queue
        let _queue = queues.next().unwrap();
        println!("Found a queue");

        println!("Venus initialized!");

        Context {instance}
    }

    pub fn run_event_loop(&self) {
        // We need a Window to render in
        let event_loop = EventLoop::new();
        let _surface = WindowBuilder::new()
            .build_vk_surface(&event_loop, self.instance.clone())
            .unwrap();

        event_loop.run(|event, _, control_flow| {
            // Default, busy loop
            *control_flow = ControlFlow::Poll;
            match event {
                Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => {
                    *control_flow = ControlFlow::Exit;
                },
                _ => ()
            }
        });
    }
}

#[cfg(test)]
mod tests {
    use crate::Context;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn it_initializes() {
        let context = Context::new();
        context.run_event_loop()
    }
}
