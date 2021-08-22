use super::{Hint, Stt};
use std::sync::Arc;
use core::mem::MaybeUninit;
use vulkano::{
    instance::{Instance, PhysicalDevice},
    device::{Device, DeviceExtensions, Queue}
};

type EventLoop = winit::event_loop::EventLoop <()>;

static mut INSTANCE: MaybeUninit <Arc <Instance>> = MaybeUninit::uninit();
static mut PHYSICAL: usize = usize::MAX;
static mut EVENTLOOP: MaybeUninit <EventLoop> = MaybeUninit::uninit();
static mut DEVICE: MaybeUninit <Arc <Device>> = MaybeUninit::uninit();
static mut QUEUE: MaybeUninit <Arc <Queue>> = MaybeUninit::uninit();

impl super::Stt {
    #[inline(always)]
    pub fn instance() -> &'static mut Arc <Instance> {
    unsafe { INSTANCE.assume_init_mut() }
}

    pub fn physical() -> PhysicalDevice <'static> {
        unsafe { PhysicalDevice::from_index(Self::instance(), PHYSICAL).unwrap() }
    }

    #[inline(always)]
    pub fn eventloop() -> &'static mut EventLoop {
    unsafe { EVENTLOOP.assume_init_mut() }
}

    #[inline(always)]
    pub fn device() -> &'static mut Arc <Device> {
    unsafe { DEVICE.assume_init_mut() }
}

    #[inline(always)]
    pub fn queue() -> &'static mut Arc <Queue> {
    unsafe { QUEUE.assume_init_mut() }
}
}

pub unsafe fn initialize(_hint: Hint) {
    INSTANCE.write(Instance::new(None, &vulkano_win::required_extensions(), None).unwrap());

    EVENTLOOP.write(EventLoop::new());

    PHYSICAL = 0;

    let queue_family = Stt::physical()
        .queue_families()
        .find(|&q| q.supports_graphics())
        .unwrap();

    let device_ext = DeviceExtensions {
        khr_swapchain: true,
        ..DeviceExtensions::none()
    };

    let (device, mut queues) = Device::new(
        Stt::physical(),
        Stt::physical().supported_features(),
        &device_ext,
        [(queue_family, 0.5)].iter().cloned()
    ).unwrap();

    DEVICE.write(device);

    let queue = queues.next().unwrap();

    QUEUE.write(queue);
}
