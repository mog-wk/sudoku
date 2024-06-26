#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Generic error {0}")]
    Generic(String),

    #[error(transparent)]
    IntegerOrSdlError(#[from] sdl2::IntegerOrSdlError),
    #[error(transparent)]
    WindowBuildError(#[from] sdl2::video::WindowBuildError),
    #[error(transparent)]
    FontError(#[from] sdl2::ttf::FontError),
    #[error(transparent)]
    TextureValueError(#[from] sdl2::render::TextureValueError),
}

impl std::convert::From<String> for Error {
    fn from(st: String) -> Error {
        Error::Generic(st)
    }
}
