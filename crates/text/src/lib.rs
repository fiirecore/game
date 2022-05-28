use serde::{Deserialize, Serialize};

pub type FontId = u8;

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct MessagePage<C: Clone + Into<[f32; 4]>> {
    pub lines: Vec<String>,
    pub wait: Option<f32>,
    #[serde(default = "Option::default")]
    pub color: Option<C>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageState<C: Clone + Into<[f32; 4]>> {
    pub pages: Vec<MessagePage<C>>,

    pub page: usize,
    pub line: usize,
    pub text: f32,
    
    pub waiting: bool,
    pub wait: f32,

    // pub scroll: f32,
    // pub button: f32,

}

impl<C: Clone + Into<[f32; 4]>> MessageState<C> {
    pub fn new(pages: Vec<MessagePage<C>>) -> Self {
        Self {
            pages,
            page: Default::default(),
            line: Default::default(),
            text: Default::default(),
            waiting: Default::default(),
            wait: Default::default(),
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
        self.text = 0.0;
        self.wait = 0.0;
        // self.button = 0.0;
        // self.scroll = 0.0;
    }
}

impl<C: Clone + Into<[f32; 4]>> From<Vec<MessagePage<C>>> for MessageState<C> {
    fn from(pages: Vec<MessagePage<C>>) -> Self {
        Self::new(pages)
    }
}

impl<C: Clone + Into<[f32; 4]>> Default for MessageState<C> {
    fn default() -> Self {
        Self {
            pages: Default::default(),
            page: Default::default(),
            line: Default::default(),
            text: Default::default(),
            waiting: Default::default(),
            wait: Default::default(),
        }
    }
}