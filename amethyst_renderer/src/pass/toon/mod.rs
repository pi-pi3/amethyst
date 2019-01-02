pub use self::separate::DrawToonSeparate;

mod separate;

use crate::pass::util::TextureType;

static VERT_SRC: &[u8] = include_bytes!("../shaders/vertex/basic.glsl");
static FRAG_SRC: &[u8] = include_bytes!("../shaders/fragment/toon.glsl");

static TEXTURES: [TextureType; 2] = [TextureType::Albedo, TextureType::Emission];
