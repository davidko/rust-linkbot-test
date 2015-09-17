#[test]
fn it_works() {
    unsafe { 
        let linkbot = linkbotFromSerialId("LOCL");
        assert_eq!(0, linkbotMove(linkbot, 90.0, 90.0, 90.0));
    }
}

#[link(name = "baromesh")]
extern {
    fn linkbotFromSerialId(serial_id: *const str) -> *mut u8;
    fn linkbotMove(linkbot: *mut u8, theta1:f64, theta2:f64, theta3:f64) -> u32;
}
