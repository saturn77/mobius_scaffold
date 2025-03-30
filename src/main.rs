use clap::Parser;
use inquire::{Confirm, Text, CustomType};
use tera::{Tera, Context};
use std::fs;

#[derive(Parser)]
#[command(name = "scaffold-egui-app")]
struct Cli {
    #[arg(long)]
    interactive: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    let (app_title, use_env_logger, use_tokio, width, height, min_width, min_height, resizable) =
        if cli.interactive {
            (
                Text::new("App title:").with_default("MobiusLoop Example").prompt()?,
                Confirm::new("Enable env_logger?")
                    .with_default(true)
                    .prompt()?,
                Confirm::new("Use Tokio runtime?")
                    .with_default(true)
                    .prompt()?,
                CustomType::new("Width:")
                    .with_default(900.0)
                    .prompt()?,
                CustomType::new("Height:")
                    .with_default(800.0)
                    .prompt()?,
                CustomType::new("Min width:")
                    .with_default(600.0)
                    .prompt()?,
                CustomType::new("Min height:")
                    .with_default(400.0)
                    .prompt()?,
                Confirm::new("Resizable?")
                    .with_default(true)
                    .prompt()?,
            )
        } else {
            panic!("Non-interactive mode not implemented yet.");
        };

    let mut context = Context::new();
    context.insert("app_title", &app_title);
    context.insert("use_env_logger", &use_env_logger);
    context.insert("use_tokio", &use_tokio);
    context.insert("width", &width);
    context.insert("height", &height);
    context.insert("min_width", &min_width);
    context.insert("min_height", &min_height);
    context.insert("resizable", &resizable);

    let tera = Tera::new("templates/**/*")?;
    let rendered = tera.render("main.rs.tera", &context)?;

    fs::write("generated_main.rs", rendered)?;
    println!("✅ Generated main.rs -> ./generated_main.rs");

    Ok(())
}
