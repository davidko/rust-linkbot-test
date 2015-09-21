extern crate eventual;

use std::sync::{Arc, Mutex};
use eventual::*;

#[test]
fn it_works() {
    unsafe { 
        let linkbot = linkbotFromSerialId("LOCL");
        assert_eq!(0, linkbotMove(linkbot, 90.0, 90.0, 90.0));
    }
}

#[test]
fn check_binding() {
    check_move();
    check_accelerometer();
    ()
}

fn check_move() -> Result<(), u32> {
    let l = try!(Linkbot::new("LOCL"));
    try!(l.move_motors(90.0, 90.0, 90.0));
    Ok(())
}

fn check_accelerometer() -> Result<(f64, f64, f64), u32> {
    println!("Testing accelerometer...");
    let l = try!(Linkbot::new("LOCL"));
    let (x,y,z) = try!(l.get_accelerometer());
    println!("Accel values: {} {} {}", x, y, z);
    Ok((x,y,z))
}

fn test_future() -> Result<(), u32> {
    let mut l = try!(Linkbot::new("LOCL"));
    let l_sync = Arc::new(Mutex::new(l));
    let l = l_sync.clone();
    let future = Future::spawn(move || {
        let l = l_sync.clone();
        let l2 = l.lock();
        let l3 = l2.unwrap();
        l3.get_accelerometer().unwrap()
        //let mut l = l.lock().unwrap();
        //l.get_accelerometer().unwrap()
    });
    let (x,y,z) = future.await().unwrap();
    println!("future excel value: {} {} {}", x, y, z);
    Ok(())
}

pub struct Linkbot {
    linkbot_impl : *mut u8,
}

impl Drop for Linkbot {
    fn drop(&mut self) {
        unsafe {
            linkbotDelete(self.linkbot_impl);
        }
    }
}

unsafe impl Send for Linkbot { }

impl Linkbot {
    fn new(serial_id : &str) -> Result<Linkbot, u32> {
        unsafe {
            let p = linkbotFromSerialId(serial_id);
            if p.is_null() {
                return Err(1);
            } else {
                return Ok( Linkbot { linkbot_impl : p } );
            }
        }
    }

    fn move_motors(&self, theta1:f64, theta2:f64, theta3:f64) -> Result<(), u32> {
        unsafe {
            match linkbotMove(self.linkbot_impl, theta1, theta2, theta3) {
                0 => Ok(()),
                e @ _ => Err(e)
            }
        }
    }

    fn get_accelerometer(&self) -> Result<(f64, f64, f64), u32> {
        unsafe {
            let mut x: f64 = 0.0;
            let mut y: f64 = 0.0;
            let mut z: f64 = 0.0;
            match linkbotGetAccelerometer(self.linkbot_impl, 
                                          &mut x as *mut f64, 
                                          &mut y as *mut f64, 
                                          &mut z as *mut f64) {
                0 => Ok((x,y,z)),
                e @ _ => Err(e)
            }
        }
    }
}

#[link(name = "baromesh")]
extern {
    fn linkbotFromSerialId(serial_id: *const str) -> *mut u8;
    fn linkbotDelete(linkbot: *mut u8) -> u32;
    fn linkbotMove(linkbot: *mut u8, theta1:f64, theta2:f64, theta3:f64) -> u32;
    fn linkbotGetAccelerometer(linkbot: *mut u8, x:*mut f64, y:*mut f64, z:*mut f64) -> u32;
}

