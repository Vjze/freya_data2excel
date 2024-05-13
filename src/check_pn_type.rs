
use crate::{bosa::{get_boxno, Data}, utils::band_data_sql_client};

#[derive(Debug, Clone, Default)]
pub struct PnType{
    pub pn_type: String,
    pub gs :String,
    pub sn: String,
    pub boc: String,
}
pub async fn do_query(sn: String) -> Option<Vec<Data>> {
    let t = check_pn_type(sn.clone()).await;
    if t.pn_type == "Bosa" {
        get_boxno(sn.clone()).await
    }else {
        None
    }
}
pub async fn check_pn_type(sn: String) -> PnType {
    let mut client = band_data_sql_client().await;
    let carton_stream = client
        .query(
            format!(
                "SELECT pn FROM carton_band where carton_no = '{}' and status = '0'",
                sn.clone()
            ),
            &[&1i32],
        )
        .await
        .unwrap();
    let rowss = carton_stream.into_row().await.unwrap();
    match rowss {
        Some(row) => {
            let pn = row.get::<&str, _>(0).unwrap().to_string();
            let stream = client
                .query(
                    format!("SELECT pn_type,gs FROM band_config where pn = '{}'", pn),
                    &[&1i32],
                )
                .await
                .unwrap();
            let pn_row = stream.into_row().await.unwrap().unwrap();
            let pn_type = pn_row.get::<&str, _>(0).unwrap().to_string();
            let gs = pn_row.get::<&str, _>(1).unwrap().to_string();
            let boc = String::from("Carton");
            PnType{
                pn_type,
                gs,
                sn,
                boc,
            }
        }
        None => {
            let box_stream = client
                .query(
                    format!(
                        "SELECT pn FROM box_band where box_no = '{}' and status = '0'",
                        sn.clone()
                    ),
                    &[&1i32],
                )
                .await
                .unwrap();
            let rowss = box_stream.into_row().await.unwrap();
            match rowss {
                Some(row) => {
                    let pn = row.get::<&str, _>(0).unwrap().to_string();
                    let stream = client
                        .query(
                            format!("SELECT pn_type,gs FROM band_config where pn = '{}'", pn),
                            &[&1i32],
                        )
                        .await
                        .unwrap();
                    let pn_row = stream.into_row().await.unwrap().unwrap();
                    let pn_type = pn_row.get::<&str, _>(0).unwrap().to_string();
                    let gs = pn_row.get::<&str, _>(1).unwrap().to_string();
                    let boc = String::from("Box");
                    PnType{
                        pn_type,
                        gs,
                        sn,
                        boc,
                    }
                }
                None => {
                    PnType::default()
                    // let _ = handle.upgrade_in_event_loop(move |ui|{
                    //     ui.set_error("找不到数据!".into());
                    //     ui.invoke_show_confirm_popup();
                    //     });
                    // PnType::default()
                }
            }
        }
    }
}
