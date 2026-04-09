use eframe::egui;
use chrono::Local;
use std::collections::VecDeque;
use sys_locale;

fn calculator_word(locale: &str) -> &'static str {
    match locale.to_lowercase().as_str() {
        "ru" | "ru-ru" => "Калькулятор",
        "zh" | "zh-cn" | "zh-tw" => "计算器",
        "es" | "es-es" => "Calculadora",
        "fr" | "fr-fr" => "Calculatrice",
        "de" | "de-de" => "Rechner",
        "pt" | "pt-pt" | "pt-br" => "Calculadora",
        "hi" | "hi-in" => "कैलकुलेटर",
        "bn" | "bn-bd" => "ক্যালকুলেটর",
        "ar" | "ar-sa" => "حاسبة",
        _ => "Calculator",
    }
}

#[derive(Clone, Debug)]
struct Localization {
    app_title: String,
    history_title: String,
    history_empty: String,
    close_button: String,
    error: String,
}

impl Localization {
    fn new(locale: &str) -> Self {
        let calculator = calculator_word(locale);
        Self {
            app_title: format!("{} Free Math", calculator),
            history_title: match locale.to_lowercase().as_str() {
                "ru" | "ru-ru" => "История".to_string(),
                "zh" | "zh-cn" | "zh-tw" => "历史".to_string(),
                "es" | "es-es" => "Historia".to_string(),
                "fr" | "fr-fr" => "Historique".to_string(),
                "de" | "de-de" => "Geschichte".to_string(),
                "pt" | "pt-pt" | "pt-br" => "História".to_string(),
                "hi" | "hi-in" => "इतिहास".to_string(),
                "bn" | "bn-bd" => "ইতিহাস".to_string(),
                "ar" | "ar-sa" => "التاريخ".to_string(),
                _ => "History".to_string(),
            },
            history_empty: match locale.to_lowercase().as_str() {
                "ru" | "ru-ru" => "История пуста".to_string(),
                "zh" | "zh-cn" | "zh-tw" => "历史为空".to_string(),
                "es" | "es-es" => "La historia está vacía".to_string(),
                "fr" | "fr-fr" => "L'historique est vide".to_string(),
                "de" | "de-de" => "Die Geschichte ist leer".to_string(),
                "pt" | "pt-pt" | "pt-br" => "O histórico está vazio".to_string(),
                "hi" | "hi-in" => "इतिहास खाली है".to_string(),
                "bn" | "bn-bd" => "ইতিহাস খালি".to_string(),
                "ar" | "ar-sa" => "التاريخ فارغ".to_string(),
                _ => "History is empty".to_string(),
            },
            close_button: match locale.to_lowercase().as_str() {
                "ru" | "ru-ru" => "Закрыть".to_string(),
                "zh" | "zh-cn" | "zh-tw" => "关闭".to_string(),
                "es" | "es-es" => "Cerrar".to_string(),
                "fr" | "fr-fr" => "Fermer".to_string(),
                "de" | "de-de" => "Schließen".to_string(),
                "pt" | "pt-pt" | "pt-br" => "Fechar".to_string(),
                "hi" | "hi-in" => "बंद करें".to_string(),
                "bn" | "bn-bd" => "বন্ধ করুন".to_string(),
                "ar" | "ar-sa" => "إغلاق".to_string(),
                _ => "Close".to_string(),
            },
            error: match locale.to_lowercase().as_str() {
                "ru" | "ru-ru" => "Ошибка".to_string(),
                "zh" | "zh-cn" | "zh-tw" => "错误".to_string(),
                "es" | "es-es" => "Error".to_string(),
                "fr" | "fr-fr" => "Erreur".to_string(),
                "de" | "de-de" => "Fehler".to_string(),
                "pt" | "pt-pt" | "pt-br" => "Erro".to_string(),
                "hi" | "hi-in" => "त्रुटि".to_string(),
                "bn" | "bn-bd" => "ত্রুটি".to_string(),
                "ar" | "ar-sa" => "خطأ".to_string(),
                _ => "Error".to_string(),
            },
        }
    }
}

