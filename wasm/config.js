import init, { WasmSimulationData } from './path_to_generated_wasm.js';

async function run() {
    await init();
    const config = WasmSimulationData.new_from_json('{"contact": {"friction_coefficient": 0.1}}');
    const jsonStr = config.get_json();
    console.log(jsonStr);
}

run();
