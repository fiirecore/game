use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MessageStates<C: Clone + Into<[f32; 4]>> {
    Running(MessageState<C>),
    /// Has cooldown
    Finished(f32),
    None,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageState<C: Clone + Into<[f32; 4]>> {
    pub pages: Vec<MessagePage<C>>,

    pub page: usize,
    pub line: usize,
    pub text: f32,

    pub waiting: bool,
    pub wait: f32,

    pub cooldown: Option<f32>,
    // pub scroll: f32,
    // pub button: f32,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct MessagePage<C: Clone + Into<[f32; 4]>> {
    pub lines: Vec<String>,
    pub wait: Option<f32>,
    #[serde(default = "Option::default")]
    pub color: Option<C>,
    // #[serde(default = "Option::default")]
    // pub font: Option<F>,
}

impl<C: Clone + Into<[f32; 4]>> MessageState<C> {
    pub const DEFAULT_COOLDOWN: Option<f32> = Some(0.5);

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

impl<C: Clone + Into<[f32; 4]>> MessageStates<C> {
    pub fn is_running(&self) -> bool {
        matches!(self, MessageStates::Running(..))
    }

    pub fn as_ref(&self) -> Option<&MessageState<C>> {
        match self {
            MessageStates::Running(state) => Some(state),
            _ => None,
        }
    }

    pub fn as_mut(&mut self) -> Option<&mut MessageState<C>> {
        match self {
            MessageStates::Running(state) => Some(state),
            _ => None,
        }
    }

    pub fn get_or_insert_with<F: FnOnce() -> MessageState<C>>(
        &mut self,
        f: F,
    ) -> &mut MessageState<C> {
        match self {
            MessageStates::Running(state) => state,
            _ => {
                *self = Self::Running(f());
                if let Self::Running(state) = self {
                    state
                } else {
                    unreachable!()
                }
            }
        }
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
            cooldown: Self::DEFAULT_COOLDOWN,
        }
    }
}

impl<C: Clone + Into<[f32; 4]>> Default for MessageStates<C> {
    fn default() -> Self {
        Self::None
    }
}

impl<C: Clone + Into<[f32; 4]>> From<Vec<MessagePage<C>>> for MessageState<C> {
    fn from(pages: Vec<MessagePage<C>>) -> Self {
        Self {
            pages,
            ..Default::default()
        }
    }
}
