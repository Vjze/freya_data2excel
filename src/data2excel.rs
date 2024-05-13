use chrono::Local;
use rust_xlsxwriter::{Color, Format, FormatAlign, FormatBorder, Workbook};

use crate::{bosa::Data, check_pn_type::check_pn_type, utils::band_data_sql_client};
pub enum Tip {
    Ok,
    Err{e:String}
} 
pub async fn do_action(datas: Vec<Data>,carton: String) -> Result<Tip, Tip>{
    let t = check_pn_type(carton.clone()).await;
    if t.gs == "ZJ" {
        universal_file_out_zj(datas, carton).await
    }else {
        universal_file_out(datas, carton).await
    }
}
pub async fn rosa_universal_file_out(_datas: Vec<Data>) -> tokio::io::Result<()> {
    let mut workbook = Workbook::new();
    let worksheet = workbook.add_worksheet();
    // worksheet.set_column_width_pixels(0, 120).unwrap();
    // worksheet.set_column_width_pixels(1, 100).unwrap();
    // worksheet.set_column_width_pixels(2, 60).unwrap();
    // worksheet.set_column_width_pixels(3, 60).unwrap();
    // worksheet.set_column_width_pixels(4, 60).unwrap();
    // worksheet.set_column_width_pixels(5, 60).unwrap();
    // worksheet.set_column_width_pixels(6, 60).unwrap();
    // worksheet.set_column_width_pixels(7, 90).unwrap();
    // worksheet.set_column_width_pixels(8, 70).unwrap();
    // worksheet.set_column_width_pixels(9, 80).unwrap();
    // worksheet.set_column_width_pixels(10, 100).unwrap();
    // worksheet.set_column_width_pixels(11, 80).unwrap();
    // worksheet.set_column_width_pixels(12, 60).unwrap();
    // worksheet.set_column_width_pixels(13, 115).unwrap();
    // worksheet.set_column_width_pixels(14, 120).unwrap();
    // worksheet.set_column_width_pixels(15, 95).unwrap();
    // worksheet.set_column_width_pixels(16,115).unwrap();
    // worksheet.set_column_width_pixels(17, 60).unwrap();
    // worksheet.set_column_width_pixels(18, 100).unwrap();
    // worksheet.set_column_width_pixels(19, 80).unwrap();
    worksheet.set_row_height(0, 52).unwrap();
    // let bt_format = Format::new()
    //     .set_align(FormatAlign::Center)
    //     .set_align(FormatAlign::VerticalCenter)
    //     .set_font_size(13)
    //     .set_border(FormatBorder::Thin)
    //     .set_background_color(Color::RGB(0x99CC00))
    //     .set_text_wrap()
    //     .set_bold();
    // let res_format = Format::new()
    //     .set_align(FormatAlign::Center)
    //     .set_align(FormatAlign::VerticalCenter)
    //     .set_font_size(13)
    //     .set_border(FormatBorder::Thin)
    //     .set_background_color(Color::RGB(0xFFFF00))
    //     .set_text_wrap()
    //     .set_bold();
    let str_format = Format::new()
        .set_align(FormatAlign::Center)
        .set_align(FormatAlign::VerticalCenter)
        .set_text_wrap()
        .set_border(FormatBorder::Thin);
    // worksheet.write_string_with_format(0, 0, "No", &bt_format).unwrap();
    // worksheet.write_string_with_format(1, 0, "Unit", &bt_format).unwrap();
    // worksheet.write_string_with_format(2, 0, "Low limit", &bt_format).unwrap();
    // worksheet.write_string_with_format(3, 0, "high limit", &bt_format).unwrap();
    // worksheet.merge_range(0, 1, 3, 1, "外箱号", &bt_format).unwrap();
    // worksheet.merge_range(0, 2, 3, 2, "内箱号", &bt_format).unwrap();
    // worksheet.merge_range(0, 3, 3, 3, "盒号", &bt_format).unwrap();
    // worksheet.merge_range(0, 4, 3, 4, "RXTO批次", &bt_format).unwrap();
    // worksheet.merge_range(0, 5, 3, 5, "S/N", &bt_format).unwrap();
    // worksheet.write_string_with_format(0, 6, "Icc", &bt_format).unwrap();
    // worksheet.write_string_with_format(0, 7, "Rssi-dark", &bt_format).unwrap();
    // worksheet.write_string_with_format(0, 8, "Responsivity", &bt_format).unwrap();
    // worksheet.write_string_with_format(0, 9, "URS(Avg)", &bt_format).unwrap();
    // worksheet.write_string_with_format(1, 6, "mA", &bt_format).unwrap();
    // worksheet.write_string_with_format(1, 7,"nA", &bt_format).unwrap();
    // worksheet.write_string_with_format(1, 8,"uA", &bt_format).unwrap();
    // worksheet.write_string_with_format(1, 9, "dBm", &bt_format).unwrap();
    // worksheet.write_string_with_format(2, 6, "28", &bt_format).unwrap();
    // worksheet.write_string_with_format(2, 7,"", &bt_format).unwrap();
    // worksheet.write_string_with_format(2, 8,"0.81", &bt_format).unwrap();
    // worksheet.write_string_with_format(2, 9, "-34.9", &bt_format).unwrap();
    // worksheet.write_string_with_format(3, 6, "36", &bt_format).unwrap();
    // worksheet.write_string_with_format(3, 7,"20", &bt_format).unwrap();
    // worksheet.write_string_with_format(3, 8,"1.19", &bt_format).unwrap();
    // worksheet.write_string_with_format(3, 9, "-21.6", &bt_format).unwrap();

    // for (n,data) in datas.iter().enumerate() {
    //     let x: u32 = n.try_into().unwrap();
    //     for (c,d) in data.iter().enumerate() {
    //         let y: u16 = c.try_into().unwrap();
    //         worksheet.write_string_with_format(x, y, d.to_string(), &str_format).unwrap();
    //     }

    //     // worksheet.write_string_with_format(x, 1, data.carton_no.clone(), &str_format).unwrap();
    //     // worksheet.write_string_with_format(x, 2, data.inside_carton_no.clone(), &str_format).unwrap();
    //     // worksheet.write_string_with_format(x, 3, data.box_no.clone(), &str_format).unwrap();
    //     // worksheet.write_string_with_format(x, 4, "", &str_format).unwrap();
    //     // worksheet.write_string_with_format(x, 5, data.sn.clone(), &str_format).unwrap();
    //     // worksheet.write_string_with_format(x, 6, data.icc.clone(), &str_format).unwrap();
    //     // worksheet.write_string_with_format(x, 7, data.pdidark.clone(), &str_format).unwrap();
    //     // worksheet.write_string_with_format(x, 8, data.res.clone(), &str_format).unwrap();
    //     // worksheet.write_string_with_format(x, 9, data.sen.clone(), &str_format).unwrap();
    // }

    workbook.save("rust_perf_test.xlsx").unwrap();

    Ok(())
}
pub async fn universal_file_out(datas: Vec<Data>, carton: String) -> Result<Tip,Tip>{
    let mut workbook = Workbook::new();
    let worksheet = workbook.add_worksheet();
    worksheet.set_column_width_pixels(0, 120).unwrap();
    worksheet.set_column_width_pixels(1, 100).unwrap();
    worksheet.set_column_width_pixels(2, 60).unwrap();
    worksheet.set_column_width_pixels(3, 60).unwrap();
    worksheet.set_column_width_pixels(4, 60).unwrap();
    worksheet.set_column_width_pixels(5, 60).unwrap();
    worksheet.set_column_width_pixels(6, 60).unwrap();
    worksheet.set_column_width_pixels(7, 90).unwrap();
    worksheet.set_column_width_pixels(8, 70).unwrap();
    worksheet.set_column_width_pixels(9, 80).unwrap();
    worksheet.set_column_width_pixels(10, 100).unwrap();
    worksheet.set_column_width_pixels(11, 80).unwrap();
    worksheet.set_column_width_pixels(12, 60).unwrap();
    worksheet.set_column_width_pixels(13, 115).unwrap();
    worksheet.set_column_width_pixels(14, 120).unwrap();
    worksheet.set_column_width_pixels(15, 95).unwrap();
    worksheet.set_column_width_pixels(16, 115).unwrap();
    worksheet.set_column_width_pixels(17, 60).unwrap();
    worksheet.set_column_width_pixels(18, 100).unwrap();
    worksheet.set_column_width_pixels(19, 80).unwrap();
    worksheet.set_row_height(0, 52).unwrap();
    let bt_format = Format::new()
        .set_align(FormatAlign::Center)
        .set_align(FormatAlign::VerticalCenter)
        .set_font_size(13)
        .set_border(FormatBorder::Thin)
        .set_background_color(Color::RGB(0x99CC00))
        .set_text_wrap()
        .set_bold();
    let res_format = Format::new()
        .set_align(FormatAlign::Center)
        .set_align(FormatAlign::VerticalCenter)
        .set_font_size(13)
        .set_border(FormatBorder::Thin)
        .set_background_color(Color::RGB(0xFFFF00))
        .set_text_wrap()
        .set_bold();
    let str_format = Format::new()
        .set_align(FormatAlign::Center)
        .set_align(FormatAlign::VerticalCenter)
        .set_text_wrap()
        .set_border(FormatBorder::Thin);
    worksheet
        .write_string_with_format(0, 0, "Box no", &bt_format)
        .unwrap();
    worksheet
        .write_string_with_format(0, 1, "SN", &bt_format)
        .unwrap();
    worksheet
        .write_string_with_format(0, 2, "Ith （mA）", &bt_format)
        .unwrap();
    worksheet
        .write_string_with_format(0, 3, "Vf    （V）", &bt_format)
        .unwrap();
    worksheet
        .write_string_with_format(0, 4, "Im （uA）", &bt_format)
        .unwrap();
    worksheet
        .write_string_with_format(0, 5, "Pf （uW）", &bt_format)
        .unwrap();
    worksheet
        .write_string_with_format(0, 6, "Rs （Ω）", &bt_format)
        .unwrap();
    worksheet
        .write_string_with_format(0, 7, "SE （mW/mA）", &bt_format)
        .unwrap();
    worksheet
        .write_string_with_format(0, 8, "LD_Kink", &bt_format)
        .unwrap();
    worksheet
        .write_string_with_format(0, 9, "MPD_Kink", &bt_format)
        .unwrap();
    worksheet
        .write_string_with_format(0, 10, "MPD_Idark （nA）", &bt_format)
        .unwrap();
    worksheet
        .write_string_with_format(0, 11, "Sen （dBm）", &bt_format)
        .unwrap();
    worksheet
        .write_string_with_format(0, 12, "Vbr （V）", &res_format)
        .unwrap();
    worksheet
        .write_string_with_format(0, 13, "Responsive_H(APD-高压) （A/W）", &res_format)
        .unwrap();
    worksheet
        .write_string_with_format(0, 14, "Responsive_L（PIN）（A/W）", &res_format)
        .unwrap();
    worksheet
        .write_string_with_format(0, 15, "Responsive", &bt_format)
        .unwrap();
    worksheet
        .write_string_with_format(0, 16, "Satuaration （dBm）", &bt_format)
        .unwrap();
    worksheet
        .write_string_with_format(0, 17, "ICC （mA）", &bt_format)
        .unwrap();
    worksheet
        .write_string_with_format(0, 18, "Isolation （dB）", &bt_format)
        .unwrap();
    worksheet
        .write_string_with_format(0, 19, "PD_Idark （nA）", &bt_format)
        .unwrap();
    for (n, data) in datas.iter().enumerate() {
        let x: u32 = n.try_into().unwrap();
        // for (c,d) in data.iter().enumerate(){
        //     let y: u16 = c.try_into().unwrap();

        //     worksheet.write_string_with_format(x + 1, y, d, &str_format).unwrap();
        // }
        worksheet.write_string_with_format(x + 1, 0, data.box_no.clone(), &str_format).unwrap();
        worksheet
            .write_string_with_format(x + 1, 1, data.sn.clone(), &str_format)
            .unwrap();
        worksheet
            .write_string_with_format(x + 1, 2, data.ith.clone(), &str_format)
            .unwrap();
        worksheet
            .write_string_with_format(x + 1, 3, data.vf.clone(), &str_format)
            .unwrap();
        worksheet
            .write_string_with_format(x + 1, 4, data.im.clone(), &str_format)
            .unwrap();
        worksheet
            .write_string_with_format(x + 1, 5, data.pf.clone(), &str_format)
            .unwrap();
        worksheet
            .write_string_with_format(x + 1, 6, data.rs.clone(), &str_format)
            .unwrap();
        worksheet
            .write_string_with_format(x + 1, 7, data.se.clone(), &str_format)
            .unwrap();
        worksheet
            .write_string_with_format(x + 1, 8, data.ldkink.clone(), &str_format)
            .unwrap();
        worksheet
            .write_string_with_format(x + 1, 9, data.mpdkink.clone(), &str_format)
            .unwrap();
        worksheet
            .write_string_with_format(x + 1, 10, data.mpdidark.clone(), &str_format)
            .unwrap();
        worksheet
            .write_string_with_format(x + 1, 11, data.sen.clone(), &str_format)
            .unwrap();
        worksheet
            .write_string_with_format(x + 1, 12, data.vbr.clone(), &str_format)
            .unwrap();
        worksheet
            .write_string_with_format(x + 1, 13, "", &str_format)
            .unwrap();
        worksheet
            .write_string_with_format(x + 1, 14, "", &str_format)
            .unwrap();
        worksheet
            .write_string_with_format(x + 1, 15, data.res.clone(), &str_format)
            .unwrap();
        worksheet
            .write_string_with_format(x + 1, 16, data.satuaration.clone(), &str_format)
            .unwrap();
        worksheet
            .write_string_with_format(x + 1, 17, data.icc.clone(), &str_format)
            .unwrap();
        worksheet
            .write_string_with_format(x + 1, 18, data.isolation.clone(), &str_format)
            .unwrap();
        worksheet
            .write_string_with_format(x + 1, 19, data.pdidark.clone(), &str_format)
            .unwrap();
    }
    let fmt = "%y%m%d";
    let now = Local::now().format(fmt).to_string();
    let file_name = format!("{}-{}-{}pcs.xlsx", carton, now, datas.len());
    // workbook.save(file_name).unwrap();
    match workbook.save(file_name) {
        Ok(_) => Ok(Tip::Ok),
        Err(e) => {
                    Err(Tip::Err{e:e.to_string()})
        },
    }
}

