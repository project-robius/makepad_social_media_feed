use makepad_widgets::*;

live_design! {
    LOGO_TEXT = {
        font_size: (16),
        font: {path: dep("crate://self/resources/IBMPlexSans-Text.ttf")}
    }

    PRIMARY_TEXT = {
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
    ICON_OPTS = dep("crate://self/resources/icons/Icon_Opts.svg")
    ICON_DM = dep("crate://self/resources/icons/Icon_Dm.svg")

    IMG_PROFILE = dep("crate://self/resources/pfp2.jpg")
    IMG_PROFILE_2 = dep("crate://self/resources/pfp.jpg")
    IMG_POST = dep("crate://self/resources/post2.jpg")

    COLOR_BG = #FDFDFD
    COLOR_SUEGGESTION_BG = #F4F4F4
    COLOR_SUEGGESTION_BOX_BG = (COLOR_BG)
    COLOR_FOLLOW_BTN_BG = #2980b9
    COLOR_FOLLOW_BTN_TEXT = #FFFFFF
}
