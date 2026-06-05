use burn_onnx::ModelGen;

fn main() {
    ModelGen::new()
        .input("../model/multipa_sim.onnx")
        .out_dir("model/")
        .run_from_script();
}
