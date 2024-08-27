use eframe::egui::{Label, RichText, Widget, vec2, Color32, CursorIcon, Layout, Direction, Sense, Ui, Response};


fn get_color(dark_mode: bool, colors: (Color32, Color32)) -> Color32 {
    if dark_mode {
        colors.0
    } else {
        colors.1
    }
}

#[must_use = "You should put this widget in an ui with `ui.add(widget);`"]

pub struct TabBar<'a> {
    selected: &'a mut usize,
    cols: Vec<String>,
    height: f32,
    sense: Sense,
    layout: Layout,
    selected_bg: (Color32, Color32),
    selected_fg: (Color32, Color32),
    hover_bg: (Color32, Color32),
    hover_fg: (Color32, Color32),
    bg: (Color32, Color32),
    fg: (Color32, Color32),
    stroke_bg: (Color32, Color32),
    underline: bool,
    heading: bool,
}
impl<'a> TabBar<'a> {
    pub fn new(cols: Vec<String>, selected: &'a mut usize, visuals: &eframe::egui::Visuals) -> Self {
        TabBar {
            cols,
            selected,
            height: 30.0,
            sense: Sense::click(),
            layout: Layout::centered_and_justified(Direction::LeftToRight),
            selected_bg: (visuals.selection.bg_fill, visuals.selection.bg_fill),
            selected_fg: (visuals.selection.stroke.color, visuals.selection.stroke.color),
            hover_bg: (visuals.widgets.hovered.bg_fill, visuals.widgets.hovered.bg_fill),
            hover_fg: (visuals.widgets.hovered.fg_stroke.color, visuals.widgets.hovered.fg_stroke.color),
            bg: (visuals.code_bg_color, visuals.code_bg_color),
            fg: (visuals.widgets.active.fg_stroke.color, visuals.widgets.active.fg_stroke.color),
            stroke_bg: (Color32::from_rgb(170, 170, 170), Color32::from_rgb(170, 170, 170)),
            underline: false,
            heading: false
        }
    }

    pub fn bg(mut self, bg: Color32, bgd: Color32) -> Self {
        self.bg = (bg, bgd);
        self
    }

    pub fn fg(mut self, fg: Color32, fgd: Color32) -> Self {
        self.fg = (fg, fgd);
        self
    }

    pub fn stroke_bg(mut self, stroke_bg: Color32, stroke_bgd: Color32) -> Self {
        self.stroke_bg = (stroke_bg, stroke_bgd);
        self
    }

    pub fn hover_bg(mut self, bg_fill: Color32, bg_filld: Color32) -> Self {
        self.hover_bg = (bg_fill, bg_filld);
        self
    }

    pub fn hover_fg(mut self, hover_fg: Color32, hover_fgd: Color32) -> Self {
        self.hover_fg = (hover_fg, hover_fgd);
        self
    }

    pub fn selected_fg(mut self, selected_fg: Color32, selected_fgd: Color32) -> Self {
        self.selected_fg = (selected_fg, selected_fgd);
        self
    }

    pub fn selected_bg(mut self, bg_fill: Color32, bg_filld: Color32) -> Self {
        self.selected_bg = (bg_fill, bg_filld);
        self
    }

    pub fn sense(mut self, sense: Sense) -> Self {
        self.sense = sense;
        self
    }

    /// The layout of the content in the cells
    pub fn layout(mut self, layout: Layout) -> Self {
        self.layout = layout;
        self
    }

    pub fn height(mut self, height: f32) -> Self {
        self.height = height;
        self
    }

    /// Underline the text of the selected tab
    pub fn underline(mut self, underline: bool) -> Self {
        self.underline = underline;
        self
    }

    /// Use a bigger Size for the text of the selected tab
    pub fn heading(mut self, heading: bool) -> Self {
        self.heading = heading;
        self
    }
}


impl<'a> Widget for TabBar<'a>
{
    fn ui(self, ui: &mut Ui) -> Response {
        let dark_mode = ui.visuals().dark_mode;
        let cols = self.cols;
        let sense = self.sense;
        let layout = self.layout;
        let selected = self.selected;
        let height = self.height;
        let selected_bg = self.selected_bg;
        let selected_fg = self.selected_fg;
        let hover_bg = self.hover_bg;
        let hover_fg = self.hover_fg;
        let bg = self.bg;
        let fg = self.fg;
        let stroke_bg = self.stroke_bg;
        let underline = self.underline;
        let heading = self.heading;

        // Paint a stroke around the tab-group
        let mut rect = ui.available_rect_before_wrap();
        rect.set_height(height);
        let mut response = ui.allocate_rect(rect, sense);

        ui.painter().rect_filled(rect, 0.0, get_color(dark_mode, stroke_bg));
        rect.set_height(height - 1.0);
        rect.set_top(rect.top() + 1.0);
        let cell_width = rect.width() / cols.len() as f32;
        rect.set_width(cell_width);

        let mut fg_color: Color32;
        for (ind, header) in cols.iter().enumerate() {
            // Paint the rectangle while preserving the stroke lines
            if ind == 0 { rect.set_left(rect.left() + 1.0) }
            if ind == cols.len() - 1 {rect.set_width(rect.width() - 1.0);}

            let mut child_ui = ui.child_ui(rect, layout, None);

            let resp = ui.allocate_rect(rect, sense);
            let clicked = resp.clicked();
            let hovered = resp.hovered();

            if clicked {
                *selected = ind;
                response = resp.clone();
                response.mark_changed();
            }

            if *selected == ind {
                let mut r = rect.clone();

                // paint stroke and round tab
                r.set_top(r.top() - 3.0);
                r.set_height(height + 1.0);
                ui.painter().rect_stroke(r, 3.0, (1.0, get_color(dark_mode, hover_fg)));
                ui.painter().rect_filled(r, 3.0, get_color(dark_mode, selected_bg));

                // paint lower rect without rounding
                r.set_top(r.top() + 4.0);
                r.set_height(height - 3.0);

                ui.painter().rect_filled(r, 0.0, get_color(dark_mode, selected_bg));
                fg_color = get_color(dark_mode, selected_fg);
            } else if hovered {
                ui.ctx().set_cursor_icon(CursorIcon::PointingHand);
                ui.painter().rect_filled(rect, 0.0, get_color(dark_mode, hover_bg));
                fg_color = get_color(dark_mode, hover_fg);
                response = resp.clone();
            } else {
                ui.painter().rect_filled(rect, 0.0, get_color(dark_mode, bg));
                fg_color = get_color(dark_mode, fg);
            };

            let rt;
            if *selected == ind {
                if underline && heading {
                    rt = RichText::new(header).color(fg_color).underline().heading();
                } else if heading {
                    rt = RichText::new(header).color(fg_color).heading();
                } else if underline {
                    rt = RichText::new(header).color(fg_color).underline();
                } else {
                    rt = RichText::new(header).color(fg_color);
                }
            } else {
                rt = RichText::new(header).color(fg_color);
            }
            child_ui.add(Label::new(rt).selectable(false));

            rect = rect.translate(vec2(cell_width, 0.0));
        }
        response
    }
}

