use termal::{
    error::Result,
    sixel::{self, RawImg},
};

fn main() -> Result<()> {
    let img_data = include_bytes!("img256.data");
    let img = RawImg::from_rgb(img_data.into(), 256, 256);
    let mut res = String::new();
    sixel::push_sixel(&mut res, &img);
    println!("{}", res);

    Ok(())
}
