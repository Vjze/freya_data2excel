use std::collections::HashMap;

use crate::{error::MyError, utils::band_data_sql_client};

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
pub async fn get_boxno(c: String) -> anyhow::Result<HashMap<usize, Data>, MyError> {
    let mut row_data = HashMap::new();
    let mut client = band_data_sql_client().await?;
    let stream = client
        .query(
            format!(
                // "SELECT box_no FROM carton_band where carton_no = '{}' and status = '0'",
                "select a.sn,ith,vop,im,pf,rs,se,Kink,ikink,MDPId,sen,vbr,res,icc,idark,IXtalk,BandTime,a.box_no from box_band a inner join carton_band b on a.box_no=b.box_no INNER JOIN sn_band_all c on a.sn=c.SN where b.carton_no='{}' and b.status = '0' order by b.create_date  desc, b.box_no desc, a.box_no asc, c.SN desc",
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
                for (k,row) in rows.into_iter().enumerate() {
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
                    let vbr = if row.get::<&str, _>(11).unwrap().to_string().is_empty() {
                        "0.00".to_string()
                    } else {
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
                        .format("%Y/%m/%d %H:%M:%S")
                        .to_string();
                    let box_no =row.get::<&str, _>(17).unwrap().to_string();
                    let infos = Data {
                        box_no,
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
                    row_data.insert(k, infos);
                }
            }
        }
        Err(_) => return Err(MyError::NoneError),
    }
    
    if row_data.is_empty() {
        Err(MyError::NoneError)
    } else {
        Ok(row_data)
    }
}
