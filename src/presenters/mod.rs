pub mod cmd;

pub trait Presenter {
  fn render(&self);
}
