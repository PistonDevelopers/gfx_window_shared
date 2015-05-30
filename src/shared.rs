//! Normal variant (not thread-safe!)

use std::rc::Rc;
use std::cell::RefCell;
use gfx;
use gfx_device_gl;
use gfx::tex::Size;
use window::{ self, OpenGLWindow };

/// A wrapper around a simple shared reference to a Piston window.
///
/// This wrapper is not thread-safe.
pub struct SharedOutput<R: gfx::Resources, W: OpenGLWindow> {
    /// The shared window reference.
    pub window: Rc<RefCell<W>>,
    frame: gfx::handle::FrameBuffer<R>,
    mask: gfx::Mask,
    supports_gamma_conversion: bool,
    gamma: gfx::Gamma,
}

impl<R: gfx::Resources, W: OpenGLWindow> SharedOutput<R, W> {
    /// Try to set the gamma conversion.
    pub fn set_gamma(&mut self, gamma: gfx::Gamma) -> Result<(), ()> {
        if self.supports_gamma_conversion || gamma == gfx::Gamma::Original {
            self.gamma = gamma;
            Ok(())
        } else {
            Err(())
        }
    }
}

impl<R: gfx::Resources, W: OpenGLWindow> gfx::Output<R> for SharedOutput<R, W> {
    fn get_size(&self) -> (Size, Size) {
        let window::Size { width, height } = self.window.borrow().size();
        (width as Size, height as Size)
    }
    fn get_handle(&self) -> Option<&gfx::handle::FrameBuffer<R>> { Some(&self.frame) }
    fn get_mask(&self) -> gfx::Mask { self.mask }
    fn get_gamma(&self) -> gfx::Gamma { self.gamma }
}

impl<R: gfx::Resources, W: OpenGLWindow> gfx::Window<R> for SharedOutput<R, W> {
    fn swap_buffers(&mut self) { self.window.borrow_mut().swap_buffers(); }
}


/// Result of successful context initialization.
pub type SharedSuccess<W> = (
    gfx::OwnedStream<
        gfx_device_gl::Device,
        SharedOutput<gfx_device_gl::Resources, W>,
    >,
    gfx_device_gl::Device,
    gfx_device_gl::Factory,
);

/// Initialize with a window.
pub fn init_shared<W: OpenGLWindow>(window: Rc<RefCell<W>>) -> SharedSuccess<W> {
    use gfx::traits::StreamFactory;
    
    let mut window_lock = window.borrow_mut();
    window_lock.make_current();
    let (device, mut factory) = gfx_device_gl::create(|s| window_lock.get_proc_address(s));
    let out = SharedOutput {
        window: window.clone(),
        frame: factory.get_main_frame_buffer(),
        mask: gfx::COLOR | gfx::DEPTH | gfx::STENCIL,
        supports_gamma_conversion: true,
        gamma: gfx::Gamma::Original,
    };
    let stream = factory.create_stream(out);
    (stream, device, factory)
}
