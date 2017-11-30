extern crate piske;
extern crate sindra;

mod test_utils;
use test_utils::examine_translated_source;

#[test]
fn test_examine() {
    let prog = r#"
let height = 1024;
let width = 1024;
set_image_dims(height, width);

let camera_center = -0.5 + 0i;
let camera_size = 3 + 3i;

let threshold = 10;
let num_max_iters = 1000;

iterate row = [0, height) {
    iterate col = [0, width) {
        let z = 0 + 0i;
        let c = project(row, col, camera_center, camera_size);
        let value = iterate over [0, num_max_iters) {
            z = z * z + c;
            let escape_value = re(z * z`);
            if escape_value > threshold {
                break escape_value;
            }
            0.0
        };
        set_pixel_data(row, col, value);
    }
}

write("output.png");
"#;

    examine_translated_source(prog);
}
