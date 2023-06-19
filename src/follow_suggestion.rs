use makepad_widgets::*;

live_design! {
    import makepad_widgets::frame::*;
    import makepad_widgets::button::Button;
    import makepad_widgets::label::Label;
    import makepad_draw::shader::std::*;

    import makepad_social_media_feed::style_constants::*;

    FollowSuggestion = <Frame> {
        walk: {width: 220, height: Fit},
        layout: {flow: Down, padding: {top: 10., bottom: 10.}, spacing: 12., align: {x: 0.5, y: 0}}
        show_bg: true
        draw_bg: {
            fn pixel(self) -> vec4 {
                return (COLOR_SUEGGESTION_BOX_BG)
            }
        }

        profile_img = <Image> {
            image: (IMG_PROFILE_2)
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
            image_scale: 1.0
            walk: {margin: 0}
            layout: {padding: 0}
        }

        username = <Label> {
            walk: {width: Fit, height: Fit},
            draw_label: {
                text_style: <BOLD_TEXT> {},
                color: #0
            },
            label: "Jane Smith",
        }

        follow_button = <Button> {
            icon_walk:{margin:{left:10}, width:16,height:Fit}
            label: "Follow"
            draw_label: {
                fn get_color(self) -> vec4 {
                    return (COLOR_FOLLOW_BTN_TEXT)
                }
            }
            draw_bg: {
                fn pixel(self) -> vec4 {
                    return (COLOR_FOLLOW_BTN_BG)
                }
            }
         }
    }
}
