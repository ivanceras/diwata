use dataview::DataView;
use dataview::Field;
use dataview::Type;

#[derive(Debug,Clone)]
pub enum Msg {
    Click,
    Tick,
}

pub struct Store {
    dataview: DataView,
    click_count: u32,
    ticks: u32,
    listeners: Vec<Box<(Fn() -> () + 'static)>>,
}

impl Store {
    pub fn new(count: u32) -> Store {
        sauron::log("creating a store");

        let fields = vec![
            Field {
                name: "pl".into(),
                sql_type: Type::Text,
                description: None,
                tags: vec![],
            },
            Field {
                name: "compiler".into(),
                sql_type: Type::Text,
                description: None,
                tags: vec![],
            },
            Field {
                name: "speed".into(),
                sql_type: Type::Text,
                description: None,
                tags: vec![],
            },
            Field {
                name: "vm".into(),
                sql_type: Type::Text,
                description: None,
                tags: vec![],
            },
            Field {
                name: "size".into(),
                sql_type: Type::Text,
                description: None,
                tags: vec![],
            },
            Field {
                name: "version".into(),
                sql_type: Type::Int,
                description: None,
                tags: vec![],
            },
        ];
        let csv = r#"
pl,version,speed,vm,size,compiler
rust,1,fast,false,small,rustc
haskel,1,fast,false,small,ghc
c,99,fast,false,small,clang
java,8,medium,true,large,jdk
            "#;
        let dataview = DataView::from_csv(fields, csv);
        sauron::log(format!("{:?}", dataview));
        Store {
            click_count: count,
            ticks: 0,
            listeners: vec![],
            dataview,
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
