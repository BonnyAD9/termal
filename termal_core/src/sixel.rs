use std::{collections::{BTreeMap, BTreeSet}, hint::black_box, ops::{Index, IndexMut}};

pub type Rgb = (u8, u8, u8);

pub trait SixelImage {
    fn sixel_width(&self) -> usize;
    fn sixel_height(&self) -> usize;
    fn sixel_get_pixel(&self, x: usize, y: usize) -> Rgb;
}

pub struct Mat<T> {
    width: usize,
    height: usize,
    data: Vec<T>,
}

impl SixelImage for Mat<Rgb> {
    fn sixel_width(&self) -> usize {
        self.width
    }

    fn sixel_height(&self) -> usize {
        self.height
    }

    fn sixel_get_pixel(&self, x: usize, y: usize) -> Rgb {
        self[(x, y)]
    }
}

impl<T> Mat<T> {
    pub fn new(width: usize, height: usize) -> Self where T: Default {
        let mut data = vec![];
        data.resize_with(width * height, Default::default);
        Self {
            width,
            height,
            data,
        }
    }

    pub fn from_vec(width: usize, height: usize, data: Vec<T>) -> Self {
        if data.len() != width * height {
            panic!("Invalid Mat data length of {} for [{width}, {height}]({})", data.len(), width * height);
        }
        Self {
            width,
            height,
            data
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn get_line(&self, y: usize) -> &[T] {
        let start = y * self.width;
        &self.data[start..start + self.width]
    }
}

impl<T> Index<(usize, usize)> for Mat<T> {
    type Output = T;

    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        if x > self.width || y > self.height {
            panic!(
                "Mat index [{x}, {y}] out of range of [{}, {}]",
                self.width,
                self.height
            );
        }
        &self.data[y * self.width + x]
    }
}

impl<T> IndexMut<(usize, usize)> for Mat<T> {
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut Self::Output {
        if x > self.width || y > self.height {
            panic!(
                "Mat index [{x}, {y}] out of range of [{}, {}]",
                self.width,
                self.height
            );
        }
        &mut self.data[y * self.width + x]
    }
}

#[derive(Default)]
struct Sixel([Rgb; 6]);

const NO_COLOR: Rgb = (0, 0, 0);

struct SixelState<'a, I> where I: SixelImage {
    lines: Mat<Sixel>,
    colors: BTreeMap<Rgb, usize>,
    img: &'a I,
    out: &'a mut String,
}

impl Sixel {
    pub fn from_img(img: &impl SixelImage, (x, y): (usize, usize)) -> Self {
        fn c_map(c: u8) -> u8 {
            (c as usize * 100 / 255) as u8
        }

        fn rgb_map((r, g, b): (u8, u8, u8)) -> Rgb {
            (c_map(r), c_map(g), c_map(b))
        }

        let mut data = [NO_COLOR; 6];

        for yo in y..img.sixel_height().min(y + 6) {
            data[yo - y] = rgb_map(img.sixel_get_pixel(x, yo));
            _ = black_box(0);
        }

        Self(data)
    }

    pub fn color_char(&self, rgb: Rgb) -> char {
        let mut code: u8 = 0;
        for (i, c) in self.0.iter().copied().enumerate() {
            if c == rgb {
                code |= 1 << i;
            }
        }

        (code + 63) as char
    }
}

pub fn push_sixel(out: &mut String, img: &impl SixelImage) {
    let mut state = SixelState {
        lines: Mat::new(img.sixel_width(), img.sixel_height() / 6),
        colors: BTreeMap::new(),
        img,
        out
    };

    state.encode();
}

impl<'a, I> SixelState<'a, I> where I: SixelImage {
    pub fn encode(&mut self) {
        *self.out += "\x1bPq";

        self.prepare();
        self.define_colors();

        for y in 0..self.lines.height() {
            self.draw_line(y);
        }

        *self.out += "\x1b\\";
    }

    pub fn prepare(&mut self) {
        let mut color_id = 1;
        for y in 0..self.lines.height() {
            for x in 0..self.lines.width() {
                let sx = Sixel::from_img(self.img, (x, y * 6));
                for &c in &sx.0 {
                    self.colors.entry(c).or_insert_with(|| {
                        color_id += 1;
                        color_id - 1
                    });
                }
                self.lines[(x, y)] = sx;
            }
        }
    }

    pub fn define_colors(&mut self) {
        for ((r, g, b), id) in &self.colors {
            *self.out += &format!("#{id};2;{r};{g};{b}");
        }
    }

    fn draw_line(&mut self, y: usize) {
        let line = self.lines.get_line(y);
        let mut line_colors = BTreeSet::new();
        for sx in line {
            line_colors.extend(sx.0);
        }

        for c in line_colors {
            *self.out += &format!("#{}", self.colors.get(&c).unwrap());
            for sx in line {
                self.out.push(sx.color_char(c));
            }
            self.out.push('$');
        }

        self.out.push('-');
    }
}