fn load_font() -> Option<Vec<u8>> {
    #[cfg(target_os = "windows")]
    {
        let windows_fonts = [
            r"C:\Windows\Fonts\msyh.ttc",
            r"C:\Windows\Fonts\arialuni.ttf",
        ];
        for path in windows_fonts {
            if let Ok(data) = std::fs::read(path) {
                return Some(data);
            }
        }
    }

    #[cfg(target_os = "linux")]
    {
        let linux_fonts = [
            "/usr/share/fonts/truetype/noto/NotoSansCJK-Regular.ttc",
            "/usr/share/fonts/truetype/noto/NotoSansCJKsc-Regular.otf",
            "/usr/share/fonts/truetype/noto/NotoSansDevanagari-Regular.ttf",
            "/usr/share/fonts/truetype/noto/NotoSansArabic-Regular.ttf",
            "/usr/share/fonts/truetype/noto/NotoSans-Regular.ttf",
            "/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf",
            "/usr/share/fonts/truetype/liberation/LiberationSans-Regular.ttf",
        ];
        for path in linux_fonts {
            if let Ok(data) = std::fs::read(path) {
                return Some(data);
            }
        }
    }

    None
}

fn main() -> Result<(), eframe::Error> {
    let locale = sys_locale::get_locale().unwrap_or_else(|| "en".to_string());
    let loc = Localization::new(&locale);
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([420.0, 640.0])
            .with_min_inner_size([420.0, 640.0]),
        ..Default::default()
    };

    eframe::run_native(
        &loc.app_title,
        options,
        Box::new(|cc| {
            if let Some(font_data) = load_font() {
                let mut fonts = egui::FontDefinitions::default();
                fonts.font_data.insert("unicode_font".to_string(), egui::FontData::from_owned(font_data));
                fonts.families.get_mut(&egui::FontFamily::Proportional).unwrap().insert(0, "unicode_font".to_string());
                fonts.families.get_mut(&egui::FontFamily::Monospace).unwrap().insert(0, "unicode_font".to_string());
                cc.egui_ctx.set_fonts(fonts);
            }
            Ok(Box::new(CalculatorApp::default()))
        }),
    )
}

#[derive(Clone, Debug)]
struct HistoryEntry {
    time: String,
    steps: Vec<String>,
}

struct CalculatorApp {
    current_expression: String,
    history_groups: VecDeque<HistoryEntry>,
    last_answer: Option<f64>,
    show_history: bool,
    hint: String,
    locale: String,
}

impl Default for CalculatorApp {
    fn default() -> Self {
        Self {
            current_expression: String::new(),
            history_groups: VecDeque::new(),
            last_answer: None,
            show_history: false,
            hint: String::new(),
            locale: sys_locale::get_locale().unwrap_or_else(|| "en".to_string()),
        }
    }
}

impl CalculatorApp {
    fn get_localization(&self) -> Localization {
        Localization::new(&self.locale)
    }

    fn safe_insert(&mut self, char_input: &str) {
        let loc = self.get_localization();
        if self.current_expression == loc.error {
            self.current_expression.clear();
        }
        self.current_expression.push_str(char_input);
        self.update_hint();
    }

    fn clear_all(&mut self) {
        self.current_expression.clear();
        self.last_answer = None;
        self.hint.clear();
    }

    fn backspace(&mut self) {
        self.current_expression.pop();
        self.update_hint();
    }

    fn prepare_math(&self, expr: &str) -> String {
        let mut result = expr
            .replace('^', "**")
            .replace('√', "sqrt(")
            .replace("π", &std::f64::consts::PI.to_string())
            .replace('e', &std::f64::consts::E.to_string());

        result = result
            .replace("sin(", "sin_rad(")
            .replace("cos(", "cos_rad(")
            .replace("tan(", "tan_rad(");

        if result.contains('!') {
            result = self.replace_factorial(&result);
        }

        result
    }

    fn replace_factorial(&self, expr: &str) -> String {
        let mut result = String::new();
        let chars: Vec<char> = expr.chars().collect();
        let mut i = 0;

        while i < chars.len() {
            if chars[i] == '!' && i > 0 && chars[i - 1].is_numeric() {
                let mut num_start = i - 1;
                while num_start > 0 && chars[num_start - 1].is_numeric() {
                    num_start -= 1;
                }
                result.truncate(result.len() - (i - num_start));
                let num_str: String = chars[num_start..i].iter().collect();
                if let Ok(n) = num_str.parse::<u32>() {
                    result.push_str(&self.factorial(n).to_string());
                }
            } else {
                result.push(chars[i]);
            }
            i += 1;
        }
        result
    }

