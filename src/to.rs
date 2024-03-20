pub trait To{
    fn to_const(&self) -> Option<*const()> {None}
    fn to_mut(&mut self) -> Option<*mut()> {None}
    fn to_self<T>(&self) -> Option<&T> {None}
    fn to_self_mut<T>(&mut self) -> Option<&mut T> {None}
}

impl To for *const() {
    fn to_self<T>(&self) -> Option<&T> {
        Some(unsafe { &*(*self as *const T)})
    }
}

impl To for *mut() {
    fn to_self_mut<T>(&mut self) -> Option<&mut T> {
        Some(unsafe { &mut *(*self as *mut T)})
    }
}

impl To for str{
    fn to_const(&self) -> Option<*const()> {
        Some(self as *const str as *const())
    }
    fn to_mut(&mut self) -> Option<*mut()> {
        Some(self as *mut str as *mut())
    }
}

impl To for i32{
    fn to_const(&self) -> Option<*const()> {
        Some(self as *const i32 as *const())
    }
    fn to_mut(&mut self) -> Option<*mut()> {
        Some(self as *mut i32 as *mut())
    }
}

impl To for char{
    fn to_const(&self) -> Option<*const()> {
        Some(self as *const char as *const())
    }
    fn to_mut(&mut self) -> Option<*mut()> {
        Some(self as *mut char as *mut())
    }
}
