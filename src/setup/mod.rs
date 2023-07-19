pub mod main_mod {
    use crate::args::{Cli, Commands};
    use crate::funcs::change_brightness;
    use brightness::Brightness;
    use clap::Parser;
    use futures::TryStreamExt;

    pub async fn main_launch() -> Result<(), brightness::Error> {
        let cli = Cli::parse();
        change_brightness(cli).await
    }
}
