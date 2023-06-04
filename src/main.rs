// Biuld: wasm-pack build --target web
use wasm_wgpu_template::run;

fn main() {
    pollster::block_on(run());
}
