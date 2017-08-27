mod input;
mod output;

pub use self::input::Input;
pub use self::output::Output;

#[derive(Clone,Copy)]
enum InputSignal {
    OpenInventory,
    CloseInventory,
    InventoryHighlight(usize),
}
