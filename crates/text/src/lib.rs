use serde::{Deserialize, Serialize};

pub type FontId = u8;

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct MessagePage<C: Clone + Into<[f32; 4]>> {
    pub lines: Vec<String>,
    pub wait: Option<f32>,
    pub color: C,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct MessageState<F, C: Clone + Into<[f32; 4]>> {
    pub font: F,
    pub pages: Vec<MessagePage<C>>,

    pub page: usize,
    pub line: usize,
    pub accumulator: f32,
    pub scroll: f32,

    pub waiting: bool,
}

impl<F, C: Clone + Into<[f32; 4]>> MessageState<F, C> {

    pub fn new(font: F, pages: Vec<MessagePage<C>>) -> Self {
        Self {
            font,
            pages,
            page: Default::default(),
            line: Default::default(),
            accumulator: Default::default(),
            scroll: Default::default(),
            waiting: Default::default(),
        }
    }

    pub fn page(&self) -> usize {
        self.page
    }

    pub fn pages(&self) -> usize {
        self.pages.len()
    }

    pub fn waiting(&self) -> bool {
        self.waiting
    }

    pub fn reset_page(&mut self) {
        self.line = 0;
        self.accumulator = 0.0;
        self.scroll = 0.0;
    }
}

impl<F: Default, C: Clone + Into<[f32; 4]>> From<Vec<MessagePage<C>>> for MessageState<F, C> {
    fn from(pages: Vec<MessagePage<C>>) -> Self {
        Self::new(Default::default(), pages)
    }
}