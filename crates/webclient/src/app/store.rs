#[derive(Debug)]
pub enum Msg {
    Click,
    Tick,
}

pub struct Store {
    click_count: u32,
    ticks: u32,
    listeners: Vec<Box<(Fn() -> () + 'static)>>,
}

impl Store {
    pub fn new(count: u32) -> Store {
        Store {
            click_count: count,
            ticks: 0,
            listeners: vec![],
        }
    }

    pub fn subscribe(&mut self, callback: Box<Fn() -> ()>) {
        self.listeners.push(callback)
    }

    pub fn msg(&mut self, msg: &Msg) {
        match msg {
            Msg::Click => self.increment_click(),
            Msg::Tick => self.update_ticks(),
        };

        // Whenever we update state we'll let all of our state listeners know that state was
        // updated
        for callback in self.listeners.iter() {
            callback();
        }
    }

    pub fn click_count(&self) -> u32 {
        self.click_count
    }

    fn increment_click(&mut self) {
        self.click_count += 1;
    }

    fn update_ticks(&mut self) {
        self.ticks += 1;
    }

    pub fn ticks(&self) -> u32 {
        self.ticks
    }
}
