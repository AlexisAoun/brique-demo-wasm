mod utils;

use wasm_bindgen::prelude::*;
use js_sys;
use brique::model::Model;
use brique::save_load::load_model_from_byte_stream;
use brique::matrix::Matrix;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn predict(model_byte_stream: js_sys::Uint8Array, input_image_data: js_sys::Float64Array) -> js_sys::Float64Array {
    let mut model: Model = load_model_from_byte_stream(&model_byte_stream.to_vec()).unwrap();
    
    
    let mut input_image: Matrix = Matrix::init(1, 28*28, input_image_data.to_vec());
    input_image.normalize();
    
    let score : Matrix = model.evaluate(&input_image, false);
    log(&score.convert_to_csv());
    js_sys::Float64Array::from(&score.data[..])
}
