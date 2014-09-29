#![crate_type = "lib"]
#![license = "MIT/ASL2"]
#![feature(globs, unsafe_destructor, phase)]

#[phase(plugin, link)] extern crate log;

extern crate libc;
extern crate debug;

extern crate libnanomsg;

mod result;


#[cfg(test)]
mod tests {
    #![allow(unused_must_use)]
    extern crate debug;

}
