use makepad_widgets::*;

live_design! {
    import makepad_widgets::button::Button;
    import makepad_draw::shader::std::*;

    import makepad_social_media_feed::style_constants::*;

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
}
