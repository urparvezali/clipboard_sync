use arboard::Clipboard;

pub struct ClipboardManager {
    clipboard: Clipboard,
    last: Option<String>,
}
impl Default for ClipboardManager {
    fn default() -> Self {
        ClipboardManager::new()
    }
}
impl ClipboardManager {
    pub fn new() -> ClipboardManager {
        let clipboard = Clipboard::new().unwrap();
        Self {
            clipboard,
            last: None,
        }
    }
    pub fn update(&mut self, curr: String) {
        self.clipboard.set_text(curr.clone()).unwrap();
        self.last = Some(curr.to_string());
    }
    pub fn get(&mut self) -> String {
        self.clipboard.get_text().unwrap_or_default()
    }
    pub fn get_and_update(&mut self) {
        let from_local = self.get();
        if self.last.is_some() && self.last.clone().unwrap() == from_local {
            return;
        }
        self.last = Some(from_local);
    }
}
