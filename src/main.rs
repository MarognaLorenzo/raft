mod component;
use component::{Component, Off, On, Broken};
fn main() {
    let component = Component::<Off>::new();
    let light = component.turn_on();
    let light = light.turn_off().turn_on().drop().lament();
}