    fn factorial(&self, n: u32) -> f64 {
        match n {
            0 | 1 => 1.0,
            n => (2..=n).fold(1.0, |acc, i| acc * i as f64),
        }
    }

    fn evaluate(&self, expr: &str) -> Result<f64, String> {
        let prepared = self.prepare_math(expr);
        self.eval_expression(&prepared)
    }

    fn eval_expression(&self, expr: &str) -> Result<f64, String> {
        self.parse_expression(expr).map(|(result, _)| result)
    }

    fn parse_expression(&self, input: &str) -> Result<(f64, usize), String> {
        let trimmed = input.trim();
        let chars: Vec<char> = trimmed.chars().collect();
        let (result, pos) = self.parse_add_sub(&chars, 0)?;

        if pos < chars.len() {
            return Err("Unexpected characters".to_string());
        }
        Ok((result, pos))
    }

    fn parse_add_sub(&self, chars: &[char], mut pos: usize) -> Result<(f64, usize), String> {
        let (mut result, new_pos) = self.parse_mul_div(chars, pos)?;
        pos = new_pos;

        while pos < chars.len() {
            match chars[pos] {
                '+' => {
                    pos += 1;
                    let (right, new_pos) = self.parse_mul_div(chars, pos)?;
                    result += right;
                    pos = new_pos;
                }
                '-' if pos > 0 && !matches!(chars[pos - 1], '+' | '-' | '*' | '/' | '(') => {
                    pos += 1;
                    let (right, new_pos) = self.parse_mul_div(chars, pos)?;
                    result -= right;
                    pos = new_pos;
                }
                _ => break,
            }
        }
        Ok((result, pos))
    }

    fn parse_mul_div(&self, chars: &[char], mut pos: usize) -> Result<(f64, usize), String> {
        let (mut result, new_pos) = self.parse_power(chars, pos)?;
        pos = new_pos;

        while pos < chars.len() {
            match chars[pos] {
                '*' => {
                    pos += 1;
                    let (right, new_pos) = self.parse_power(chars, pos)?;
                    result *= right;
                    pos = new_pos;
                }
                '/' => {
                    pos += 1;
                    let (right, new_pos) = self.parse_power(chars, pos)?;
                    if right.abs() < 1e-10 {
                        return Err("Division by zero".to_string());
                    }
                    result /= right;
                    pos = new_pos;
                }
                _ => break,
            }
        }
        Ok((result, pos))
    }

    fn parse_power(&self, chars: &[char], mut pos: usize) -> Result<(f64, usize), String> {
        let (mut result, new_pos) = self.parse_unary(chars, pos)?;
        pos = new_pos;

        while pos < chars.len() && chars[pos] == '*' && pos + 1 < chars.len() && chars[pos + 1] == '*'
        {
            pos += 2;
            let (right, new_pos) = self.parse_unary(chars, pos)?;
            result = result.powf(right);
            pos = new_pos;
        }
        Ok((result, pos))
    }

    fn parse_unary(&self, chars: &[char], mut pos: usize) -> Result<(f64, usize), String> {
        if pos < chars.len() && chars[pos] == '-' {
            pos += 1;
            let (value, new_pos) = self.parse_primary(chars, pos)?;
            Ok((-value, new_pos))
        } else if pos < chars.len() && chars[pos] == '+' {
            pos += 1;
            self.parse_primary(chars, pos)
        } else {
            self.parse_primary(chars, pos)
        }
    }

    fn parse_primary(&self, chars: &[char], mut pos: usize) -> Result<(f64, usize), String> {
        if pos >= chars.len() {
            return Err("Unexpected end of expression".to_string());
        }

        if chars[pos] == '(' {
            pos += 1;
            let (result, new_pos) = self.parse_add_sub(chars, pos)?;
            pos = new_pos;
            if pos >= chars.len() || chars[pos] != ')' {
                return Err("Missing closing parenthesis".to_string());
            }
            Ok((result, pos + 1))
        } else if self.is_function_start(chars, pos) {
            self.parse_function(chars, pos)
        } else {
            self.parse_number(chars, pos)
        }
    }

