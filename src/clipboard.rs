pub enum Mime {
    None,
}

struct Clipboard {
    index: u8,
    //pages: [Mime; 256],
    pages: Vec<Mime>,
}
impl Clipboard {
    fn clear(&self) {
        self.pages = [Mime::None; 256];
    }
    fn set_index(&self, index_new: u8) {
        self.index = index.new;
    }
    fn clip_end(&self) {
        self.index = 0;
    }
}
