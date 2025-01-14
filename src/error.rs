#[derive(Debug)]
pub enum ClipboardErr {
    NotAvailable,
    CantSet,
    CantUpdate,
	CantGet,
}
