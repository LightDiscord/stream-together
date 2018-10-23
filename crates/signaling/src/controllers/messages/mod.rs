use super::super::messages::{ Incoming, Outcoming };
use super::super::controllers::{ Server, Spaceship };
use ws::Error;

pub trait Handler {
    fn handle(&self, connection: &mut Server, message: Incoming) -> Result<Outcoming, Error>;
}

macro_rules! register {
    ($( $path:ident => $name:ident ),*) => {
        $(
            mod $path;
            pub use self::$path::$name;
        )*
    };
}

register! {
    become_captain => BecomeCaptain,
    leave_crew => LeaveCrew
}
