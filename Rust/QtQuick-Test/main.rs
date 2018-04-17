// QML Rust Bindings Test (https://github.com/cyndis/qmlrs)
extern crate qmlrs; 

fn main() {
	let mut engine = qmlrs::Engine::new(); // What is 'mut'? -- Mutability in Rust
	
	engine.load_local_file("test_ui.qml");
	engine.exec();
}
