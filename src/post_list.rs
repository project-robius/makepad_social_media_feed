use crate::post_widget::*;
use makepad_widgets::*;

live_design! {
    import makepad_social_media_feed::post_widget::PostWidget;

    PostList = {{PostList}} {
        layout: {
            flow: Down,
            spacing: 10,
        },
        walk: {width: Fill, height: Fit},
        post: <PostWidget> {}
    }
}

#[derive(Clone, Debug, Default, Eq, Hash, Copy, PartialEq, FromLiveId)]
pub struct PostId(pub LiveId);

#[derive(Live)]
pub struct PostList {
    #[live]
    walk: Walk,
    #[live]
    layout: Layout,

    #[live]
    post: Option<LivePtr>,

    #[rust]
    posts: Vec<Post>,
    #[rust]
    items: ComponentMap<PostId, PostWidgetRef>,
}

impl LiveHook for PostList {
    fn before_live_design(cx: &mut Cx) {
        register_widget!(cx, PostList);
    }
}

impl Widget for PostList {
    fn handle_widget_event_with(
        &mut self,
        cx: &mut Cx,
        event: &Event,
        dispatch_action: &mut dyn FnMut(&mut Cx, WidgetActionItem),
    ) {
        self.handle_event_with(cx, event, &mut |cx, action| {
            dispatch_action(cx, action);
        });
    }

    fn get_walk(&self) -> Walk {
        self.walk
    }

    fn redraw(&mut self, _cx: &mut Cx) {
        // todo
    }

    fn draw_walk_widget(&mut self, cx: &mut Cx2d, walk: Walk) -> WidgetDraw {
        let _ = self.draw_walk(cx, walk);
        WidgetDraw::done()
    }
}

impl PostList {
    pub fn handle_event_with(
        &mut self,
        cx: &mut Cx,
        event: &Event,
        dispatch_action: &mut dyn FnMut(&mut Cx, WidgetActionItem),
    ) {
        for (id, item) in self.items.iter_mut() {
            let item_uid = WidgetUid(id.0 .0);

            if let Some(mut inner) = item.borrow_mut() {
                inner.handle_event_with(cx, event, &mut |cx_imm, action| {
                    dispatch_action(cx_imm, WidgetActionItem::new(action.into(), item_uid));
                });
            }
        }
    }

    pub fn draw_walk(&mut self, cx: &mut Cx2d, walk: Walk) {
        cx.begin_turtle(walk, self.layout);

        for (_id, post) in self.posts.iter().enumerate() {
            let widget_id = LiveId(post.id).into();
            let current_post = self.items.get_or_insert(cx, widget_id, |cx| {
                PostWidgetRef::new_from_ptr(cx, self.post)
            });

            current_post.set_post(post.clone());

            let _ = current_post.draw_walk_widget(cx, walk);
        }

        cx.end_turtle();
        self.items.retain_visible();
    }

    pub fn set_posts(&mut self, posts: Vec<Post>) {
        self.posts = posts
    }
}
