mod client;
mod server;
mod shared;
mod generated;
mod mutils;

// use ansi_term::{Color, Style};

use bevy::{
    prelude::*,
    // utils::tracing::{field::Field, self},
    window::PresentMode, 
    // log::Level,
};

use client::UIPlugin;

use shared::systems::{
    GamePlugin,
    ResourceCachePlugin,
};

// use std::fmt::Write;
// use std::env;

// use tracing_appender::{
//     non_blocking::{NonBlockingBuilder, WorkerGuard},
//     // rolling::RollingFileAppender,
// };
// use tracing_subscriber::{
//     field::Visit,
//     fmt::{FormatEvent, FormatFields},
//     layer::SubscriberExt,
//     registry::LookupSpan,
//     util::SubscriberInitExt,
//     EnvFilter,
// };

// struct CustomFormatter;
// struct FieldFormatter<'a> {
//     buffer: &'a mut String,
// }
// impl<'a> Visit for FieldFormatter<'a> {
//     fn record_debug(&mut self, field: &Field, value: &dyn std::fmt::Debug) {
//         // write!(self.buffer, "{}={:?}; ", field.name(), value).unwrap();
//         if field.name() == "message" {
//             write!(self.buffer, "{:?} ", value).unwrap();
//         } else {
//             write!(self.buffer, "{}={:?}; ", field.name(), value).unwrap();
//         }
//     }
// }
// impl<S, N> FormatEvent<S, N> for CustomFormatter
// where
//     S: tracing::Subscriber + for<'a> LookupSpan<'a>,
//     N: for<'a> FormatFields<'a> + 'static,
// {
//     fn format_event(
//         &self,
//         _ctx: &tracing_subscriber::fmt::FmtContext<S, N>,
//         mut writer: tracing_subscriber::fmt::format::Writer<'_>,
//         event: &tracing::Event<'_>,
//     ) -> std::fmt::Result {
//         let mut fields_buffer = String::new();
//         let mut field_formatter = FieldFormatter { buffer: &mut fields_buffer };
//         event.record(&mut field_formatter);
//         let level = event.metadata().level();
//         let level_style = match *level {
//             Level::TRACE => Style::new().dimmed(),
//             Level::DEBUG => Style::new(),
//             Level::INFO => Style::new().fg(Color::Green),
//             Level::WARN => Style::new().fg(Color::Yellow),
//             Level::ERROR => Style::new().bold().fg(Color::Red),
//         };
//         let gray_style = Style::new().fg(Color::Fixed(8)); // ANSI color code 8 for gray
//         let level_str = level_style.paint(format!("[{}]", level));
//         let time_str = gray_style.paint(chrono::Utc::now().format("%Y-%m-%dT%H:%M:%S%.6fZ").to_string());
//         let target_str = gray_style.paint( format!("{}:",event.metadata().target()));
//         writeln!(
//             writer,
//             "{} {} {} {}",
//             time_str,
//             level_str,
//             target_str,
//             fields_buffer
//         )
//     }
// }

// // #[cfg(not(target_arch = "wasm32"))]
// fn setup_logger(log_file: &str) -> Option<WorkerGuard> {

//     if cfg!(target_arch = "wasm32") {
//         info!("Running in WebAssembly, using default logger");
//         return None;
//     }

//     // let file_appender = RollingFileAppender::new(tracing_appender::rolling::Rotation::NEVER, ".", log_file);
//     let file_appender = tracing_appender::rolling::never(".", log_file);
//     let (non_blocking, guard) = NonBlockingBuilder::default()
//         .lossy(false)
//         .buffered_lines_limit(1000)
//         .finish(file_appender);
//     let fmt_layer = tracing_subscriber::fmt::layer()
//         .with_target(false)
//         .with_writer(non_blocking)
//         .with_ansi(true)
//         .event_format(CustomFormatter)
//         ;
//     let filter_layer = EnvFilter::try_from_default_env()
//         .unwrap_or_else(|_| EnvFilter::new("info"))
//         .add_directive("wgpu=error".parse().unwrap())
//         .add_directive("bevy_render=info".parse().unwrap())
//         .add_directive("bevy_ecs=info".parse().unwrap());
//     tracing_subscriber::registry()
//         .with(filter_layer)
//         .with(fmt_layer)
//         .init();
//     info!("Logging to file: {}", log_file);
//     Some(guard)
// }


fn main() {
    
    // // println!("Current working directory: {:?}", env::current_dir().unwrap());
    // let mut log_guard: Option<WorkerGuard> = None;
    // if cfg!(not(target_arch = "wasm32")) {
    //     info!("Not Running in WebAssembly, using text file logger.");
    //     log_guard = setup_logger("log.txt");
    // }

    App::new()
        .add_plugins(DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Jastery!".into(),
                    //resolution: (1400., 800.).into(),
                    present_mode: PresentMode::AutoVsync,
                    // Tells wasm to resize the window according to the available canvas
                    fit_canvas_to_parent: true,
                    // Tells wasm not to override default event handling, like F5, Ctrl+R etc.
                    prevent_default_event_handling: true,
                    ..default()
                }),
                ..default()
            })
            // .disable::<bevy::log::LogPlugin>()
        )
        .add_plugin(UIPlugin)
        .add_plugin(ResourceCachePlugin)
        .add_plugin(GamePlugin)        
        .run();

    // if cfg!(not(target_arch = "wasm32")) {
    //     // This will ensure logs are flushed before the application exits
    //     if let Some(guard) = log_guard {
    //         drop(guard);
    //     }
    // }

}


