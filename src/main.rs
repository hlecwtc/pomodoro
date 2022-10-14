#![windows_subsystem = "windows"]
#![allow(non_snake_case)]

use dioxus::prelude::*;
use std::time::Duration;

#[inline_props]
pub fn ClockTime(cx: Scope, count: UseState<i32>, color: UseState<String>) -> Element{
    let clock_display = format!("{} : {:0>2}", *count.current()/60, *count.current()%60);
    cx.render(rsx!{
        h1{
            style: "display: flex; justify-content: center; color: {color}",
            "{clock_display}"
        }
    })
}

#[inline_props]
pub fn Clock(
    cx: Scope,
    task_time: UseState<i32>, 
    short_break: UseState<i32>, 
    long_break: UseState<i32>,
    count: UseState<i32>,
    start: UseState<bool>,
    color: UseState<String>
) -> Element{
    let clock_count = use_state(&cx, || 0);
    let start_button = use_state(&cx, || "block");
    let stop_button = use_state(&cx, || "none");

        use_future(&cx, (), move |_| {
            let mut clock_count = clock_count.clone();
            let mut count = count.clone();
            let task = task_time.clone();
            let s_break = short_break.clone();
            let l_break = long_break.clone();
            let start = start.clone();
            let color = color.clone();

            async move {
                loop {
                    tokio::time::sleep(Duration::from_millis(1000)).await;
                if *start.current(){
                    count -= 1;
                    if *count.current() < 0{
                        clock_count += 1;
                        match *clock_count.current()%8{
                            0|2|4|6=>{
                                count.set(*task.current()*60);
                                color.set("#B22222".to_string());
                            },
                            7=>{
                                count.set(*l_break.current()*60);
                                color.set("#B28B22".to_string());
                            },
                            _=>{
                                count.set(*s_break.current()*60);
                                color.set("#00008B".to_string());
                            },    
                        }
                    }
                }
            }
        }
    });

    cx.render(rsx!{
        ClockTime{count: count.clone(), color: color.clone()}
        div{
        button{
            id: "StartButton",
            style: "display: {start_button};",
            onclick:move |_ev| {
                start.set(true);
                start_button.set("none");
                stop_button.set("block");
            },
            "Start"
        }
        button{
            id: "StopButton",
            style: "display: {stop_button};",
            onclick:move |_ev| {
                start.set(false);
                start_button.set("block");
                stop_button.set("none");
            },
            "Stop"
        }
        button{
            id: "ResetButton",
            style: "display: block;",
            onclick: move |_ev| {
                start.set(false);
                start_button.set("block");
                stop_button.set("none");
                count.set(*task_time.current()*60);
                clock_count.set(0);
                color.set("#B22222".to_string());
            },
            "Reset"
        }
        }
    })
}

#[derive(PartialEq, Eq)]
pub enum Times{
    TaskTime,
    BreakTime,
    ShortBreakTime
}

#[inline_props]
pub fn SetTime(
    cx: Scope,
    time: UseState<i32>,
    count: UseState<i32>,
    start: UseState<bool>,
    max: u32,
    step: u32
) -> Element {
    cx.render(rsx!{
            input{
                r#type: "range",
                min: "0",
                max: "{max}",
                step: "{step}",
                value: "{time}",
                onchange: move |ev| {
                    time.set(ev.value.parse::<i32>().unwrap());
                    if !*start.current(){
                        count.set(ev.value.parse::<i32>().unwrap()*60)
                    }
                }
            }
            label{"{time} minutes"}
            br{}
    })
}

fn app(cx: Scope) -> Element {
    let task_time = use_state(&cx, || 30);
    let short_break_time = use_state(&cx, || 3);
    let break_time = use_state(&cx, || 10);
    let count = use_state(&cx, || 30*60);
    let menu = use_state(&cx, || "none");
    let timer = use_state(&cx, || "block");
    let start = use_state(&cx, || false);
    let color = use_state(&cx, || "#b22222".to_string());

    cx.render(rsx!{
        style { [include_str!("../src/style.css")] }
        div{
            class: "header",
            button{
                onclick: move |_ev| {
                    menu.set("none");
                    timer.set("block");
                },
                "Timer"
            }
            button{
                onclick: move |_ev| {
                    menu.set("block");
                    timer.set("none");
                },    
                "Menu"
            }
        }
        div{
            class: "menu",
            style: "display: {menu}; height: 80%",
            div{
                style: "padding-top: 10px;",
                label{"Focus Time"}
                SetTime{
                    time: task_time.clone(), count: count.clone(), start: start.clone(), max: 60, step: 5
                }
            }
            div{
                label{"Short Break"}
                SetTime{
                    time: short_break_time.clone(), count: count.clone(), start: start.clone(), max: 10, step: 1
                }
            }
            div{
                label{"Long Break"}
                SetTime{
                    time: break_time.clone(), count: count.clone(), start: start.clone(), max: 30, step: 2
                }
            }
        }
        
        div{
            class: "timer",
            style: "display: {timer};",
            Clock{
                task_time: task_time.clone(), short_break: short_break_time.clone(),
                long_break: break_time.clone(), count: count.clone(), start: start.clone(), color: color.clone()
            }
        }
    })
}

fn main() {
    dioxus::desktop::launch_cfg(
        app, |c| c.with_window(|w|w
            .with_title("POMODORO")
            .with_inner_size(
                dioxus::desktop::tao::dpi::LogicalSize::new(330.0, 200.0)
            )
            .with_resizable(false)
        )
    );
}
