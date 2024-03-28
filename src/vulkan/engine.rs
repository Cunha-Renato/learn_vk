use std::{collections::HashSet, ffi::CStr, os::raw::c_void, rc::Rc};
use sllog::*;
use vulkanalia::{
    loader::{LibloadingLoader, LIBRARY}, vk::{self, DebugUtilsMessengerEXT, Device, EntryV1_0, ExtDebugUtilsExtension, HasBuilder, SurfaceKHR, }, Entry, Instance,
    window as vk_window,
};
use winit::window::Window;
use crate::MyError;

const VALIDATION_ENABLED: bool = cfg!(debug_assertions);
const VALIDATION_LAYER: vk::ExtensionName = vk::ExtensionName::from_bytes(b"VK_LAYER_KHRONOS_validation");
const DEVICE_EXTENSIONS: &[vk::ExtensionName] = &[vk::KHR_SWAPCHAIN_EXTENSION.name];

struct EngineData {
    instance: Instance,
    messenger: DebugUtilsMessengerEXT,
    gpu: vk::PhysicalDevice,
    device: Device,
    surface: SurfaceKHR,
}

pub struct VulkanEngine {
    frame_number: usize,
    window: Rc<Window>,
    pub stop_rendering: bool,
    data: EngineData,
}
impl VulkanEngine {
    pub unsafe fn new(window: Window) -> Result<Self, MyError> {
        let loader = LibloadingLoader::new(LIBRARY)?;
        let entry = Entry::new(loader).unwrap();

        let frame_number = 0;
        let stop_rendering = false;
        
        let (instance, messenger) = init_vulkan(
            entry,
            &window,
        )?;

        init_swapchain();
        init_commands();
        init_sync_structures();

        Ok(Self {
            frame_number,
            stop_rendering,
            window: Rc::new(window),
        })
    }
    
    pub unsafe fn destroy(&mut self) -> Result<(), MyError> {
        todo!()
    }

    pub unsafe fn render(&mut self) -> Result<(), MyError> {
        todo!()
    }
}
unsafe fn init_vulkan(
    entry: Entry,
    window: &Window,
) -> Result<(Instance, DebugUtilsMessengerEXT), MyError> {
    // Application Info
    let application_info = vk::ApplicationInfo::builder()
    .application_name(b"Vulkan Engine\0")
    .application_version(vk::make_version(1, 3, 0))
    .engine_name(b"No Engine\0")
    .engine_version(vk::make_version(1, 3, 0))
    .api_version(vk::make_version(1, 3, 0));

    // Layers

    let available_layers = entry
        .enumerate_instance_layer_properties()?
        .iter()
        .map(|l| l.layer_name)
        .collect::<HashSet<_>>();

    if VALIDATION_ENABLED && !available_layers.contains(&VALIDATION_LAYER) {
        return Err("Validation layer requested but not supported.".into());
    }

    let layers = if VALIDATION_ENABLED {
        vec![VALIDATION_LAYER.as_ptr()]
    } else {
        Vec::new()
    };

    // Extensions

    let mut extensions = vk_window::get_required_instance_extensions(window)
        .iter()
        .map(|e| e.as_ptr())
        .collect::<Vec<_>>();

    // Required by Vulkan SDK on macOS since 1.3.216.
    let flags = vk::InstanceCreateFlags::empty();

    if VALIDATION_ENABLED {
        extensions.push(vk::EXT_DEBUG_UTILS_EXTENSION.name.as_ptr());
    }

    // Create

    let mut info = vk::InstanceCreateInfo::builder()
        .application_info(&application_info)
        .enabled_layer_names(&layers)
        .enabled_extension_names(&extensions)
        .flags(flags);

    let mut debug_info = vk::DebugUtilsMessengerCreateInfoEXT::builder()
        .message_severity(vk::DebugUtilsMessageSeverityFlagsEXT::all())
        .message_type(vk::DebugUtilsMessageTypeFlagsEXT::all())
        .user_callback(Some(debug_callback));

    if VALIDATION_ENABLED {
        info = info.push_next(&mut debug_info);
    }

    let instance = entry.create_instance(&info, None)?;

    // Messenger

    let mut messenger = DebugUtilsMessengerEXT::default();
    if VALIDATION_ENABLED {
        messenger = instance.create_debug_utils_messenger_ext(&debug_info, None)?;
    }

    Ok((instance, messenger))
}
unsafe fn init_swapchain() {
    todo!() 
}
unsafe fn init_commands() {
    todo!() 
}
unsafe fn init_sync_structures() {
    todo!()
}

extern "system" fn debug_callback(
    severity: vk::DebugUtilsMessageSeverityFlagsEXT,
    type_: vk::DebugUtilsMessageTypeFlagsEXT,
    data: *const vk::DebugUtilsMessengerCallbackDataEXT,
    _: *mut c_void,
) -> vk::Bool32 
{
    let data = unsafe { *data };
    let message = unsafe { CStr::from_ptr(data.message) }.to_string_lossy();
    
    match severity {
        vk::DebugUtilsMessageSeverityFlagsEXT::ERROR => 
            error!("({:?}) {}", type_, message),

        vk::DebugUtilsMessageSeverityFlagsEXT::WARNING => 
            warn!("({:?}) {}", type_, message),

        vk::DebugUtilsMessageSeverityFlagsEXT::INFO => 
            info!("({:?}) {}", type_, message),

        _ => trace!("({:?}) {}", type_, message)
    }
    
    vk::FALSE
}