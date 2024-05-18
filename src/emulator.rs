use vigem_client::{Client, XGamepad, Xbox360Wired};

pub struct Emulator {
    pub target: Xbox360Wired<Client>,
    pub gamepad: XGamepad,
}

impl Emulator {
    pub fn new() -> Self {
        let client = vigem_client::Client::connect().unwrap();
        let id = vigem_client::TargetId::XBOX360_WIRED;
        let mut target = Xbox360Wired::new(client, id);

        target.plugin().unwrap();
        target.wait_ready().unwrap();

        Emulator {
            target,
            gamepad: XGamepad::default(),
        }
    }

    pub fn update(&mut self) {
        self.target.update(&self.gamepad).unwrap();
    }
}
