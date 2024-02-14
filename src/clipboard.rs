use copypasta::{ClipboardContext, ClipboardProvider};

pub fn copy_to_clipboard(
    content: &str,
) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    let mut ctx = ClipboardContext::new().unwrap();
    ctx.set_contents(content.to_string())?;
    Ok(())
}
