use makepad_widgets::*;

live_design! {
    import makepad_widgets::button::Button;
    import makepad_widgets::label::Label;
    import makepad_widgets::frame::*
    import makepad_draw::shader::std::*;
    import makepad_widgets::swipe_list::*;

    import makepad_social_media_feed::helper_components::*;
    import makepad_social_media_feed::icon_button::*;
    import makepad_social_media_feed::style_constants::*;

    PostListEntry = <SwipeListEntry> {
        layout: {flow: Down, padding: 0.0}
        walk: {width: Fill, height: Fit, margin: {bottom: 50}}

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
                    align: {x: 0.5, y: 0.5}
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
                <FillerX> {}
                options = <IconButton> { draw_icon: { svg_file: (ICON_OPTS) } icon_walk: { width: 20.0, height: 20.0 } }
            }

            content = <Frame> {
                layout: {
                    flow: Down,
                    spacing: 10,
                    padding: 0.0
                },
                walk: {width: Fill, height: Fit},

                <Image> {
                    image: (IMG_POST),
                    image_scale: 0.2,
                    walk: {margin: 0}
                    layout: {padding: 0}
                }
            }

            actions = <Frame> {
                layout: {
                    flow: Right,
                },
                walk: {width: Fill, height: Fit},

                like_button = <IconButton> { draw_icon: { svg_file: (ICON_HEART) } icon_walk: { width: 20.0, height: 20.0 } }
                comment_button = <IconButton> { draw_icon: { svg_file: (ICON_COMMENT) } icon_walk: { width: 20.0, height: 20.0 } }
                share_button = <IconButton> { draw_icon: { svg_file: (ICON_SHARE) } icon_walk: { width: 20.0, height: 20.0 } }
                <FillerX> {}
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
                        text_style: <PRIMARY_TEXT> {},
                        color: #0
                    },
                    label: "caption text",
                }
            }

            view_comments_btn = <Button> {
                icon_walk: {width: Fit, height: Fit}
                walk: {margin: 0.}
                layout: {padding: 0.}
                label: "View all 26 comments"

                draw_label: {
                    text_style: <PRIMARY_TEXT> {},
                    color: #F2F2F2
                },

                draw_bg: {
                    instance hover: 0.0
                    instance pressed: 0.0

                    fn pixel(self) -> vec4 {
                        let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                        return sdf.result
                    }
                }
            }

            posted_time = <Label> {
                draw_label: {
                    text_style: <PRIMARY_TEXT> {},
                    color: #D0D0D0
                }
                label: "5 hours ago"
            }
        }
    }
}
