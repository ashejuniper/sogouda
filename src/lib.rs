extern crate web_view;

// use deno_core::op_sync;
// use deno_core::JsRuntime;
use web_view::*;

pub fn launch_app(title: &str, url: &str, width: Option<i32>, height: Option<i32>, is_frameless: Option<bool>, debug: Option<bool>) -> bool {
    web_view::builder()
        .title(title)
        .content(Content::Url(url))
        .size(width.unwrap_or(640), height.unwrap_or(480))
        .frameless(is_frameless.unwrap_or(false))
        .debug(debug.unwrap_or(false))
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

    return false;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        use crate::launch_app;

        const URL: &str = "https://ipfs.io/ipfs/QmQPeNsJPyVWPFDVHb77w8G42Fvo15z4bG2X8D2GhfbSXc/readme";

        let result =
            launch_app(
                "Gouda App",
                URL,
                None,
                None,
                None,
                None
            );

        assert_eq!(result, false);
    }
}
