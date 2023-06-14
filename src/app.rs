use makepad_micro_serde::*;
use makepad_platform::network::{HttpRequest, Method};
use makepad_widgets::*;

live_design! {
    import makepad_widgets::button::Button;
    import makepad_widgets::desktop_window::DesktopWindow;
    import makepad_widgets::label::Label;
    import makepad_widgets::frame::*
    import makepad_draw::shader::std::*;
    import makepad_widgets::swipe_list::*;

    LOGO_TEXT = {
        font_size: (16),
        font: {path: dep("crate://self/resources/IBMPlexSans-Text.ttf")}
    }

    REGULAR_TEXT = {
        font_size: (12),
        font: {path: dep("crate://self/resources/IBMPlexSans-Text.ttf")}
    }

    BOLD_TEXT = {
        font_size: (12),
        font: {path: dep("crate://self/resources/IBMPlexSans-SemiBold.ttf")}
    }

    ICON_HEART = dep("crate://self/resources/icons/Icon_Heart.svg")
    ICON_COMMENT = dep("crate://self/resources/icons/Icon_Comment.svg")
    ICON_SHARE = dep("crate://self/resources/icons/Icon_Share.svg")
    ICON_SAVE = dep("crate://self/resources/icons/Icon_Save.svg")
    ICON_FAV = dep("crate://self/resources/icons/Icon_Favorite.svg")

    IMG_PROFILE = dep("crate://self/resources/pfp2.jpg")
    IMG_POST = dep("crate://self/resources/post2.jpg")

    COLOR_BG = #xfff8ee

    IconButton = <Button> {
        draw_icon: {
            svg_file: (ICON_HEART)
            fn get_color(self) -> vec4 {
                return vec4(0.0, 0.0, 0.0, 1.0);
            }
        }
        icon_walk: {margin: {left: 0.0}, width: 14, height: Fit}
        label: ""
        draw_bg: {
            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.box(
                    1.,
                    1.,
                    self.rect_size.x - 2.0,
                    self.rect_size.y - 2.0,
                    2.0
                )

                sdf.fill((COLOR_BG));

                return sdf.result
            }
        }
    }
    
    PostListEntry = <SwipeListEntry> {
        layout: {flow: Down, padding: 0.0}
        walk: {width: Fill, height: Fit, margin: {bottom: 60}}
        
        center: <Frame> {
            layout: {
                flow: Down,
                spacing: 10,
                padding: 0.0
            },
            walk: {width: Fill, height: Fit},
    
            header = <Frame> {
                layout: {
                    flow: Right,
                    spacing: 10,
                },
                walk: {width: Fill, height: Fit},
    
                profile_img = <Image> {
                    image: (IMG_PROFILE)
                    draw_bg:{
                        fn pixel(self) -> vec4 {
                            let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                            let c = self.rect_size * 0.5;
                            sdf.circle(c.x,c.y,c.x - 2.)
                            sdf.fill_keep(self.get_color());
                            sdf.stroke(#FFF8EE, 1);
                            return sdf.result
                        }
                    }
                    image_scale: 0.65
                    walk: {margin: 0}
                    layout: {padding: 0}
                }
    
                username = <Label> {
                    walk: {width: Fit, height: Fit},
                    draw_label: {
                        text_style: <BOLD_TEXT> {},
                        color: #0
                    },
                    label: "username",
                }
            }

            content = <Image> {
                image: (IMG_POST),
                image_scale: 0.2,
                walk: {margin: 0}
                layout: {padding: 0}
            }
    
            actions = <Frame> {
                layout: {
                    flow: Right,
                    spacing: 0.0,
                    padding: 0.0,
                },
                walk: {width: Fill, height: Fit},
    
                like_button = <IconButton> { draw_icon: { svg_file: (ICON_HEART) } icon_walk: { width: 20.0, height: 20.0 } }
                comment_button = <IconButton> { draw_icon: { svg_file: (ICON_COMMENT) } icon_walk: { width: 20.0, height: 20.0 } }
                share_button = <IconButton> { draw_icon: { svg_file: (ICON_SHARE) } icon_walk: { width: 20.0, height: 20.0 } }
                save_button = <IconButton> { draw_icon: { svg_file: (ICON_SAVE) } icon_walk: { width: 20.0, height: 20.0 } }
            }
    
            caption = <Frame> {
                layout: {
                    flow: Right,
                    spacing: 3,
                },
                walk: {width: Fill, height: Fit},
    
                username = <Label> {
                    walk: {width: Fit, height: Fit},
                    draw_label: {
                        text_style: <BOLD_TEXT> {},
                        color: #0
                    },
                    label: "username",
                }
    
                caption_text = <Label> {
                    walk: {width: Fill, height: Fit},
                    draw_label: {
                        text_style: <REGULAR_TEXT> {},
                        color: #0
                    },
                    label: "caption text",
                }
            }
        }
    }

    PostList = <SwipeList> {
        walk: { height: Fill, margin: 2}
        Entry = <PostListEntry>{ }
    }

    App = {{App}} {
        ui: <DesktopWindow>{
            window: {inner_size: vec2(720, 1080)},
            show_bg: true
            layout: {
                flow: Down,
                spacing: 0.0,
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
                    return (COLOR_BG)
                }
            }

            header = <Frame> {
                layout: {
                    flow: Right,
                    spacing: 240,
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

                message_label = <Label> {
                    walk: {width: Fit, height: Fit},
                    draw_label: {
                        text_style: <LOGO_TEXT>{},
                        color: #0
                    },
                    label: "Makegram",
                }

                notifications_button = <IconButton> { draw_icon: { svg_file: (ICON_FAV) } icon_walk: { width: 20.0, height: 20.0 } }
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
    }

    fn after_new_from_doc(&mut self, _cx: &mut Cx) {
        // Self::fetch_posts(cx);

        let mut posts = Vec::new();
            
        posts.push(Post {
            id: 908234823904,
            user: "matthewwastaken".to_string(),
            user_pfp_url: "some_pfp_url".to_string(),
            content_url: "some_content_url".to_string(),
            caption: "This is a a test caption, it looks pretty neat with word-wrapping. Looking forward to links and emojis".to_string(),
        });

        posts.push(Post {
            id: 908234823902,
            user: "i_am_susan".to_string(),
            user_pfp_url: "some_pfp_url".to_string(),
            content_url: "some_content_url".to_string(),
            caption: "lorem ipusm dolor set amet sit amet, lorem ipusm dolor set amet sit amet".to_string(),
        });

        posts.push(Post {
            id: 908234823902,
            user: "makegram".to_string(),
            user_pfp_url: "some_pfp_url".to_string(),
            content_url: "some_content_url".to_string(),
            caption: "lorem ipusm dolor set amet sit amet, lorem ipusm dolor set amet sit amet".to_string(),
        });

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

        let post_list = self.ui.get_swipe_list_set(ids!(post_list));

        if let Event::Draw(event) = event {
            // self.ui.draw_widget_all(&mut Cx2d::new(cx, event));

            let cx = &mut Cx2d::new(cx, event);
            while let Some(next) = self.ui.draw_widget(cx).hook_widget() {
                if let Some(mut list) = post_list.has_widget(&next).borrow_mut() {
                    for post in self.posts.iter() {
                        if let Some(item) = list.get_entry(cx, LiveId(post.id as u64).into(), live_id!(Entry)) {
                            item.get_label(id!(username)).set_label(&post.user);
                            item.get_label(id!(caption_text)).set_label(&post.caption);
                            
                            // TODO 
                            // item.get_image(id!(profile_img)).set_url(&post.user_pfp_url); 
                            // item.get_image(id!(content)).set_url(&post.content_url); 
                            item.draw_widget_all(cx);
                        }
                    }
                }
            }
            return
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