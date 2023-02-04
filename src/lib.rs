use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;

#[wasm_bindgen]
#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Item {
    sort_key: usize,
}

#[wasm_bindgen]
pub fn sort(js_array: JsValue) -> JsValue {
    let mut arr: Vec<Item> = serde_wasm_bindgen::from_value(js_array).unwrap();
    arr.sort_by(|a, b| a.sort_key.cmp(&b.sort_key));

    serde_wasm_bindgen::to_value(&arr).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;

    #[wasm_bindgen_test]
    fn it_works() {
        let items = vec![
            Item { sort_key: 4 },
            Item { sort_key: 3 },
            Item { sort_key: 5 },
            Item { sort_key: 1 },
        ];
        let js_array = serde_wasm_bindgen::to_value(&items).unwrap();

        let result = sort(js_array);
        let deserialized_result: Vec<Item> = serde_wasm_bindgen::from_value(result).unwrap();

        let expected = vec![
            Item { sort_key: 1 },
            Item { sort_key: 3 },
            Item { sort_key: 4 },
            Item { sort_key: 5 },
        ];

        assert_eq!(deserialized_result, expected);
    }
}
