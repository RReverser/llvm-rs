use ffi::{core, LLVMMemoryBuffer};
use ffi::prelude::LLVMMemoryBufferRef;
use std::ops::Deref;
use std::raw::Slice;
use std::mem;
use util::{self, CBox, DisposeRef};


pub struct MemoryBuffer;
native_ref!(&MemoryBuffer = LLVMMemoryBufferRef);
impl MemoryBuffer {
    pub fn new_from_file(path: &str) -> Result<CBox<MemoryBuffer>, CBox<str>> {
        util::with_cstr(path, |path| unsafe {
            let mut output = mem::uninitialized();
            let mut error = mem::uninitialized();
            if core::LLVMCreateMemoryBufferWithContentsOfFile(path, &mut output, &mut error) == 1 {
                Err(CBox::new(error))
            } else {
                Ok(CBox::new(output))
            }
        })
    }
}
impl Deref for MemoryBuffer {
    type Target = str;
    fn deref(&self) -> &str {
        unsafe {
            mem::transmute(Slice {
                data: core::LLVMGetBufferStart(self.into()),
                len: core::LLVMGetBufferSize(self.into()) as usize
            })
        }
    }
}
impl DisposeRef for MemoryBuffer {
    type RefTo = LLVMMemoryBuffer;
    unsafe fn dispose(ptr: LLVMMemoryBufferRef) {
        core::LLVMDisposeMemoryBuffer(ptr)
    }
}
