use rhai_dylib::plugin_loader::{dylib::Libloading, PluginLoader};

fn main() {
    // Inspect the type id generated by the binary.
    println!(
        "main: {:?}",
        std::any::TypeId::of::<rhai_dylib::rhai::Map>()
    );

    let mut loader = Libloading::new();
    let mut engine = rhai_dylib::rhai::Engine::new();

    // Load the plugin. TODO: impl windows / mac support.
    loader
        .load("./plugin/target/debug/libplugin.so")
        .expect("failed to load plugin");

    // Register the plugin's module into the engine.
    loader
        .apply(&mut engine)
        .expect("failed to apply plugin API");

    engine
        .run(
            r"
print_stuff();
debug(triple_add(1, 2, 3));
",
        )
        .expect("failed to execute script");
}