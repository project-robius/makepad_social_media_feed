use makepad_micro_serde::*;
use makepad_platform::network::{HttpRequest, Method};
use makepad_widgets::*;

use crate::{post_list::PostList, post_widget::Post};

live_design! {
    import makepad_widgets::button::Button;
    import makepad_widgets::desktop_window::DesktopWindow;
    import makepad_widgets::label::Label;
    import makepad_widgets::frame::Image;
    import makepad_widgets::text_input::TextInput;
    import makepad_widgets::frame::*
    import makepad_draw::shader::std::*;
    import makepad_social_media_feed::post_list::PostList;

    LOGO_TEXT = {
        font_size: (16),
        font: {path: dep("crate://self/resources/IBMPlexSans-Text.ttf")}
    }

    ICO_SAVE = dep("crate://self/resources/icons/Icon_Save.svg")
    ICO_HEART = dep("crate://self/resources/icons/Icon_Heart.svg")

    COLOR_BLACK = #x00000000

    IconButton = <Button> {
        draw_icon: {
            svg_file: (ICO_SAVE),
            fn get_color(self) -> vec4 {
                return mix(
                    mix(
                        (COLOR_BLACK),
                        (COLOR_BLACK),
                        self.hover
                    ),
                    (COLOR_BLACK),
                    self.pressed
                )
            }
         }
        icon_walk: { width: 7.5, height: Fit }
        draw_bg: {
            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                return sdf.result
            }
        }
        layout: { padding: 0.0 }
        label: "icon"
    }

    App = {{App}} {
        ui: <DesktopWindow>{

            show_bg: true
            layout: {
                flow: Down,
                spacing: 20,
                align: {
                    x: 0.0,
                    y: 0.0
                }
            },
            walk: {
                width: Fill,
                height: Fill
            },
            draw_bg: {
                fn pixel(self) -> vec4 {
                    return vec4(0.95, 0.95, 0.95, 1.0);
                }
            }

            header = <Frame> {
                layout: {
                    flow: Right,
                    spacing: 20,
                    align: {
                        x: 0.2,
                        y: 0.1
                    }
                },
                walk: {
                    width: Fill,
                    height: 170
                }

                message_label = <Label> {
                    walk: {width: 300, height: Fit},
                    draw_label: {
                        text_style: <LOGO_TEXT>{},
                        color: #0
                    },
                    label: "Makepadgram",
                }

                notifications_button = <IconButton> { draw_icon: { svg_file: (ICO_HEART) } icon_walk: { width: 15.0, height: Fit } }
            }

            post_list = <PostList> {}
        }
    }
}

app_main!(App);

#[derive(Live)]

pub struct App {
    #[live]
    ui: WidgetRef,
    #[rust]
    posts: Vec<Post>,
}

impl LiveHook for App {
    fn before_live_design(cx: &mut Cx) {
        crate::makepad_widgets::live_design(cx);
        crate::post_list::live_design(cx);
        crate::post_widget::live_design(cx);
    }

    fn after_new_from_doc(&mut self, cx: &mut Cx) {
        Self::fetch_posts(cx);
    }
}

impl App {
    fn fetch_posts(cx: &mut Cx) {
        let server_url = "http://localhost:3000/posts".to_string();
        let request_identifier = LiveId::from_str("PostsFetch").unwrap();
        let mut request = HttpRequest::new(request_identifier, server_url, Method::GET);
        request.set_header("Content-Type".to_string(), "application/json".to_string());
        cx.http_request(request);
    }
}

impl AppMain for App {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        if let Event::Draw(event) = event {
            return self.ui.draw_widget_all(&mut Cx2d::new(cx, event));
        }

        if let Event::HttpResponse(event) = event {
            event.response.id.as_string(|id: Option<&str>| match id {
                Some("PostsFetch") => {
                    if event.response.status_code == 200 {
                        let posts = event.response.get_json_body_as::<PostListFetch>().unwrap();

                        self.posts = posts.0;
                        self.ui.redraw(cx);
                    }
                }
                _ => (),
            });
        }

        if let Some(mut post_list) = self.ui.get_widget(id!(post_list)).borrow_mut::<PostList>() {
            post_list.set_posts(self.posts.clone());

            // to run with local data instead (no network):
            // let mut posts = Vec::new();
            // posts.push(Post {
            //     id: 908234823904,
            //     user: "matthew".to_string(),
            //     content_url: "some_image_url".to_string(),
            //     caption: "lorem ipusm dolor set amet sit amet".to_string(),
            // });
            // post_list.set_posts(posts);
        }
    }
}

#[derive(DeJson, SerJson)]
struct PostListFetch(Vec<Post>);
