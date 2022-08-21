use headless_chrome::protocol::cdp::Page;
use headless_chrome::Browser;
use std::error::Error;
use std::fs;
// use std::path::Path;
// extern crate image;

fn browse_wikipedia() -> Result<(), Box<dyn Error>> {
    let browser = Browser::default()?;

    let tab = browser.wait_for_initial_tab()?;

    /// Navigate to wikipedia
    tab.navigate_to("https://www.wikipedia.org")?;

    /// Wait for network/javascript/dom to make the search-box available
    /// and click it.
    tab.wait_for_element("input#searchInput")?.click()?;

    /// Type in a query and press `Enter`
    tab.type_str("WebKit")?.press_key("Enter")?;

    /// We should end up on the WebKit-page once navigated
    let elem = tab.wait_for_element("#firstHeading")?;
    assert!(tab.get_url().ends_with("WebKit"));

    /// Take a screenshot of the entire browser window
    let jpeg_data =
        tab.capture_screenshot(Page::CaptureScreenshotFormatOption::Jpeg, None, None, true)?;
    // image::save_buffer(&Path::new("image.png"), jpeg_data, 800, 600, image::ColorType::Rgba8);

    fs::write("foo.jpg", jpeg_data).expect("Unable to write file");

    /// Take a screenshot of just the WebKit-Infobox
    let _png_data = tab
        .wait_for_element("#mw-content-text > div > table.infobox.vevent")?
        .capture_screenshot(Page::CaptureScreenshotFormatOption::Png)?;

    // Run JavaScript in the page
    let remote_object = elem.call_js_fn(
        r#"
        function getIdTwice () {
            // `this` is always the element that you called `call_js_fn` on
            console.log("getIdTwice");
            const id = this.id;
            return id + id;
        }
    "#,
        vec![],
        false,
    )?;
    match remote_object.value {
        Some(returned_string) => {
            dbg!(&returned_string);
            // println!("{}",returned_string)
            // assert_eq!(returned_string, "firstHeadingfirstHeading".to_string());
        }
        _ => unreachable!(),
    };

    Ok(())
}

fn main() {
    browse_wikipedia();

    return;
}
