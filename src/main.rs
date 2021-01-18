use druid::widget::{Button, Flex, Label};
use druid::{AppLauncher, LocalizedString, PlatformError, Widget, WidgetExt, WindowDesc, Data};

#[derive(Clone, Data)]
struct Counter(i32);

fn main() -> Result<(), PlatformError> {
    // Window builder. We set title and size
    let main_window = WindowDesc::new(ui_builder)
        .title("Hello, Druid!")
        .window_size((200.0, 100.0));
    
    // Data to be used in the app (=state)
    let data: Counter = Counter(0);

    // Set Locale
    //let env = Env::default();
    //L10nManager::new(["default"], "resources/i18n/");
    
    // Run the app
    AppLauncher::with_window(main_window)
        .use_simple_logger() // Neat!
        .launch(data)
}

fn ui_builder() -> impl Widget<Counter> {
    // The label text will be computed dynamically based on the current locale and count
    let text = LocalizedString::new("counter-app")
        .with_arg("count", |data: &Counter, _env| (*data).0.into());
    let label = Label::new(text).padding(5.0).center();

    // Two buttons with on_click callback
    let button_plus = Button::new("+1")
        .on_click(|_ctx, data: &mut Counter, _env| (*data).0 += 1)
        .padding(5.0);
    let button_minus = Button::new("-1")
        .on_click(|_ctx, data: &mut Counter, _env| (*data).0 -= 1)
        .padding(5.0);

    // Container for the two buttons
    let flex = Flex::row()
        .with_child(button_plus)
        .with_spacer(1.0)
        .with_child(button_minus);

    // Container for the whole UI
    Flex::column()
        .with_child(label)
        .with_child(flex)
}