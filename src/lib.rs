extern crate web_view;

// use deno_core::op_sync;
// use deno_core::JsRuntime;
use web_view::*;

unsafe extern "C" fn sogouda_launch_app(
    title: *mut u8,
    title_length: usize,
    url: *mut u8,
    url_length: usize,
    width: i32,
    height: i32,
    is_frameless: bool,
    debug: bool
) {
    let title_string =
        String::from_raw_parts(
            title,
            title_length,
            title_length
        );

    let url_string =
        String::from_raw_parts(
            url,
            url_length,
            url_length
        );

    web_view::builder()
        .title(title_string.as_str())
        .content(Content::Url(url_string.as_str()))
        .size(width, height)
        .frameless(is_frameless)
        .debug(debug)
        .user_data("")
        .invoke_handler(|webview, arg| {
            match arg {
                "exit" => webview.exit(),
                _ => (),
            }
            Ok(())
        })
        .run()
    .unwrap();
}

pub fn launch_app(title: &str, url: &str, width: Option<i32>, height: Option<i32>, is_frameless: Option<bool>, debug: Option<bool>) {
    unsafe {
        sogouda_launch_app(
            title.to_string().as_mut_ptr(),
            title.len(),
            url.to_string().as_mut_ptr(),
            url.len(),
            width.unwrap_or(640),
            height.unwrap_or(480),
            is_frameless.unwrap_or(false),
            debug.unwrap_or(false)
        );
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        use crate::launch_app;

        const URL: &str = "https://ipfs.io/ipfs/QmQPeNsJPyVWPFDVHb77w8G42Fvo15z4bG2X8D2GhfbSXc/readme";

        launch_app(
            "Gouda App",
            URL,
            None,
            None,
            None,
            None
        );

        assert_eq!(
            true,
            true
        );
    }
}
