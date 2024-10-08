use termal::{error::Result, sixel::{self, Mat, Rgb}};

fn main() -> Result<()> {
    let imgData = vec![
        (255, 255, 255), (0, 255, 0), (0, 255, 0), (0, 255, 0), (0, 255, 0), (255, 255, 255),
        (255, 0, 0), (255, 255, 255), (0, 255, 0), (0, 255, 0), (255, 255, 255), (0, 0, 255),
        (255, 0, 0), (255, 0, 0), (255, 255, 255), (255, 255, 255), (0, 0, 255), (0, 0, 255),
        (255, 0, 0), (255, 0, 0), (255, 255, 255), (255, 255, 255), (0, 0, 255), (0, 0, 255),
        (255, 0, 0), (255, 255, 255), (0, 0, 0), (0, 0, 0), (255, 255, 255), (0, 0, 255),
        (255, 255, 255), (0, 0, 0), (0, 0, 0), (0, 0, 0), (0, 0, 0), (255, 255, 255),
        (255, 255, 255), (0, 255, 0), (0, 255, 0), (0, 255, 0), (0, 255, 0), (255, 255, 255),
        (255, 0, 0), (255, 255, 255), (0, 255, 0), (0, 255, 0), (255, 255, 255), (0, 0, 255),
        (255, 0, 0), (255, 0, 0), (255, 255, 255), (255, 255, 255), (0, 0, 255), (0, 0, 255),
        (255, 0, 0), (255, 0, 0), (255, 255, 255), (255, 255, 255), (0, 0, 255), (0, 0, 255),
        (255, 0, 0), (255, 255, 255), (0, 0, 0), (0, 0, 0), (255, 255, 255), (0, 0, 255),
        (255, 255, 255), (0, 0, 0), (0, 0, 0), (0, 0, 0), (0, 0, 0), (255, 255, 255),
    ];
    let img: Mat<Rgb> = Mat::from_vec(6, 12, imgData);
    let mut res = String::new();
    sixel::push_sixel(&mut res, &img);
    println!("{}", res);

    Ok(())
}
