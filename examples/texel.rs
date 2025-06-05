use termal::{
    error::Result,
    image::{self, RawImg},
};

fn main() -> Result<()> {
    let img_data = include_bytes!("img3_256.data");
    let img = RawImg::from_rgb(img_data.into(), 256, 256);
    let mut res = String::new();
    image::push_texel_quater_no_bg(
        &img,
        &mut res,
        "\n",
        Some(80),
        None,
        (0, 0, 0),
    );
    println!("{}", res);

    Ok(())
}
