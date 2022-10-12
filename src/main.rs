#![allow(non_snake_case)]

use dioxus::prelude::*;
use std::time::Duration;

#[inline_props]
pub fn ClockTime(cx: Scope, now_m: UseState<i32>, now_s: UseState<i32>) -> Element{
    let clock_display = format!("{} : {:0>2}", *now_m.current(), *now_s.current());
    cx.render(rsx!{
        h1{"{clock_display}"}
    })
}

#[inline_props]
pub fn Clock(
    cx: Scope,
    task_time: UseState<i32>, 
    short_break: UseState<i32>, 
    long_break: UseState<i32>
) -> Element{
    let m_count = use_state(&cx, || *task_time.current());
    let s_count = use_state(&cx, || 0);
    let start = use_state(&cx, || false);

        use_future(&cx, (), move |_| {
            let mut clock_count = 0;
            let mut count = *task_time.current()*60;
            let mut m_count = m_count.clone();
            let mut s_count = s_count.clone();
            let task = task_time.clone();
            let s_break = short_break.clone();
            let l_break = long_break.clone();
            let start = start.clone();

            async move {
                loop {
                    tokio::time::sleep(Duration::from_millis(1000)).await;
                if *start.current(){
                    s_count -= 1;
                    count -= 1;
                    if count%60 == 59{
                        m_count -= 1;
                        s_count.set(59);
                    if count < 0{
                        clock_count += 1;
                        s_count.set(0);
                        match clock_count%8{
                            0|2|4|6=>{
                                count = *task.current()*60;
                                m_count.set(*task.current());
                            },
                            7=>{
                                count = *l_break.current()*60;
                                m_count.set(*l_break.current());
                            },
                            _=>{
                                count = *s_break.current()*60;
                                m_count.set(*s_break.current());
                            },    
                            }
                        }
                    }
                } else {
                    m_count.set(*task.current());
                    s_count.set(0);
                    count = *task.current()*60;
            }
        }
        }
    });
    

    cx.render(rsx!{
        button{
            style: "float: left;",
            onclick:move |_ev| {start.set(true);},"Start"
        }
        button{
            style: "float: right;",
            onclick:move |_ev| {start.set(false);},"Stop"
        }
        div{
            style: "text-align: center;",
            ClockTime{now_m: m_count.clone(), now_s: s_count.clone()}
        }
    })
}

#[derive(PartialEq, Eq)]
pub enum Times{
    TaskTime,
    BreakTime,
    ShortBreakTime
}

fn app(cx: Scope) -> Element {
    let change_time = use_state(&cx, || Times::TaskTime);
    let task_time = use_state(&cx, || 27);
    let short_break_time = use_state(&cx, || 3);
    let break_time = use_state(&cx, || 10);

    cx.render(rsx!{
        div{
            select{
                onchange: move |ev| {
                    match ev.value.as_str(){
                    "Task Time"=>change_time.set(Times::TaskTime),
                    "Short Break Time"=>change_time.set(Times::ShortBreakTime),
                    "Break Time"=>change_time.set(Times::BreakTime),
                    _=>(),
                }},
                option{value:"Task Time", "Task Time"}
                option{value:"Short Break Time", "Short Break Time"}
                option{value:"Break Time", "Break Time"}
            }
            select{
                onchange: move |ev| {
                    match **change_time{
                        Times::TaskTime=>task_time.set(ev.value.parse::<i32>().unwrap()),
                        Times::BreakTime=>break_time.set(ev.value.parse::<i32>().unwrap()),
                        Times::ShortBreakTime=>short_break_time.set(ev.value.parse::<i32>().unwrap()),
                    }
                },
                (0..90).map(|m| rsx!{ option{value: "{m}", "{m}"} })
            }
            label{"Minutes"}
        }
        Clock{task_time: task_time.clone(), short_break: short_break_time.clone(), long_break: break_time.clone()}
    })
}

fn main() {
    dioxus::desktop::launch(app);
}
