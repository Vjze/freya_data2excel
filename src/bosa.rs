use crate::utils::band_data_sql_client;

#[derive(Debug)]
struct Info {
    box_no: String,
    sn: String,
}
#[derive(Debug, Clone, Default)]
pub struct Data {
    pub box_no: String,
    pub sn: String,
    pub ith: String,
    pub vf: String,
    pub im: String,
    pub pf: String,
    pub rs: String,
    pub se: String,
    pub ldkink: String,
    pub mpdkink: String,
    pub mpdidark: String,
    pub sen: String,
    pub vbr: String,
    pub res: String,
    pub satuaration: String,
    pub icc: String,
    pub isolation: String,
    pub pdidark: String,
    pub testtime: String,
}
pub async fn get_boxno(c: String) -> Option<Vec<Data>> {
    let mut boxnos: Vec<String> = vec![];
    let mut sns: Vec<Info> = vec![];
    let mut row_data = vec![];
    let mut client = band_data_sql_client().await;
    let stream = client
        .query(
            format!(
                "SELECT box_no FROM carton_band where carton_no = '{}' and status = '0'",
                c
            ),
            &[&1i32],
        )
        .await
        .unwrap();
    let rows = stream.into_results().await;
    match rows {
        Ok(rowsets) => {
            for i in 0..rowsets.len() {
                let rows = rowsets.get(i).unwrap();
                for row in rows {
                    let box_no = row.get::<&str, _>(0).unwrap().to_string();
                    boxnos.push(box_no)
                }
            }
        }
        Err(_) => return None,
    }
    for boxno in boxnos {
        let stream = client
            .query(
                format!(
                    "SELECT sn FROM box_band where box_no = '{}' and status = '0'",
                    boxno.clone()
                ),
                &[&1i32],
            )
            .await
            .unwrap();
        let rows = stream.into_results().await;
        match rows {
            Ok(rowsets) => {
                for i in 0..rowsets.len() {
                    let rows = rowsets.get(i).unwrap();
                    for row in rows {
                        let sn = row.get::<&str, _>(0).unwrap().to_string();
                        let info = Info {
                            box_no: boxno.clone(),
                            sn,
                        };
                        sns.push(info)
                    }
                }
            }
            Err(_) => return None,
        }
    }

    // let mut infos = vec![];
    let mut client = band_data_sql_client().await;

    for info in sns {
        let sql_col =
            "SN,ith,vop,im,pf,rs,se,Kink,ikink,MDPId,sen,vbr,res,icc,idark,IXtalk,BandTime";
        let sql_msg = format!(
            "SELECT top 1 {} FROM sn_band_all where sn = '{}' and status = '0' order by ID desc",
            sql_col, info.sn
        );
        // println!("{}", sql_msg);
        let stream = client.query(sql_msg, &[&1i32]).await.unwrap();
        let rowss = stream.into_row().await.unwrap();
        match rowss {
            Some(row) => {
                // for i in 0..rowsets.len() {
                //     let rows = rowsets.get(i).unwrap();
                //     for row in rows {
                // let mut infos = vec![];
                let sn = row.get::<&str, _>(0).unwrap().to_string();
                let ith = row.get::<&str, _>(1).unwrap().to_string();
                let vf = row.get::<&str, _>(2).unwrap().to_string();
                let im = row.get::<&str, _>(3).unwrap().to_string();
                let pf = row.get::<&str, _>(4).unwrap().to_string();
                let rs = row.get::<&str, _>(5).unwrap().to_string();
                let se = row.get::<&str, _>(6).unwrap().to_string();
                let ldkink = row.get::<&str, _>(7).unwrap().to_string();
                let mpdkink = row.get::<&str, _>(8).unwrap().to_string();
                let mpdidark = row.get::<&str, _>(9).unwrap().to_string();
                let sen = row.get::<&str, _>(10).unwrap().to_string();
                let vbr = if row.get::<&str, _>(11).unwrap().to_string().is_empty(){
                    "0.00".to_string()
                }else{
                    row.get::<&str, _>(11).unwrap().to_string()
                };
                let res = row.get::<&str, _>(12).unwrap().to_string();
                let icc = row.get::<&str, _>(13).unwrap().to_string();
                let idark = row.get::<&str, _>(14).unwrap().to_string();
                let satuaration = "-3";
                let isolation = row.get::<&str, _>(15).unwrap().to_string();
                let testtime = row
                    .get::<chrono::NaiveDateTime, _>(16)
                    .unwrap()
                    .format("%Y/%m/%d %H:%M:%S").to_string();
                let infos = Data {
                    box_no: info.box_no,
                    sn,
                    ith,
                    vf,
                    im,
                    pf,
                    rs,
                    se,
                    ldkink,
                    mpdkink,
                    mpdidark,
                    sen,
                    vbr,
                    res,
                    satuaration: satuaration.to_string(),
                    icc,
                    isolation,
                    pdidark: idark,
                    testtime,
                };
                // infos.push(c.clone());
                // infos.push(info.box_no);
                // infos.push(sn);
                // infos.push(po);
                // infos.push(ith);
                // infos.push(sen);
                // infos.push(res);
                row_data.push(infos);
            }
            //     }
            // }
            None => return None,
        }
    }
    if row_data.is_empty(){
        None
    }else {
        Some(row_data)
    }
    
}
