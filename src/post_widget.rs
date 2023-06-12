use makepad_micro_serde::*;
use makepad_widgets::*;

live_design! {
    import makepad_widgets::frame::Frame;
    import makepad_widgets::frame::Image;
    import makepad_widgets::label::Label;
    import makepad_draw::shader::std::*;

    REGULAR_TEXT = {
        font_size: (12),
        font: {path: dep("crate://self/resources/IBMPlexSans-Text.ttf")}
    }

    BOLD_TEXT = {
        font_size: (12),
        font: {path: dep("crate://self/resources/IBMPlexSans-SemiBold.ttf")}
    }

    PostWidget = <Frame> {
        layout: {
            flow: Down,
            spacing: 10,
        },
        walk: {width: Fill, height: Fit},

        header: <Frame> {
            layout: {
                flow: Right,
                spacing: 5,
            },
            walk: {width: Fill, height: Fit},

            // TODO: Images are not showing
            profile_picture = <Image> {
                image: {path: dep("crate://self/resources/pfp.png")},
                walk: {width: (178 * 0.175), height: (121 * 0.175), margin: { top: 0.0, right: 0.0, bottom: 0.0, left: 10.0 } }
                layout: {padding: 0}
            },

            username = <Label> {
                walk: {width: Fit, height: Fit},
                draw_label: {
                    text_style: <BOLD_TEXT> {},
                    color: #0
                },
                label: "themathew",
            }
        }

        content: <Frame> {
            <Image> {
                image: {path: dep("crate://self/resources/post.png")},
                walk: {width: Fill, height: Fill},
            }
        },

        caption: <Frame> {
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
                label: "themathew",
            }

            caption_text = <Label> {
                walk: {width: Fit, height: Fit},
                draw_label: {
                    text_style: <REGULAR_TEXT> {},
                    color: #0
                },
                label: "lorem ipusm dolor set amet sit amet",
            }
        }
    }
}

#[derive(Live)]
pub struct PostWidget {
    #[live]
    walk: Walk,
    #[live]
    layout: Layout,
    #[live]
    header: Frame,
    // not sure if this is the right way to do this (wrapping things in Frame and calling draw_walk in them)
    // Image is not exposed as a widget because it is part of Frame
    #[live]
    content: Frame,
    #[live]
    caption: Frame,

    #[rust]
    post: Post,
}

#[derive(Clone, PartialEq, WidgetRef, Debug)]
pub struct PostWidgetRef(WidgetRef);

impl PostWidgetRef {
    pub fn set_post(&self, post: Post) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.post = post;
        }
    }
}

impl LiveHook for PostWidget {
    fn before_live_design(cx: &mut Cx) {
        register_widget!(cx, PostWidget);
    }
}

impl Widget for PostWidget {
    fn handle_widget_event_with(
        &mut self,
        _cx: &mut Cx,
        _event: &Event,
        _dispatch_action: &mut dyn FnMut(&mut Cx, WidgetActionItem),
    ) {
        // TODO
        // self.handle_event_with(cx, event, &mut |cx, action| {
        //     dispatch_action(cx, action);
        // });
    }

    fn get_walk(&self) -> Walk {
        self.walk
    }

    fn redraw(&mut self, _cx: &mut Cx) {
        // TODO
    }

    fn draw_walk_widget(&mut self, cx: &mut Cx2d, walk: Walk) -> WidgetDraw {
        let _ = self.draw_walk(cx, walk);
        WidgetDraw::done()
    }
}

impl PostWidget {
    pub fn handle_event_with(
        &mut self,
        _cx: &mut Cx,
        _event: &Event,
        _dispatch_action: &mut dyn FnMut(&mut Cx, PostAction),
    ) {
        // Here we could handle actions like, double tapping to like a post.
    }

    pub fn draw_walk(&mut self, cx: &mut Cx2d, walk: Walk) {
        crate::makepad_error_log::log!("PostWidget::draw_walk, post: {:?}", self.post);

        let _ = self.header.draw_walk(cx, walk);
        let _ = self.content.draw_walk(cx, walk);
        let _ = self.caption.draw_walk(cx, walk);
    }
}

#[derive(Clone, WidgetAction)]
pub enum PostAction {
    None,
}

#[derive(Debug, Clone, PartialEq, Default, DeJson, SerJson)]
pub struct Post {
    pub id: u64,
    pub user: String,
    pub content_url: String,
    pub caption: String,
}
