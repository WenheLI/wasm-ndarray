mod utils;

use ndarray::*;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u8(a : u8);
}

#[wasm_bindgen]
pub struct ND {
    instance: Array<f64, IxDyn>
}

impl ND {
    pub fn new(arr: Array<f64, IxDyn>) -> ND {
        ND {
            instance: arr
        }
    }
}

// for create
#[wasm_bindgen]
impl ND {
    pub fn zeros(shape: Vec<usize>) -> ND {
        let arr = ndarray::Array::<f64, _>::zeros(shape);
        ND::new(arr)
    }

    pub fn ones(shape: Vec<usize>) -> ND {
        let arr = ndarray::Array::<f64, _>::ones(shape);
        ND::new(arr)
    }

    pub fn from_array(arr: Vec<f64>, shape: Vec<usize>) -> ND {
        let arr = Array::from_shape_vec(shape, arr);
        match arr {
            Ok(arr_inst) => ND::new(arr_inst),
            Err(error) => wasm_bindgen::throw_str(&error.to_string())
        }
    }

    // pub fn eye(n: i32) -> ND {
    //     if (n <= 0) {
    //         wasm_bindgen::throw_str("Should be a number greater than zero")
    //     }
    //     let arr = Array::eye(n as usize);
    // }

    // pub fn range(start: f64, end: f64, step: f64) -> ND {
    //     let arr = Array::range(start, end, step);
        
    //     ND::new()
    // }
}

// access prop
#[wasm_bindgen]
impl ND {
    pub fn print(&self) -> String {
        self.instance.to_string()
    }
    pub fn ndim(&self) -> usize {
        self.instance.ndim()
    }
    pub fn len(&self) -> usize {
        self.instance.len()
    }
    pub fn shape(&self) -> Vec<usize> {
        self.instance.shape().to_vec()
    }
    pub fn data(&self) -> Vec<f64> {
        self.instance.as_slice().unwrap().to_vec()
    }
}

// matrix op