    fn is_function_start(&self, chars: &[char], pos: usize) -> bool {
        if pos + 4 <= chars.len() {
            let s: String = chars[pos..pos.min(pos + 4)].iter().collect();
            if s.starts_with("sin_") || s.starts_with("cos_") || s.starts_with("tan_") || s.starts_with("exp(") {
                return true;
            }
        }
        if pos + 5 <= chars.len() {
            let s: String = chars[pos..pos.min(pos + 5)].iter().collect();
            if s.starts_with("sqrt(") {
                return true;
            }
        }
        false
    }

    fn parse_function(&self, chars: &[char], mut pos: usize) -> Result<(f64, usize), String> {
        let func_name: String = chars[pos..pos.min(pos + 5)]
            .iter()
            .take_while(|&&c| c.is_alphabetic() || c == '_')
            .collect();

        pos += func_name.len();

        let (arg, new_pos) = if chars[pos] == '(' {
            pos += 1;
            let (val, new_pos) = self.parse_add_sub(chars, pos)?;
            if new_pos >= chars.len() || chars[new_pos] != ')' {
                return Err("Missing closing parenthesis in function".to_string());
            }
            (val, new_pos + 1)
        } else {
            return Err(format!("Expected '(' after function {}", func_name));
        };

        let result = match func_name.as_str() {
            "sin_" => arg.sin(),
            "cos_" => arg.cos(),
            "tan_" => arg.tan(),
            "sqrt" => {
                if arg < 0.0 {
                    return Err("Square root of negative number".to_string());
                }
                arg.sqrt()
            }
            "exp" => arg.exp(),
            _ => return Err(format!("Unknown function: {}", func_name)),
        };

        Ok((result, new_pos))
    }

    fn parse_number(&self, chars: &[char], mut pos: usize) -> Result<(f64, usize), String> {
        let start = pos;
        let mut has_dot = false;

        while pos < chars.len() && (chars[pos].is_numeric() || (chars[pos] == '.' && !has_dot))
        {
            if chars[pos] == '.' {
                has_dot = true;
            }
            pos += 1;
        }

        if pos == start {
            return Err("Expected number".to_string());
        }

        let num_str: String = chars[start..pos].iter().collect();
        let num = num_str
            .parse::<f64>()
            .map_err(|_| "Invalid number".to_string())?;

        Ok((num, pos))
    }

    fn calculate(&mut self) {
        let loc = self.get_localization();
        if self.current_expression.is_empty() || self.current_expression == loc.error {
            return;
        }

        match self.evaluate(&self.current_expression) {
            Ok(result) => {
                let result_rounded = (result * 100000000.0).round() / 100000000.0;
                let result_str = if result_rounded.fract() == 0.0 {
                    format!("{:.0}", result_rounded)
                } else {
                    format!("{}", result_rounded)
                };

                let time = Local::now().format("%H:%M:%S").to_string();
                let entry = format!("{} = {}", self.current_expression, result_str);

                let mut appended = false;
                if let Some(last) = self.last_answer {
                    if !self.history_groups.is_empty()
                        && self.current_expression.starts_with(&last.to_string())
                    {
                        if let Some(last_group) = self.history_groups.back_mut() {
                            last_group.steps.push(entry.clone());
                            appended = true;
                        }
                    }
                }

                if !appended {
                    self.history_groups.push_back(HistoryEntry {
                        time,
                        steps: vec![entry],
                    });
                }

                self.last_answer = Some(result_rounded);
                self.current_expression = result_str;
                self.hint.clear();
            }
            Err(_) => {
                self.current_expression = loc.error.clone();
                self.hint.clear();
            }
        }
    }

    fn update_hint(&mut self) {
        let loc = self.get_localization();
        if self.current_expression.is_empty() || self.current_expression == loc.error {
            self.hint.clear();
            return;
        }

        let open_parens = self.current_expression.matches('(').count();
        let close_parens = self.current_expression.matches(')').count();

        if open_parens == close_parens && self.current_expression.chars().any(|c| c.is_numeric()) {
            match self.evaluate(&self.current_expression) {
                Ok(result) => {
                    let rounded = (result * 1000000.0).round() / 1000000.0;
                    self.hint = format!("= {}", rounded);
                }
                Err(_) => self.hint.clear(),
            }
        } else {
            self.hint.clear();
        }
    }

    fn get_font_size(&self) -> f32 {
        let len = self.current_expression.len();
        if len > 40 {
            14.0
        } else if len > 20 {
            20.0
        } else {
            30.0
        }
    }

}

