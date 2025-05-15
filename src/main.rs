use std::{collections::HashMap, fs::File, path::Path};
use chrono::{Datelike, Local, NaiveDate};
use iced::{self, widget::{Button, Column, Row}, Font, Length, Sandbox};
use serde::{Deserialize,Serialize};
use serde_json;
use iced::widget::Text;  
use iced::alignment::Horizontal;  // 导入 Horizontal 对齐类型
use iced::Color;                // 颜色类型
use iced::Background;           // 背景设置    

use iced::widget::button::StyleSheet; // 按钮样式 Trait
use std::boxed::Box;            // 用于包装自定义样式
use iced::Theme;
struct CustomButtonStyle {
    background: Color,
    text_color: Color,
}

// 实现样式 Trait
impl StyleSheet for CustomButtonStyle {
    type Style = Theme; // 关联主题类型

    fn active(&self, _style: &Self::Style) -> iced::widget::button::Appearance {
        iced::widget::button::Appearance {
            background: Some(Background::Color(self.background)),
            text_color: self.text_color,
            ..Default::default()
        }
    }
}



struct HabitTracker {
    data: HabitData,
    current_month : NaiveDate,
    today : NaiveDate,
    // prev_button : button::State,
    // next_button : button::State,

}
#[derive(Debug, Clone, Serialize, Deserialize)]
struct HabitData {
    records : HashMap<NaiveDate,bool>,
}

impl HabitData {
    fn new() -> Self {
        Self {
            records : HashMap::new(),
        }
    }

    fn save(&self) -> Result< (),Box<dyn std::error::Error> > {
        let path = "habit_data.json";
        let file = File::create(path)?;
        serde_json::to_writer(file,self)?;
        Ok(())
    }

    fn load() -> Self {
        let path = "habit_data.json";
        if Path::new(path).exists() {
            if let Ok(file) = File::open(path) {
                if let Ok(data) = serde_json::from_reader(file) {
                    return data;
                }
            }
        }
        Self::new()

    }
}

#[derive(Debug, Clone)]
enum Message {
    ToggleDate(NaiveDate),
    PrevMonth,
    NextMonth,
}

impl Sandbox for HabitTracker {
    type Message = Message;

    fn new() -> Self {
        let today = Local::now().date_naive();
        HabitTracker { 
            data: HabitData::load(),
            current_month : today,
            today,
            // prev_button : button::State::new(),
            // next_button : button::State::new(),
         }
    }

    fn title(&self) -> String {
        String::from("每日打卡")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::ToggleDate(data) => {
                if data == self.today {
                    let entry = self.data.records //获取hashMap
                                            .entry(data)//根据key 获取数据
                                            .or_insert(false); //如果不存在增加一个，设置值为 false
                    *entry = ! *entry;
                    let _ = self.data.save();
                }
            }
            Message::PrevMonth => {
                let first_day = self.current_month.with_day(1).unwrap(); // 获取当前月第一天
                self.current_month = first_day.checked_sub_months(chrono::Months::new(1)).unwrap(); //减去一个月
            }
            Message::NextMonth => {
                let first_day = self.current_month.with_day(1).unwrap();
                self.current_month = first_day.checked_add_months(chrono::Months::new(1)).unwrap();
            }
        }
    }

    fn view(&self) -> iced::Element<'_, Self::Message> {
        let month_title = self.current_month.format("%Y年%m月").to_string();
        // 占位符	含义	            示例（2023-10-10 15:30:45）
        //     %Y	四位年份	            2023
        //     %y	两位年份（后两位）	    23
        //     %m	两位月份（补零）	    10
        //     %b	缩写的英文月份	        Oct
        //     %B	完整英文月份	        October
        //     %d	两位日期（补零）	    10
        //     %H	24小时制的小时	        15
        //     %M	两位分钟	            30
        //     %S	两位秒	                45
        //     %a	缩写的星期几	        Tue
        //     %A	完整的星期几	        Tuesday

        let total = self.data.records.values()//获取值
                        .filter(|&&v| v)//过滤
                        .count(); //统计个数
        
        let streak = current_streak(&self.data, self.today);

        let mut calendar = Column::new()//创建垂直容器
                                .align_items(iced::Alignment::Center)//设置子元素水平居中
                                .spacing(20);//设置子元素间距为10像素

        let nav_row = Row::new()
                    .spacing(20)
                    .push(Button::new(Text::new("<-"))
                                        .on_press(Message::PrevMonth) 
                         )
                    .push(Text::new(month_title).size(20))
                    .push(Button::new(Text::new("->"))
                        .on_press(Message::NextMonth)
                    );
        calendar = calendar.push(nav_row);

        let weekdays = ["日", "一", "二", "三", "四", "五", "六"];
        let week_row = Row::new().spacing(5);
        let week_row = weekdays.iter().fold(week_row, |row,day|{
                row.push(Text::new(*day).width(Length::Fixed(60.0)))
        }) ;

        calendar = calendar.push(week_row);

        let date = self.current_month.with_day(1).unwrap();
        let weekday_num = date.weekday().num_days_from_sunday();
        let mut date = date - chrono::Duration::days(weekday_num as i64);

        for j in 0..6 {
            let mut week_row = Row::new().spacing(5);
            for i in 0..7 {

                if (date.month() != self.current_month.month()) && i == 0 && j != 0{
                    break;
                }
                let is_today = date == self.today;
                let is_marked = self.data.records.get(&date).copied().unwrap_or(false);

                let label = if is_today && is_marked {
                    "√"
                } else if is_today {
                    "今天"
                } else if is_marked{
                    "√"
                }
                else {
                    ""
                };
                
                let (text,btstyle) = if date.month() != self.current_month.month() {
                    (Text::new(" ")
                    .horizontal_alignment(Horizontal::Center)
                    .width(Length::Fixed(60.0)),
                        CustomButtonStyle {
                            background: Color::TRANSPARENT,
                            text_color: Color::TRANSPARENT,
                        })
                }
                else {
                    (Text::new(format!("{}\n{}",date.day(),label))
                    .horizontal_alignment(Horizontal::Center)
                    .width(Length::Fixed(60.0)),
                            CustomButtonStyle {
                        background: Color::from_rgb(1.0, 0.6, 0.0),
                        text_color: Color::BLACK,
                    })
                };
                
                let mut btn = Button::new(text)
                        .style(iced::theme::Button::Custom(Box::new(btstyle)) );

                if is_today {
                    btn = btn.on_press(Message::ToggleDate(date));
                }

                week_row = week_row.push(btn);

                date = date.succ_opt().unwrap();
            }

            calendar = calendar.push(week_row);
        }

        let stats = Column::new()
        .push(Text::new(format!("总打卡次数：{}",total)))
        .push(Text::new(format!("当前连续打卡次数: {}",streak)));

    Column::new()
        .padding(20)
        .spacing(20)
        .push(stats)
        .push(calendar)
        .into()

    }
}


fn current_streak(data : &HabitData,today: NaiveDate) -> u32 {
    let mut current = today;
    let mut streak = 0;

    while let Some(&marked) = data.records.get(&current) {
        if marked {
            streak += 1;
            current = current.pred_opt().unwrap();// pred_opt() 返回前一天
        } else {
            break;
        }
    }

    streak
}


fn main () -> iced::Result {
    let settings = iced::Settings {
        default_font : Font::with_name("Microsoft YaHei"),
        ..iced::Settings::default()
    };
    HabitTracker::run(settings)
}