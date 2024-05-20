#![cfg_attr(target_os = "windows", windows_subsystem = "windows")]

use eframe::egui::{self, FontDefinitions, FontFamily, RichText, Layout, Align, TextStyle};
use rand::seq::SliceRandom;
use rand::thread_rng;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "奖励还是惩罚？",
        options,
        Box::new(|cc| {
            let app = MyApp::default();
            app.setup_fonts(&cc.egui_ctx);
            Box::new(app)
        }),
    )
}

struct MyApp {
    mode: String,
    result: String,
    rewards_pool: Vec<&'static str>,
    punishments_pool: Vec<&'static str>,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            mode: String::new(),
            result: String::new(),
            rewards_pool: vec!["一杯果茶", "换电脑壁纸", "免跑操一次", "免一次惩罚", "一次不搞卫生","笔","棒棒糖","QQ糖"],
            punishments_pool: vec!["自习课下课开合跳X10", "帮六科课代表搬作业各一次", "倒一星期垃圾", "外出上课最后走（关电器）", "讲台上开合跳X20", "大象转圈X20", "当一个星期一体机壁纸", "高歌一曲 or 才艺表演"],
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.with_layout(Layout::top_down(Align::Center), |ui| {
                ui.heading("奖励还是惩罚？");

                ui.add_space(20.0); // 添加一些空间

                ui.horizontal(|ui| {
                    ui.label("请选择模式：");
                });

                ui.add_space(20.0); // 添加一些空间

                ui.horizontal(|ui| {
                    let button_size = egui::vec2(150.0, 50.0); // 按钮尺寸
                    let button_text = |text: &str| RichText::new(text).size(30.0); // 按钮文字大小

                    if ui.add_sized(button_size, egui::Button::new(button_text("奖励"))).clicked() {
                        self.mode = "奖励".to_owned();
                        self.result = self.draw_lottery(&self.mode);
                    }
                    ui.add_space(20.0); // 按钮之间的空间
                    if ui.add_sized(button_size, egui::Button::new(button_text("惩罚"))).clicked() {
                        self.mode = "惩罚".to_owned();
                        self.result = self.draw_lottery(&self.mode);
                    }
                });

                ui.add_space(20.0); // 添加一些空间

                ui.label(RichText::new(format!("当前模式：{}", self.mode)).heading().size(25.0));
                ui.label(RichText::new(format!("抽奖结果：{}", self.result)).heading().size(25.0));
            });
        });
    }
}

impl MyApp {
    fn draw_lottery(&self, mode: &str) -> String {
        let mut rng = thread_rng();
        match mode {
            "奖励" => self.rewards_pool.choose(&mut rng).unwrap_or(&"无效的模式").to_string(),
            "惩罚" => self.punishments_pool.choose(&mut rng).unwrap_or(&"无效的模式").to_string(),
            _ => "无效的模式，请选择‘奖励’或‘惩罚’。".to_string(),
        }
    }

    fn setup_fonts(&self, ctx: &egui::Context) {
        let mut fonts = FontDefinitions::default();

        // 使用相对路径加载字体文件
        fonts.font_data.insert(
            "my_font".to_owned(),
            egui::FontData::from_static(include_bytes!("../assets/SourceHanSansCN-VF.otf")),
        );

        // 设置字体族
        fonts
            .families
            .get_mut(&FontFamily::Proportional)
            .unwrap()
            .insert(0, "my_font".to_owned());
        fonts
            .families
            .get_mut(&FontFamily::Monospace)
            .unwrap()
            .push("my_font".to_owned());

        // 设置文本样式
        ctx.set_fonts(fonts);

        // 设置默认文本样式大小
        let mut style = (*ctx.style()).clone();
        style.text_styles.insert(TextStyle::Heading, egui::FontId::new(30.0, FontFamily::Proportional));
        style.text_styles.insert(TextStyle::Body, egui::FontId::new(20.0, FontFamily::Proportional));
        ctx.set_style(style);
    }
}
