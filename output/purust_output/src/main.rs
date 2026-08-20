fn main() {
    let mut _effect = Purs_Main::main();
    (_effect.unwrap_func())(purust_core::Value::Record(perceus_ptr::PerceusPtr::new(purust_core::Record_a { ..Default::default() })));
}
