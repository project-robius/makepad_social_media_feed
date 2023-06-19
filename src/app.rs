use makepad_micro_serde::*;
use makepad_platform::network::{HttpRequest, Method};
use makepad_widgets::*;

live_design! {
    import makepad_widgets::button::Button;
    import makepad_widgets::desktop_window::DesktopWindow;
    import makepad_widgets::label::Label;
    import makepad_widgets::frame::*;
    import makepad_draw::shader::std::*;
    import makepad_widgets::swipe_list::*;
    import makepad_widgets::slides_view::Slide;
    import makepad_widgets::slides_view::SlideChapter;
    import makepad_widgets::slides_view::SlideBody;
    import makepad_widgets::slides_view::SlidesView;

    import makepad_social_media_feed::post_list_entry::*;
    import makepad_social_media_feed::icon_button::*;
    import makepad_social_media_feed::helper_components::*;
    import makepad_social_media_feed::follow_suggestion::*;
    import makepad_social_media_feed::style_constants::*;

    PostList = <SwipeList> {
        walk: { height: Fill, margin: 2}
        Entry = <PostListEntry>{ }
    }

    App = {{App}} {
        ui: <DesktopWindow>{
            window: {inner_size: vec2(540, 1320)},
            show_bg: true
            block_signal_event: true
            layout: {
                flow: Down,
                spacing: 0.0,
                align: {
                    x: 0.0,
                    y: 0.0
                },
                padding: 0.0
            },
            walk: {
                width: Fill,
                height: Fill
            },
            draw_bg: {
                fn pixel(self) -> vec4 {
                    return (COLOR_BG)
                }
            }

            header = <Frame> {
                layout: {
                    flow: Right,
                    align: {
                        x: 0.2,
                        y: 0.1
                    }
                },
                walk: {
                    width: Fill,
                    height: Fit,
                    margin: {
                        top: 20,
                        bottom: 40,
                        left: 5,
                        right: 5
                    }
                }

                <Label> {
                    walk: {width: Fit, height: Fit},
                    draw_label: {
                        text_style: <LOGO_TEXT>{},
                        color: #0
                    },
                    label: "Makegram",
                }
                <FillerX> {}
                dms_button = <IconButton> { draw_icon: { svg_file: (ICON_DM) } icon_walk: { width: 20.0, height: 20.0 } }
            }

            <ScrollX> {
                walk: {width: 540, height: Fit, margin: {bottom: 20}},
                layout: {spacing: 15., padding: 15.},
                show_bg: true,
                draw_bg: {
                    fn pixel(self) -> vec4 {
                        return (COLOR_SUEGGESTION_BG)
                    }
                }
                <FollowSuggestion> {}
                <FollowSuggestion> {}
                <FollowSuggestion> {}
                <FollowSuggestion> {}
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
        crate::style_constants::live_design(cx);
        crate::helper_components::live_design(cx);
        crate::icon_button::live_design(cx);
        crate::post_list_entry::live_design(cx);
        crate::follow_suggestion::live_design(cx);
    }

    fn after_new_from_doc(&mut self, _cx: &mut Cx) {
        // Self::fetch_posts(cx);

        let mut posts = Vec::new();

        for i in 1..10 {
            posts.push(Post {
                id: i,
                user: "matthewwastaken".to_string(),
                user_pfp_url: "some_pfp_url".to_string(),
                content_url: "some_content_url".to_string(),
                caption: "This is a a test caption, it looks pretty neat with word-wrapping. Looking forward to links and emojis".to_string(),
            });
        }

        self.posts = posts;
    }
}

impl App {
    fn _fetch_posts(cx: &mut Cx) {
        let server_url = "http://localhost:3000/posts".to_string();
        let request_identifier = LiveId::from_str("PostsFetch").unwrap();
        let mut request = HttpRequest::new(request_identifier, server_url, Method::GET);
        request.set_header("Content-Type".to_string(), "application/json".to_string());
        cx.http_request(request);
    }
}

impl AppMain for App {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        // makepad_error_log::log!("Event: {:?}", event);

        let post_list = self.ui.get_swipe_list_set(ids!(post_list));

        if let Event::Draw(event) = event {
            let cx = &mut Cx2d::new(cx, event);

            while let Some(next) = self.ui.draw_widget(cx).hook_widget() {
                if let Some(mut list) = post_list.has_widget(&next).borrow_mut() {
                    for post in self.posts.iter() {
                        if let Some(item) =
                            list.get_entry(cx, LiveId(post.id as u64).into(), live_id!(Entry))
                        {
                            item.get_label(id!(username)).set_label(&post.user);
                            item.get_label(id!(caption_text)).set_label(&post.caption);

                            // TODO
                            // item.get_frame(id!(content)).set_image(cx, &post.content_url);
                            // item.get_frame(id!(profile_img)).set_image(cx, &post.user_pfp_url);
                            item.draw_widget_all(cx);
                        }
                    }
                }
            }
            return;
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

        // if let Some(mut post_list) = self.ui.get_widget(id!(post_list)).borrow_mut::<PostList>() {
        //     post_list.set_posts(self.posts.clone());
        //     post_list.set_posts(posts);
        // }
        self.ui.handle_widget_event(cx, event);
    }
}

#[derive(DeJson, SerJson)]
struct PostListFetch(Vec<Post>);

#[derive(Debug, Clone, PartialEq, Default, DeJson, SerJson)]
pub struct Post {
    pub id: u64,
    pub user: String,
    pub user_pfp_url: String,
    pub content_url: String,
    pub caption: String,
}
