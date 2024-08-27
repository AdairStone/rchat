use std::{time::{SystemTime, UNIX_EPOCH}, sync::{Arc, RwLock}};

use chrono::{Utc, FixedOffset, DateTime, Timelike, NaiveDate, Datelike, TimeZone, Local, NaiveDateTime};
use once_cell::sync::Lazy;
use serde::Serializer;

use crate::app_config::SETTINGS;


pub fn current_ms()-> u128 { 
    SystemTime::now().duration_since(UNIX_EPOCH).expect("获取系统时间失败").as_millis()
}

pub fn current_ns()-> u128 { 
    SystemTime::now().duration_since(UNIX_EPOCH).expect("获取系统时间失败").as_nanos()
}

pub fn current_s()-> u64 { 
    SystemTime::now().duration_since(UNIX_EPOCH).expect("获取系统时间失败").as_secs()
}

pub fn current_date_ymd() -> String {
    Utc::now().with_timezone(&FixedOffset::east_opt(SETTINGS.time_zone*3600).unwrap()).format("%Y-%m-%d").to_string()
}

pub fn current_date() -> DateTime<FixedOffset> {
    let r = Utc::now().with_timezone(&FixedOffset::east_opt(SETTINGS.time_zone*3600).unwrap());
    return r;
}

pub fn date_ymdhms(date: DateTime<FixedOffset>) -> String {
    let fmt = "%Y-%m-%d %H:%M:%S";
    return date.format(fmt).to_string();
}

pub fn format_date_ymdhms(date: DateTime<FixedOffset>) -> String {
    let fmt = "%Y-%m-%d %H:%M:%S%.3f";
    return date.format(fmt).to_string();
}

pub fn format_date_ymd(date: DateTime<FixedOffset>) -> String {
    let fmt = "%Y-%m-%d";
    return date.format(fmt).to_string();
}

pub fn with_current_date(time: &str) -> anyhow::Result<DateTime<FixedOffset>> {
    let fmthour = "%Y-%m-%d %H:%M:%S%.3f%z";
    let date = current_date_ymd();
    let d = format!("{} {}+08:00", date, time);
    return Ok(DateTime::parse_from_str(d.as_str(), fmthour)?);
}

pub fn to_date(date_time: &str) -> anyhow::Result<DateTime<FixedOffset>> {
    let fmthour = "%Y-%m-%d %H:%M:%S%.3f%z";
    let d = format!("{}+08:00", date_time);
    return Ok(DateTime::parse_from_str(d.as_str(), fmthour)?);
}

pub fn to_date_with_ymd(date_time: &str) -> anyhow::Result<DateTime<FixedOffset>> {
    let fmthour = "%Y-%m-%d %H:%M:%S%.3f%z";
    let d = format!("{} 00:00:00.000+08:00", date_time);
    return Ok(DateTime::parse_from_str(d.as_str(), fmthour)?);
}

pub fn timespace_2m(time: DateTime<FixedOffset>) -> (String, String){
    let minute = time.minute();
    let hour = time.hour();
    let mut start_minute = 0;
    let mut start_hour = 0;
    let mut end_minute = 0;
    let mut end_hour = 0;
    let mval = minute / 2;
    if minute >= 58 {
        start_hour = hour;
        start_minute = mval * 2;
        end_minute = 0;
        end_hour = hour + 1;
    } else {
        start_hour = hour;
        start_minute = mval * 2;
        end_minute = mval * 2 + 2;
        end_hour = hour;
    }
    let start_time = FixedOffset::east_opt(8*3600).unwrap().from_local_datetime(&NaiveDate::from_ymd_opt(time.year(), time.month(), time.day()).unwrap().and_hms_opt(start_hour, start_minute, 0).unwrap()).unwrap();
    let end_time =FixedOffset::east_opt(8*3600).unwrap().from_local_datetime(&NaiveDate::from_ymd_opt(time.year(), time.month(), time.day()).unwrap().and_hms_opt(end_hour, end_minute, 0).unwrap()).unwrap();

    (format_date_ymdhms(start_time),format_date_ymdhms(end_time))
}

