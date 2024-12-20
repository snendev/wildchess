use wasm_bindgen::prelude::*;

#[wasm_bindgen]
unsafe extern "C" {
    unsafe fn setInterval(closure: &Closure<dyn FnMut()>, millis: u32) -> u64;
    unsafe fn clearInterval(token: u64);
}

#[derive(Debug, PartialEq)]
pub(crate) struct Interval(u64);

impl Interval {
    pub(crate) fn new(func: &Closure<dyn FnMut()>, millis: u32) -> Self {
        // SAFETY: `setInterval` is defined by the browser.
        let interval_id = unsafe { setInterval(func, millis) };
        Self(interval_id)
    }
}

impl Drop for Interval {
    fn drop(&mut self) {
        // SAFETY: any `Interval` created by `setInterval` must be dropped using `clearInterval`, which is defined by the browser.
        unsafe {
            clearInterval(self.0);
        }
    }
}
