use std::io::Write;
pub trait Renderable {
    fn render<W: Write>(&self, stdout: &mut W);
}