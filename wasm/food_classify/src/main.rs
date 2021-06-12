use std::io::{self, Read};
use ssvm_tensorflow_interface;
use urlencoding::encode;
use ssvm_wasi_helper::ssvm_wasi_helper::_initialize;

pub fn main() {
  _initialize();
  let model_data: &[u8] = include_bytes!("lite-model_aiy_vision_classifier_food_V1_1.tflite");
  let labels = include_str!("aiy_food_V1_labelmap.txt");

  let mut buffer = Vec::new();
  io::stdin().read_to_end(&mut buffer).expect("Error reading from STDIN");

  let flat_img = ssvm_tensorflow_interface::load_jpg_image_to_rgb8(&buffer, 192, 192);

  let mut session = ssvm_tensorflow_interface::Session::new(&model_data, ssvm_tensorflow_interface::ModelType::TensorFlowLite);
  session.add_input("input", &flat_img, &[1, 192, 192, 3])
         .run();
  let res_vec: Vec<u8> = session.get_output("MobilenetV1/Predictions/Softmax");

  let mut i = 0;
  let mut max_index: i32 = -1;
  let mut max_value: u8 = 0;
  while i < res_vec.len() {
      let cur = res_vec[i];
      if cur > max_value {
          max_value = cur;
          max_index = i as i32;
      }
      i += 1;
  }

  let mut confidence = "可能有";
  if max_value > 200 {
      confidence = "非常可能有";
  } else if max_value > 125 {
      confidence = "很可能有";
  } else if max_value > 50 {
      confidence = "可能有";
  }

  let mut label_lines = labels.lines();
  for _i in 0..max_index {
    label_lines.next();
  }

  let class_name = label_lines.next().unwrap();

  if max_value > 50 {
    let s = format!(r#"
    {{
      "type": "post",
      "msg": [
        [{{
            "tag": "text",
            "text": "上传的图片里面{} "
          }},
          {{
            "tag": "a",
            "href": "https://www.bing.com/search?q={}",
            "text": "{}"
          }}
        ]
      ]
    }}
    "#, confidence.to_string(), encode(class_name), class_name).replace("\n", "");
    println!(r#"{{"result": {}}}"#, s);
  } else {
    println!("上传的图片里面没有检测到食品");
  }
}

#[cfg(test)]
mod tests {
  #[test]
  fn it_works() {
  }
}
