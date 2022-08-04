use image::{DynamicImage, GenericImageView, Pixel};

pub fn download(pokemon: &str, side: &str) -> anyhow::Result<Vec<u8>> {
    let pokemon = if pokemon == "castform" && side != super::ICON {
        "castform/normal"
    } else {
        pokemon
    };
    let bytes = attohttpc::get(&format!(
        "https://raw.githubusercontent.com/pret/pokefirered/master/graphics/pokemon/{}/{}.png",
        pokemon, side
    ))
    .send()?
    .bytes()?;
    let mut image = image::load_from_memory(&bytes)?;
    let (top, bottom) = get_heights(&image);
    image.crop(0, top, image.width(), bottom - top + 1);

    let mut writer = std::io::Cursor::new(Vec::new());
    image.write_to(&mut writer, image::ImageFormat::Png)?;

    Ok(writer.into_inner())
}

fn get_heights(image: &DynamicImage) -> (u32, u32) {
    let mut top = 0;
    let mut bottom = image.height();

    for b_counter in 0..image.height() {
        if !transparent_row(image, b_counter) {
            top = b_counter;
            break;
        }
    }

    for t_counter in (0..image.height()).rev() {
        if !transparent_row(image, t_counter) {
            bottom = t_counter;
            break;
        }
    }

    (top, bottom)
}

fn transparent_row(image: &DynamicImage, y: u32) -> bool {
    for x in 0..image.width() {
        if !transparent(image, x, y) {
            return false;
        }
    }
    return true;
}

fn transparent(image: &DynamicImage, x: u32, y: u32) -> bool {
    image.get_pixel(x, y).channels()[3] == 0
}