pub fn timespace_5m(time: DateTime<FixedOffset>) -> (String, String){
    let minute = time.minute();
    let hour = time.hour();
    let mut start_minute = 0;
    let mut start_hour = 0;
    let mut end_minute = 0;
    let mut end_hour = 0;
    let mval = minute / 5;
    if minute >= 55 {
        start_hour = hour;
        start_minute = mval * 5;
        end_minute = 0;
        end_hour = hour + 1;
    } else {
        start_hour = hour;
        start_minute = mval * 5;
        end_minute = mval * 5 + 5;
        end_hour = hour;
    }
    let start_time = FixedOffset::east_opt(8*3600).unwrap().from_local_datetime(&NaiveDate::from_ymd_opt(time.year(), time.month(), time.day()).unwrap().and_hms_opt(start_hour, start_minute, 0).unwrap()).unwrap();
    let end_time =FixedOffset::east_opt(8*3600).unwrap().from_local_datetime(&NaiveDate::from_ymd_opt(time.year(), time.month(), time.day()).unwrap().and_hms_opt(end_hour, end_minute, 0).unwrap()).unwrap();

    (format_date_ymdhms(start_time),format_date_ymdhms(end_time))
}

pub fn runed(time: DateTime<FixedOffset>) -> u32 {
    let hour = time.hour();
    let minute: u32 = time.minute();
    let mut runed = 0;
    if hour==9 && minute >=30 {
        runed = minute - 30;
    } else if hour == 10 {
        runed = 30 + minute;
    } else if hour ==11&& minute <=30 {
        runed = 90 + minute;
    } else if (hour == 11 && minute >=30 )|| (hour == 12) {
        runed = 120;
    } else if hour == 13 {
        runed = 120 + minute;
    } else if hour == 14 {
        runed = 180 + minute;
    } else if hour >=15 {
        runed = 240;
    }
    runed
}

pub fn time_with_ymdhms(y:i32, m:u32, d:u32, hh:u32,mm:u32, ss:u32)-> DateTime<FixedOffset>{
    let dt1: chrono::NaiveDateTime = NaiveDate::from_ymd_opt(y, m, d).unwrap().and_hms_opt(hh, mm, ss).unwrap();
    let time: chrono::DateTime<FixedOffset> = FixedOffset::east_opt(8*3600).unwrap().from_local_datetime(&dt1).unwrap();
    time
}

//"%Y-%m-%d %H:%M:%S"
pub fn parse_string_to_datetime(s: &str, format: &str) -> anyhow::Result<DateTime<Local>> {
    let naive_datetime = NaiveDateTime::parse_from_str(s, format)?;
    let local_datetime = Local.from_local_datetime(&naive_datetime).earliest();
    Ok(local_datetime.unwrap())
}

pub fn local_time_to_offset(time: &DateTime<Local>)-> DateTime<FixedOffset> {
    time.with_timezone(&FixedOffset::east_opt(8*3600).unwrap())
}


pub fn serialize_datetime_with_timezone<S>(date: &zino::prelude::DateTime, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    // 定义项目中的指定时区，例如东八区 +08:00
    
    let timezone = FixedOffset::east_opt(SETTINGS.time_zone * 3600).unwrap();

    // // 将日期转换为指定时区
    // let datetime_with_tz = date.with_timezone(&timezone);
    // date.0.with_timezone(datetime_with_tz)
    let s = date.format_with_zone(&timezone);
    // 以指定格式进行序列化，例如 RFC3339
    // let s = datetime_with_tz.to_rfc3339();
    serializer.serialize_str(&s)
}


mod test {
    use chrono::{Utc, Local, FixedOffset, NaiveDate, TimeZone, Datelike, Timelike};
    use crate::utils::date_utils::{timespace_2m, current_date, runed, to_date_with_ymd, timespace_5m, parse_string_to_datetime};

    // #[tokio::test]
    // #[test]
    async fn test_time() {
        let dt = current_date();
        let dt1: chrono::NaiveDateTime = NaiveDate::from_ymd_opt(dt.year(), dt.month(), dt.day()).unwrap().and_hms_opt(14, 11, 00).unwrap();
        let time: chrono::DateTime<FixedOffset> = FixedOffset::east_opt(8*3600).unwrap().from_local_datetime(&dt1).unwrap();
        println!("timespace2:{:?}", timespace_2m(time));
        println!("timespace5:{:?}", timespace_5m(time));
        println!("runed:{:?}", runed(time));
    }

    
    #[test]
    // #[tokio::test]
    fn test_to_date_with_ymd() {
        println!("{:?}", to_date_with_ymd("2023-08-08"));
        println!("{:?}", parse_string_to_datetime("2023-08-08 00:00:00", "%Y-%m-%d %H:%M:%S"));
        
    }
}