impl eframe::App for CalculatorApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.set_pixels_per_point(1.1);

        let loc = self.get_localization();

        ctx.input(|i| {
            for key_event in &i.events {
                if let egui::Event::Key {
                    key,
                    pressed: true,
                    ..
                } = key_event
                {
                    match key {
                        egui::Key::Enter => self.calculate(),
                        egui::Key::Escape => self.clear_all(),
                        egui::Key::Backspace => self.backspace(),
                        _ => {}
                    }
                }
            }

            for key_event in &i.events {
                if let egui::Event::Text(text) = key_event {
                    if let Some(c) = text.chars().next() {
                        if c.is_numeric() || "+-*/().^".contains(c) {
                            self.safe_insert(&c.to_string());
                        }
                    }
                }
            }
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical(|ui| {
                if ui.button("📜").clicked() {
                    self.show_history = !self.show_history;
                }

                ui.add_space(8.0);

                let display_height = 120.0;
                let (_display_id, display_rect) = ui.allocate_space(egui::vec2(ui.available_width(), display_height));
                ui.allocate_ui_at_rect(display_rect, |ui| {
                    egui::Frame::none()
                        .fill(egui::Color32::from_rgb(20, 20, 20))
                        .rounding(egui::Rounding::same(5.0))
                        .inner_margin(egui::Margin::same(12.0))
                        .show(ui, |ui| {
                            ui.with_layout(egui::Layout::top_down(egui::Align::RIGHT), |ui| {
                                let expr_height = 80.0;
                                let (_expr_id, expr_rect) = ui.allocate_space(egui::vec2(ui.available_width(), expr_height));
                                ui.allocate_ui_at_rect(expr_rect, |ui| {
                                    ui.label(
                                        egui::RichText::new(self.current_expression.as_str())
                                            .size(self.get_font_size())
                                            .color(egui::Color32::from_rgb(235, 235, 235)),
                                    );
                                });

                                let hint_height = 40.0;
                                let (_hint_id, hint_rect) = ui.allocate_space(egui::vec2(ui.available_width(), hint_height));
                                ui.allocate_ui_at_rect(hint_rect, |ui| {
                                    if !self.hint.is_empty() {
                                        ui.label(
                                            egui::RichText::new(self.hint.as_str())
                                                .size(16.0)
                                                .color(egui::Color32::from_rgb(170, 170, 170)),
                                        );
                                    }
                                });
                            });
                        });
                });

                ui.add_space(12.0);

                ui.vertical_centered(|ui| {
                    ui.with_layout(egui::Layout::top_down(egui::Align::LEFT), |ui| {
                        ui.horizontal(|ui| {
                            ui.spacing_mut().item_spacing = egui::vec2(8.0, 8.0);
                            if ui.add_sized(
                                [85.0, 50.0],
                                egui::Button::new(egui::RichText::new("C").color(egui::Color32::WHITE))
                                    .fill(egui::Color32::from_rgb(255, 59, 48)),
                            )
                            .clicked()
                            {
                                self.clear_all();
                            }
                            if ui.add_sized(
                                [85.0, 50.0],
                                egui::Button::new(egui::RichText::new("√").color(egui::Color32::WHITE))
                                    .fill(egui::Color32::from_rgb(68, 68, 68)),
                            )
                            .clicked()
                            {
                                self.safe_insert("√(");
                            }
                            if ui.add_sized(
                                [85.0, 50.0],
                            egui::Button::new(egui::RichText::new("e").color(egui::Color32::WHITE))
                                .fill(egui::Color32::from_rgb(68, 68, 68)),
                        )
                        .clicked()
                        {
                            self.safe_insert("e");
                        }
                        if ui.add_sized(
                            [85.0, 50.0],
                            egui::Button::new(
                                egui::RichText::new("DEL")
                                    .size(22.0)
                                    .color(egui::Color32::WHITE),
                            )
                            .fill(egui::Color32::from_rgb(68, 68, 68)),
                        )
                        .clicked()
                        {
                            self.backspace();
                        }
                    });

                        ui.add_space(8.0);
                        ui.horizontal(|ui| {
                            ui.spacing_mut().item_spacing = egui::vec2(8.0, 8.0);
                            if ui.add_sized(
                                [85.0, 50.0],
                                egui::Button::new(egui::RichText::new("^").color(egui::Color32::WHITE))
                                    .fill(egui::Color32::from_rgb(68, 68, 68)),
                            )
                            .clicked()
                            {
                                self.safe_insert("^");
                            }
                            if ui.add_sized(
                                [85.0, 50.0],
                                egui::Button::new(egui::RichText::new("π").color(egui::Color32::WHITE))
                                    .fill(egui::Color32::from_rgb(68, 68, 68)),
                            )
                            .clicked()
                            {
                                self.safe_insert("π");
                            }
                            if ui.add_sized(
                                [85.0, 50.0],
                                egui::Button::new(egui::RichText::new("(").color(egui::Color32::WHITE))
                                    .fill(egui::Color32::from_rgb(68, 68, 68)),
                            )
                            .clicked()
                            {
                                self.safe_insert("(");
                            }
                            if ui.add_sized(
                                [85.0, 50.0],
                                egui::Button::new(egui::RichText::new(")").color(egui::Color32::WHITE))
                                    .fill(egui::Color32::from_rgb(68, 68, 68)),
                            )
                            .clicked()
                            {
                                self.safe_insert(")");
                            }
                        });

                        ui.add_space(8.0);
                        ui.horizontal(|ui| {
                            ui.spacing_mut().item_spacing = egui::vec2(8.0, 8.0);
                            if ui.add_sized(
                                [85.0, 50.0],
                                egui::Button::new(egui::RichText::new("tan").color(egui::Color32::WHITE))
                                    .fill(egui::Color32::from_rgb(85, 85, 85)),
                            )
                            .clicked()
                            {
                                self.safe_insert("tan(");
                            }
                            if ui.add_sized(
                                [85.0, 50.0],
                                egui::Button::new(egui::RichText::new("n!").color(egui::Color32::WHITE))
                                    .fill(egui::Color32::from_rgb(85, 85, 85)),
                            )
                            .clicked()
                            {
                                self.safe_insert("!");
                            }
                            if ui.add_sized(
                                [85.0, 50.0],
                                egui::Button::new(egui::RichText::new("log").color(egui::Color32::WHITE))
                                    .fill(egui::Color32::from_rgb(85, 85, 85)),
                            )
                            .clicked()
                            {
                                self.safe_insert("log(");
                            }
                            if ui.add_sized(
                                [85.0, 50.0],
                                egui::Button::new(egui::RichText::new("ln").color(egui::Color32::WHITE))
                                    .fill(egui::Color32::from_rgb(85, 85, 85)),
                            )
                            .clicked()
                            {
                                self.safe_insert("ln(");
                            }
                        });

                        ui.add_space(8.0);
                        ui.horizontal(|ui| {
                            ui.spacing_mut().item_spacing = egui::vec2(8.0, 8.0);
                            if ui.add_sized(
                                [85.0, 50.0],
                                egui::Button::new(egui::RichText::new("7").color(egui::Color32::WHITE))
                                    .fill(egui::Color32::from_rgb(51, 51, 51)),
                            )
                            .clicked()
                            {
                                self.safe_insert("7");
                            }
                            if ui.add_sized(
                                [85.0, 50.0],
                                egui::Button::new(egui::RichText::new("8").color(egui::Color32::WHITE))
                                    .fill(egui::Color32::from_rgb(51, 51, 51)),
                            )
                            .clicked()
                            {
                                self.safe_insert("8");
                            }
                            if ui.add_sized(
                                [85.0, 50.0],
                                egui::Button::new(egui::RichText::new("9").color(egui::Color32::WHITE))
                                    .fill(egui::Color32::from_rgb(51, 51, 51)),
                            )
                            .clicked()
                            {
                                self.safe_insert("9");
                            }
                            if ui.add_sized(
                                [85.0, 50.0],
                                egui::Button::new(egui::RichText::new("/").color(egui::Color32::WHITE))
                                    .fill(egui::Color32::from_rgb(255, 149, 0)),
                            )
                            .clicked()
                            {
                                self.safe_insert("/");
                            }
                        });

                        ui.add_space(8.0);
                        ui.horizontal(|ui| {
                            ui.spacing_mut().item_spacing = egui::vec2(8.0, 8.0);
                            if ui.add_sized(
                                [85.0, 50.0],
                                egui::Button::new(egui::RichText::new("4").color(egui::Color32::WHITE))
                                    .fill(egui::Color32::from_rgb(51, 51, 51)),
                            )
                            .clicked()
                            {
                                self.safe_insert("4");
                            }
                            if ui.add_sized(
                                [85.0, 50.0],
                                egui::Button::new(egui::RichText::new("5").color(egui::Color32::WHITE))
                                    .fill(egui::Color32::from_rgb(51, 51, 51)),
                            )
                            .clicked()
                            {
                                self.safe_insert("5");
                            }
                            if ui.add_sized(
                                [85.0, 50.0],
                                egui::Button::new(egui::RichText::new("6").color(egui::Color32::WHITE))
                                    .fill(egui::Color32::from_rgb(51, 51, 51)),
                            )
                            .clicked()
                            {
                                self.safe_insert("6");
                            }
                            if ui.add_sized(
                                [85.0, 50.0],
                                egui::Button::new(egui::RichText::new("*").color(egui::Color32::WHITE))
                                    .fill(egui::Color32::from_rgb(255, 149, 0)),
                            )
                            .clicked()
                            {
                                self.safe_insert("*");
                            }
                        });

                        ui.add_space(8.0);
                        ui.horizontal(|ui| {
                            ui.spacing_mut().item_spacing = egui::vec2(8.0, 8.0);
                            if ui.add_sized(
                                [85.0, 50.0],
                                egui::Button::new(egui::RichText::new("1").color(egui::Color32::WHITE))
                                    .fill(egui::Color32::from_rgb(51, 51, 51)),
                            )
                            .clicked()
                            {
                                self.safe_insert("1");
                            }
                            if ui.add_sized(
                                [85.0, 50.0],
                                egui::Button::new(egui::RichText::new("2").color(egui::Color32::WHITE))
                                    .fill(egui::Color32::from_rgb(51, 51, 51)),
                            )
                            .clicked()
                            {
                                self.safe_insert("2");
                            }
                            if ui.add_sized(
                                [85.0, 50.0],
                                egui::Button::new(egui::RichText::new("3").color(egui::Color32::WHITE))
                                    .fill(egui::Color32::from_rgb(51, 51, 51)),
                            )
                            .clicked()
                            {
                                self.safe_insert("3");
                            }
                            if ui.add_sized(
                                [85.0, 50.0],
                                egui::Button::new(egui::RichText::new("-").color(egui::Color32::WHITE))
                                    .fill(egui::Color32::from_rgb(255, 149, 0)),
                            )
                            .clicked()
                            {
                                self.safe_insert("-");
                            }
                        });

                        ui.add_space(8.0);
                        ui.horizontal(|ui| {
                            ui.spacing_mut().item_spacing = egui::vec2(8.0, 8.0);
                            if ui.add_sized(
                                [85.0, 50.0],
                                egui::Button::new(egui::RichText::new("0").color(egui::Color32::WHITE))
                                    .fill(egui::Color32::from_rgb(51, 51, 51)),
                            )
                            .clicked()
                            {
                                self.safe_insert("0");
                            }
                            if ui.add_sized(
                                [178.0, 50.0],
                                egui::Button::new(egui::RichText::new("=").color(egui::Color32::WHITE))
                                    .fill(egui::Color32::from_rgb(255, 149, 0)),
                            )
                            .clicked()
                            {
                                self.calculate();
                            }
                            if ui.add_sized(
                                [85.0, 50.0],
                                egui::Button::new(egui::RichText::new("+").color(egui::Color32::WHITE))
                                    .fill(egui::Color32::from_rgb(255, 149, 0)),
                            )
                            .clicked()
                            {
                                self.safe_insert("+");
                            }
                        });

                    });
                });
            });
        });

        if self.show_history {
            egui::Window::new(format!("📜 {}", loc.history_title)).default_width(400.0).show(ctx, |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    if self.history_groups.is_empty() {
                        ui.label(
                            egui::RichText::new(&loc.history_empty)
                                .color(egui::Color32::GRAY),
                        );
                    } else {
                        for group in self.history_groups.iter().rev() {
                            for step in &group.steps {
                                ui.label(
                                    egui::RichText::new(format!("[{}] - {}", group.time, step))
                                        .color(egui::Color32::from_rgb(209, 209, 209)),
                                );
                            }
                        }
                    }
                });

                if ui.button(&loc.close_button).clicked() {
                    self.show_history = false;
                }
            });
        }
    }
}
