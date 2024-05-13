use freya::prelude::*;

use crate::{check_pn_type::do_query, check_version::check_version, data2excel::do_action, updata::updata};

pub fn app() -> Element {
    // use_init_theme(DARK_THEME);
    let mut input_value = use_signal(|| String::new());
    let mut result = use_signal(|| vec![]);
    let mut qty = use_signal(|| 0);
    let mut show_popup = use_signal(|| false);
    let mut popup_text = use_signal(|| String::default());
    let mut check = use_signal(||false);
    let mut checked = use_signal(||false);
    let columns = use_hook(|| {
        vec![
            ("盒号"),
            ("Sn"),
            ("Ith"),
            ("Pf"),
            ("Se"),
            ("Sen"),
            ("Res"),
            ("icc"),
            ("testtime"),
        ]
    });
    
    let query_onclick = move |_| {
        let c = input_value.read().clone();
        spawn(async move {
            let results = do_query(c).await;
            match results {
                Some(res) => {
                    qty.set(res.len());
                    result.set(res);
                },
                None => {
                    popup_text.set("该号码没有找到装箱或者装盒信息，请确认后再进行查询!!!".to_string());
                    show_popup.set(true)
                },
            }
            
        });
    };

    let action_onclick = move |_| {
        let c = input_value.read().clone();
        let d = result.read().clone();
        spawn(async move {
            let results = do_action(d, c).await;
            match results {
                Ok(_) => {
                    let c = input_value.read().clone();
                    popup_text.set(format!("{}已经导出到软件根目录!",c));
                    show_popup.set(true)
                },
                Err(e) => {
                    match e {
                        crate::data2excel::Tip::Ok => {
                            let c = input_value.read().clone();
                            popup_text.set(format!("{}已经导出到软件根目录!",c));
                            show_popup.set(true)
                        },
                        crate::data2excel::Tip::Err { e } => {
                            popup_text.set(e);
                            show_popup.set(true)
                        },
                    }
                },
            }
            // qty.set(results.len());
            // result.set(results);
        });
    };
    let updata_onclick = move |_| {
        
        spawn(async move {
            let results = updata().await;
            match results {
                Ok(_) => {
                    popup_text.set("软件升级成功!".to_string());
                    show_popup.set(true);
                    checked.set(false)
                },
                Err(_e) => {
                    popup_text.set("软件升级失败!".to_string());
                    show_popup.set(true)
                },
            }
            // qty.set(results.len());
            // result.set(results);
        });
    };
    if check.read().clone() == false {
        spawn(async move {
            let results = check_version().await;
            match results {
                Some(res) => {
                    if *res.version > *env!("CARGO_PKG_VERSION") {
                        popup_text.set("软件有更新!".to_string());
                        check.set(true);
                        checked.set(true)
                    }
                },
                None => {
                    popup_text.set("找不到软件版本状态!!".to_string());
                    check.set(true);
                    checked.set(true)
                },
            }
            
        });
    }
        
    rsx!(
        Body {
            if *checked.read() {
                Popup {
                    show_close_button:false,
                    // oncloserequest: move |_| {
                    //     checked.set(false)
                    // },
                    PopupTitle {
                        label {
                            "提示"
                        }
                    }
                    PopupContent {
                        rect {
                            width:"100%",
                            height:"100%",
                            padding: "10",
                            direction: "vertical",
                            main_align: "center",
                            cross_align: "center",
                            rect {
                                width:"100%",
                                height:"90%",
                                main_align: "center",
                                cross_align: "start",
                                label {
                                    text_align: "center",
                                    "{popup_text.read().clone()}"
                                }
                            }
                            rect{
                                cross_align: "end",
                                main_align: "end",
                                Button {
                                    onclick: updata_onclick,
                                    label {
                                        "升级"
                                    }
                                }
                            }
                        }
                    }
                }
            }
            if *show_popup.read() {
                Popup {
                    oncloserequest: move |_| {
                        show_popup.set(false)
                    },
                    PopupTitle {
                        label {
                            "提示"
                        }
                    }
                    PopupContent {
                        label {
                            text_align: "center",
                            "{popup_text.read().clone()}"
                        }
                    }
                }
            }
        rect {
            direction: "vertical",
            rect {
                // padding: "10",
                direction: "horizontal",
                // background: "rgb(15, 15, 15)",
                width: "100%",
                height: "5%",
                rect {
                    overflow: "clip",
                    width: "80%",
                    height: "100%",
                    Input {
                        theme: Some(InputThemeWith{
                            width: Some("100%".into()),
                            ..Default::default()
                        }),
                        value: input_value.read().clone(),
                        onchange: move |txt| {
                            input_value.set(txt);
                        }
                    },
                }
                rect {
                    overflow: "clip",
                    height: "100%",
                    width: "5%",
                    // main_align: "center",
                    // cross_align: "center",
                    Button {
                        onclick: move |_|{
                            input_value.set(String::default())
                        },
                        label {
                            "清空"
                        }
                    }
                }
                rect {
                    overflow: "clip",
                    height: "100%",
                    width: "5%",
                    // main_align: "center",
                    // cross_align: "center",
                    Button {
                        onclick: query_onclick,
                        label {
                            "查询"
                        }
                    }
                }
                rect {
                    overflow: "clip",
                    height: "100%",
                    width: "5%",
                    // main_align: "center",
                    // cross_align: "center",
                    Button {
                        onclick: action_onclick,
                        label {
                            "导出"
                        }
                    }
                }
                rect {
                    overflow: "clip",
                    height: "100%",
                    width: "0.6%",
                    main_align: "center",
                    cross_align: "center",
                    label {
                        width: "100%",
                        // text_align: "center",
                        // cross_align: "center",
                        ""
                    }
                }
                rect {
                    overflow: "clip",
                    height: "100%",
                    width: "4.4%",
                    main_align: "center",
                    cross_align: "center",
                    label {
                        width: "100%",
                        // text_align: "center",
                        // cross_align: "center",
                        "数量：{qty}"
                    }
                }
            }
            rect {
                width:"100%",
                height: "95%",
                Table {
                    columns: 9,
                    TableHead {
                        TableRow {
                            for (n, text) in columns.into_iter().enumerate() {
                                TableCell {
                                    key: "{n}",
                                    // order_direction: if *order.read() == order_by { Some(*order_direction.read()) } else { None },
                                    // onclick: move |_| on_column_head_click(&order_by),
                                    label {
                                        width: "100%",
                                        text_align: "center",
                                        "{text}"
                                    }
                                }
                            }
                        }
                    }
                    TableBody {
                        ScrollView {
                            for (i, item) in result.read().iter().enumerate() {
                                TableRow {
                                    key: "{i}",
                                    alternate_colors: i % 2 == 0,
                                        TableCell {
                                            key: "{0}",
                                            label {
                                                width: "100%",
                                                text_align: "center",
                                                "{item.box_no}"
                                            }
                                        }
                                        TableCell {
                                            key: "{1}",
                                            label {
                                                width: "100%",
                                                text_align: "center",
                                                "{item.sn}"
                                            }
                                        }
                                        TableCell {
                                            key: "{2}",
                                            label {
                                                width: "100%",
                                                text_align: "center",
                                                "{item.ith}"
                                            }
                                        }
                                        TableCell {
                                            key: "{3}",
                                            label {
                                                width: "100%",
                                                text_align: "center",
                                                "{item.pf}"
                                            }
                                        }
                                        TableCell {
                                            key: "{4}",
                                            label {
                                                width: "100%",
                                                text_align: "center",
                                                "{item.se}"
                                            }
                                        }

                                        TableCell {
                                            key: "{5}",
                                            label {
                                                width: "100%",
                                                text_align: "center",
                                                "{item.sen}"
                                            }
                                        }

                                        TableCell {
                                            key: "{6}",
                                            label {
                                                width: "100%",
                                                text_align: "center",
                                                "{item.res}"
                                            }
                                        }

                                        TableCell {
                                            key: "{7}",
                                            label {
                                                width: "100%",
                                                text_align: "center",
                                                "{item.icc}"
                                            }
                                        }
                                        TableCell {
                                            key: "{8}",
                                            label {
                                                width: "100%",
                                                text_align: "center",
                                                "{item.testtime}"
                                            }
                                        }
                                    // for (n, item) in items.iter().enumerate() {
                                    //     TableCell {
                                    //         key: "{n}",
                                    //         label {
                                    //             width: "100%",
                                    //             text_align: "center",
                                    //             "{item}"
                                    //         }
                                    //     }
                                    // }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    )
}
