extern crate ccs811;
extern crate embedded_hal_mock as hal;

mod common;
use common::{destroy, new};

#[test]
fn can_create_and_destroy() {
    let sensor = new(&[]);
    destroy(sensor);
}
