//! This code is copied almost exactly from the [alert-after](https://github.com/frewsxcv/alert-after) project.
//! Some minor modifications have been made to add icons and such to the messages.

#[cfg(target_os = "linux")]
extern crate notify_rust;

#[cfg(target_os = "macos")]
extern crate mac_notification_sys;

#[cfg(target_os = "windows")]
extern crate winrt;

trait Platform {
    fn setup() -> Self;
    fn notify(title: &str, body: &str);
    fn teardown(&mut self);
}

#[cfg(target_os = "linux")]
struct Linux;
#[cfg(target_os = "linux")]
impl Platform for Linux {
    fn notify(msg_title: &str, msg_body: &str) {
        notify_rust::Notification::new()
            .summary(msg_title)
            .body(msg_body)
            .icon("x-office-calendar")
            .show()
            .unwrap();
    }
    fn setup() -> Self {
        Linux
    }
    fn teardown(&mut self) {}
}

#[cfg(target_os = "macos")]
struct MacOs;
#[cfg(target_os = "macos")]
impl Platform for MacOs {
    fn notify(msg_title: &str, msg_body: &str) {
        let bundle = mac_notification_sys::get_bundle_identifier("Calendar")
            .unwrap_or_else(|| mac_notification_sys::get_bundle_identifier_or_default("Script Editor"));
        mac_notification_sys::set_application(&bundle).unwrap();
        mac_notification_sys::send_notification(msg_title, &None, msg_body, &None).unwrap();
    }
    fn setup() -> Self {
        MacOs
    }
    fn teardown(&mut self) {}
}

#[cfg(target_os = "windows")]
struct Windows(winrt::RuntimeContext);
#[cfg(target_os = "windows")]
impl Platform for Windows {
    fn notify(msg_title: &str, msg_body: &str) {
        use winrt::windows::data::xml::dom::*;
        use winrt::windows::ui::notifications::*;
        use winrt::*;
        unsafe {
            let toast_xml =
                ToastNotificationManager::get_template_content(ToastTemplateType_ToastText02)
                    .unwrap();
            let toast_text_elements = toast_xml
                .get_elements_by_tag_name(&FastHString::new("text"))
                .unwrap();

            toast_text_elements
                .item(0)
                .unwrap()
                .append_child(
                    &*toast_xml
                        .create_text_node(&FastHString::from(msg_title))
                        .unwrap()
                        .query_interface::<IXmlNode>()
                        .unwrap(),
                )
                .unwrap();
            toast_text_elements
                .item(1)
                .unwrap()
                .append_child(
                    &*toast_xml
                        .create_text_node(&FastHString::from(msg_body))
                        .unwrap()
                        .query_interface::<IXmlNode>()
                        .unwrap(),
                )
                .unwrap();

            let toast = ToastNotification::create_toast_notification(&*toast_xml).unwrap();
            ToastNotificationManager::create_toast_notifier_with_id(&FastHString::new(
                "{1AC14E77-02E7-4E5D-B744-2EB1AE5198B7}\\WindowsPowerShell\\v1.0\\powershell.exe",
            ))
            .unwrap()
            .show(&*toast)
            .unwrap();
        }
    }
    fn setup() -> Self {
        Windows(winrt::RuntimeContext::init())
    }
    fn teardown(&mut self) {
        self.0.uninit();
    }
}

#[cfg(target_os = "windows")]
type CurrPlatform = Windows;
#[cfg(target_os = "macos")]
type CurrPlatform = MacOs;
#[cfg(target_os = "linux")]
type CurrPlatform = Linux;

pub fn alert(title: &str, message: &str) {
    let mut platform = CurrPlatform::setup();
    CurrPlatform::notify(title, message);
    platform.teardown();
}
