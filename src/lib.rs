use std::sync::Once;
use vulkano::device::Device;
use vulkano::device::DeviceExtensions;
use vulkano::device::Features;
use vulkano::instance::Instance;
use vulkano::instance::InstanceExtensions;
use vulkano::instance::PhysicalDevice;

static START: Once = Once::new();

pub fn initialize() {
    START.call_once(|| {
        // Setup Vulkano instance
        let instance = Instance::new(None, &InstanceExtensions::none(), None)
            .expect("failed to create instance");
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
    })
}

#[cfg(test)]
mod tests {
    use crate::initialize;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn it_initializes() {
        initialize();
    }
}