pub async fn universal_file_out_zj(datas: Vec<Data>, carton: String) -> Result<Tip,Tip> {
    let mut client = band_data_sql_client().await;
    let sql_msg = format!("SELECT pn FROM carton_band where carton_no = '{}'", carton);
    let stream = client.query(sql_msg, &[&1i32]).await.unwrap();
    let rowss = stream.into_row().await.unwrap().unwrap();
    let pn_msg = format!(
        "SELECT cpn FROM band_config where pn = '{}'",
        rowss.get::<&str, _>(0).unwrap().to_string()
    );
    let pn_stream = client.query(pn_msg, &[&1i32]).await.unwrap();
    let row = pn_stream.into_row().await.unwrap().unwrap();
    let pn = row.get::<&str, _>(0).unwrap().to_string();
    let mut workbook = Workbook::new();
    let worksheet = workbook.add_worksheet();
    worksheet.set_column_width_pixels(0, 200).unwrap();
    worksheet.set_column_width_pixels(1, 120).unwrap();
    worksheet.set_column_width_pixels(2, 160).unwrap();
    worksheet.set_column_width_pixels(3, 60).unwrap();
    worksheet.set_column_width_pixels(4, 60).unwrap();
    worksheet.set_column_width_pixels(5, 60).unwrap();
    worksheet.set_column_width_pixels(6, 60).unwrap();
    worksheet.set_column_width_pixels(7, 60).unwrap();
    worksheet.set_column_width_pixels(8, 60).unwrap();
    worksheet.set_column_width_pixels(9, 60).unwrap();
    worksheet.set_column_width_pixels(10, 100).unwrap();
    worksheet.set_column_width_pixels(11, 100).unwrap();
    worksheet.set_column_width_pixels(12, 120).unwrap();
    worksheet.set_column_width_pixels(13, 100).unwrap();
    worksheet.set_row_height(0, 12).unwrap();
    let bt_format = Format::new()
        .set_align(FormatAlign::Center)
        .set_align(FormatAlign::VerticalCenter)
        .set_font_size(9)
        .set_border(FormatBorder::Thin)
        .set_background_color(Color::RGB(0xCCFFFF))
        .set_text_wrap()
        .set_bold();
    // let res_format = Format::new()
    //     .set_align(FormatAlign::Center)
    //     .set_align(FormatAlign::VerticalCenter)
    //     .set_font_size(13)
    //     .set_border(FormatBorder::Thin)
    //     .set_background_color(Color::RGB(0xFFFF00))
    //     .set_text_wrap()
    //     .set_bold();
    let str_format = Format::new()
        .set_align(FormatAlign::Center)
        .set_align(FormatAlign::VerticalCenter)
        .set_text_wrap()
        .set_border(FormatBorder::Thin);
    worksheet
        .write_string_with_format(0, 0, "PN", &bt_format)
        .unwrap();
    worksheet
        .write_string_with_format(0, 1, "SN", &bt_format)
        .unwrap();
    worksheet
        .write_string_with_format(0, 2, "TestDateTime", &bt_format)
        .unwrap();
    worksheet
        .write_string_with_format(0, 3, "Ith", &bt_format)
        .unwrap();
    worksheet
        .write_string_with_format(0, 4, "Po", &bt_format)
        .unwrap();
    worksheet
        .write_string_with_format(0, 5, "IMD", &bt_format)
        .unwrap();
    worksheet
        .write_string_with_format(0, 6, "Vf", &bt_format)
        .unwrap();
    worksheet
        .write_string_with_format(0, 7, "Rs", &bt_format)
        .unwrap();
    worksheet
        .write_string_with_format(0, 8, "Kink", &bt_format)
        .unwrap();
    worksheet
        .write_string_with_format(0, 9, "Vbr", &bt_format)
        .unwrap();
    worksheet
        .write_string_with_format(0, 10, "Sensitivity(卡)", &bt_format)
        .unwrap();
    worksheet
        .write_string_with_format(0, 11, "Responsive", &bt_format)
        .unwrap();
    worksheet
        .write_string_with_format(0, 12, "Paketnum1", &bt_format)
        .unwrap();
    worksheet
        .write_string_with_format(0, 13, "TestDate", &bt_format)
        .unwrap();
    // for (n,data) in datas.iter().enumerate() {
    //     let x: u32 = n.try_into().unwrap();
    //     for (c,d) in data.iter().enumerate(){
    //         let y: u16 = c.try_into().unwrap();

    //         worksheet.write_string_with_format(x + 1, y, d, &str_format).unwrap();
    //     }
    for (n, data) in datas.iter().enumerate() {
        let x: u32 = n.try_into().unwrap();
        worksheet
            .write_string_with_format(x + 1, 0, pn.clone(), &str_format)
            .unwrap();
        worksheet
            .write_string_with_format(x + 1, 1, data.sn.clone(), &str_format)
            .unwrap();
        worksheet
            .write_string_with_format(x + 1, 2, data.testtime.clone(), &str_format)
            .unwrap();
        worksheet
            .write_string_with_format(x + 1, 3, data.ith.clone(), &str_format)
            .unwrap();
        worksheet
            .write_string_with_format(x + 1, 4, data.pf.clone(), &str_format)
            .unwrap();
        worksheet
            .write_string_with_format(x + 1, 5, data.im.clone(), &str_format)
            .unwrap();
        worksheet
            .write_string_with_format(x + 1, 6, data.vf.clone(), &str_format)
            .unwrap();
        worksheet
            .write_string_with_format(x + 1, 7, data.rs.clone(), &str_format)
            .unwrap();
        worksheet
            .write_string_with_format(x + 1, 8, data.ldkink.clone(), &str_format)
            .unwrap();
        worksheet
            .write_string_with_format(x + 1, 9, data.vbr.clone(), &str_format)
            .unwrap();
        worksheet
            .write_string_with_format(x + 1, 10, data.sen.clone(), &str_format)
            .unwrap();
        worksheet
            .write_string_with_format(x + 1, 11, data.res.clone(), &str_format)
            .unwrap();
        worksheet
            .write_string_with_format(x + 1, 12, data.box_no.clone(), &str_format)
            .unwrap();
        worksheet
            .write_string_with_format(x + 1, 13, "0", &str_format)
            .unwrap();
        // worksheet.write_string_with_format(x + 1, 14, data.resl.clone(), &str_format).unwrap();
        // worksheet.write_string_with_format(x + 1, 15, data.res.clone(), &str_format).unwrap();
        // worksheet.write_string_with_format(x + 1, 16, data.satuaration.clone(), &str_format).unwrap();
        // worksheet.write_string_with_format(x + 1, 17, data.icc.clone(), &str_format).unwrap();
        // worksheet.write_string_with_format(x + 1, 18, data.isolation.clone(), &str_format).unwrap();
        // worksheet.write_string_with_format(x + 1, 19, data.pdidark.clone(), &str_format).unwrap();
        // }
    }
    let fmt = "%y%m%d";
    let now = Local::now().format(fmt).to_string();
    let file_name = format!("{}-{}-{}pcs.xlsx", carton, now, datas.len());
    match workbook.save(file_name) {
        Ok(_) => Ok(Tip::Ok),
        Err(e) => {Err(Tip::Err { e: e.to_string() })},
    }

}
