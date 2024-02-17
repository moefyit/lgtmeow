use crate::cli::args::ChooseArgs;
#[cfg(feature = "copy")]
use crate::clipboard;
use crate::config::Config;
use crate::kitchen::partial_data::get_partial_metadata;
use crate::reply;
use console::style;
use rand::prelude::SliceRandom;

pub fn choose(args: ChooseArgs) -> std::io::Result<()> {
    if !Config::exists() {
        eprintln!("Please run `lgtmeow setup` first.");
        Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Config not found",
        ))?;
    }
    let metadata = get_partial_metadata();
    let config = Config::load().unwrap();
    let replies = reply::load_saved_replies_from_config(config, &metadata);
    let selected_reply;
    if args.random {
        selected_reply = replies.choose(&mut rand::thread_rng()).unwrap();
        eprintln!("{}", selected_reply.title);
        println!("{}", selected_reply.content);
    } else {
        cliclack::intro(style(" Choose LGTMeow 🐾 ").on_cyan().black())?;
        let selected_idx = cliclack::select("Choose a emoji pair to create LGTMeow 🐾")
            .items(
                &replies
                    .iter()
                    .enumerate()
                    .map(|(idx, reply)| (idx, reply.title.clone(), "".to_string()))
                    .collect::<Vec<(_, _, _)>>(),
            )
            .interact()?;
        selected_reply = &replies[selected_idx];
        cliclack::log::success(&selected_reply.title)?;
        cliclack::log::info(&selected_reply.content)?;
        cliclack::outro(style("Choose LGTMeow 🐾 successfully!").green())?;
    }
    if args.copy {
        #[cfg(not(feature = "copy"))]
        {
            panic!("Copy feature is not enabled, please recompile with `--features copy`");
        }
        #[cfg(feature = "copy")]
        {
            clipboard::copy_to_clipboard(&selected_reply.content).unwrap();
        }
    }

    Ok(())
}
