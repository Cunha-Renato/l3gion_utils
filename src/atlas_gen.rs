use std::collections::HashMap;

use error::LgFontAtlasGeneratorResult;
use image::EncodableLayout;

pub mod error {
    use thiserror::Error;

    pub type LgFontAtlasGeneratorResult<T> = Result<T, LgFontAtlasGeneratorError>;

    #[derive(Debug, Error)]
    pub enum LgFontAtlasGeneratorError {
        #[error("LgFontAtlasGeneratorError: Failed to parse font file - {0}.")]
        ParseFail(String),
    }
    impl From<ttf_parser::FaceParsingError> for LgFontAtlasGeneratorError {
        fn from(value: ttf_parser::FaceParsingError) -> Self {
            Self::ParseFail(value.to_string())
        }
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct GlyphData {
    pub character: char,
    pub start_x: u32,
    pub start_y: u32,
    pub width: u32,
    pub height: u32,
    pub hor_advance: u32,
    pub ver_advance: u32,
    pub min_y: u32,
}

pub struct LgFontAtlasGenerator<'a> {
    face: ttf_parser::Face<'a>,
    texture: image::ImageBuffer<image::Rgb<u8>, Vec<u8>>,
    scale_factor: f64,
}
impl<'a> LgFontAtlasGenerator<'a> {
    pub fn from_bytes(bytes: &'a [u8], px: u32) -> LgFontAtlasGeneratorResult<Self> {
        let face = ttf_parser::Face::parse(bytes, 0)?;
        let scale_factor = px as f64 / face.units_per_em() as f64;

        Ok(Self {
            face,
            texture: image::RgbImage::new(0, 0),
            scale_factor,
        })
    }

    pub fn generate(&mut self, range: std::ops::Range<char>) -> HashMap<char, GlyphData> {
        let chars = range.collect::<Vec<_>>();
        let num_glyphs_hor = (chars.len() as f32).sqrt().ceil() as u32;
        let mut glyph_data_map = HashMap::with_capacity(chars.len());

        let mut atlas_data = chars
            .iter()
            .filter_map(|c| prepare_shape(&self.face, *c, 4.0, self.scale_factor))
            .collect::<Vec<_>>();
        atlas_data.sort_by(|a, b| b.0.height.cmp(&a.0.height));

        let mut max_height = 0;
        let mut max_width = 0;

        atlas_data
            .chunks_mut(num_glyphs_hor as usize)
            .for_each(|ck| {
                let mut temp_width = 0;
                let mut temp_height = 0;

                ck.iter_mut().for_each(|(data, _)| {
                    temp_width += data.width;
                    temp_height = temp_height.max(data.height);
                });

                max_width = max_width.max(temp_width);
                max_height += temp_height;
            });

        let mut atlas = image::RgbImage::new(max_width, max_height);
        let mut cursor = (0, 0);
        let mut row_height = 0;

        atlas_data.into_iter().for_each(|(mut data, sdf)| {
            if cursor.0 + data.width > max_width {
                cursor.0 = 0;
                cursor.1 += row_height;
                row_height = 0;
            }

            data.start_x = cursor.0;
            data.start_y = cursor.1;
            glyph_data_map.insert(data.character, data);

            row_height = row_height.max(data.height);

            for j in 1..data.height {
                for i in 0..data.width {
                    let x = i + cursor.0;
                    let y = j + cursor.1;

                    let pixel = sdf.get_pixel(i, data.height - j);
                    atlas.put_pixel(x, y, *pixel);
                }
            }

            cursor.0 += data.width;
        });

        self.texture = atlas;

        glyph_data_map
    }

    pub fn get_bytes(&self) -> &[u8] {
        self.texture.as_bytes()
    }

    pub fn get_texture(self) -> image::ImageBuffer<image::Rgb<u8>, Vec<u8>> {
        self.texture
    }
}

fn prepare_shape(
    face: &ttf_parser::Face,
    character: char,
    range: f64,
    scale_factor: f64,
) -> Option<(GlyphData, image::ImageBuffer<image::Rgb<u8>, Vec<u8>>)> {
    use fdsm::transform::Transform;
    use fdsm::*;

    let id = face.glyph_index(character)?;
    let mut shape = shape::Shape::load_from_face(face, id);
    let bbox = face.glyph_bounding_box(id)?;

    let width =
        ((bbox.x_max as f64 - bbox.x_min as f64) * scale_factor + 2.0 * range).ceil() as u32;
    let height =
        ((bbox.y_max as f64 - bbox.y_min as f64) * scale_factor + 2.0 * range).ceil() as u32;
    let hor_advance =
        (face.glyph_hor_advance(id).unwrap_or_default() as f64 * scale_factor).ceil() as u32;
    let ver_advance =
        (face.glyph_ver_advance(id).unwrap_or_default() as f64 * scale_factor).ceil() as u32;
    let min_y = (bbox.y_min as f64 * scale_factor).ceil() as u32;

    let transformation =
        nalgebra::convert::<_, nalgebra::Affine2<f64>>(nalgebra::Similarity2::new(
            nalgebra::Vector2::new(
                range - bbox.x_min as f64 * scale_factor,
                range - bbox.y_min as f64 * scale_factor,
            ),
            0.0,
            1.0 * scale_factor,
        ));

    shape.transform(&transformation);

    let colored_shape = shape::Shape::edge_coloring_simple(shape, 0.03, 69441337420);
    let glyph_data = GlyphData {
        character,
        width,
        height,
        hor_advance,
        ver_advance,
        min_y,
        ..Default::default()
    };

    let prepared_colored_shape = colored_shape.prepare();
    let mut msdf = image::RgbImage::new(glyph_data.width, glyph_data.height);
    generate::generate_msdf(&prepared_colored_shape, range, &mut msdf);
    render::correct_sign_msdf(
        &mut msdf,
        &prepared_colored_shape,
        bezier::scanline::FillRule::Nonzero,
    );

    Some((glyph_data, msdf))
}
