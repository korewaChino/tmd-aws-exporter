// TMD Automatic Weather Station query code
// uses reqwest, json and multipart stuff
//
use serde::Deserialize;
use stable_eyre::Result;

const AWS_ALPHANUMERIC_ENDPOINT: &str = "http://www.aws-observation.tmd.go.th/aws/awsAlphnData";

// exampleslop below
// async fn fetch_aws(client: &reqwest::Client, dt: DateTime<Utc>, station: u32) -> reqwest::Result<String> {
//     let timestamp = dt.format("%Y%m%d%H%M").to_string();
//     let indate = dt.format("%Y/%m/%d").to_string();
//     let hr = dt.format("%H").to_string();
//     let min = dt.format("%M").to_string();

//     client
//         .post("http://www.aws-observation.tmd.go.th/aws/awsAlphnData")
//         .header("ajax-forward", "ajax")
//         .header("X-Requested-With", "XMLHttpRequest")
//         .header("Referer", "http://www.aws-observation.tmd.go.th/aws/awsAlphn")
//         .form(&[
//             ("sdate", timestamp.as_str()),
//             ("fdate", timestamp.as_str()),
//             ("regions", "1"),
//             ("station", &station.to_string()),
//             ("types", "all"),
//             ("indate", indate.as_str()),
//             ("shr", hr.as_str()),
//             ("smin", min.as_str()),
//         ])
//         .send()
//         .await?
//         .text()
//         .await
// }

#[derive(Debug, Deserialize)]
pub struct AwsResponse {
    pub data: AwsData,
    #[serde(rename = "resultMessage")]
    pub result_message: String,
    #[serde(rename = "resultCode")]
    pub result_code: u32,
}

#[derive(Debug, Deserialize)]
pub struct AwsData {
    #[serde(rename = "totCnt")]
    pub total_count: u32,
    pub list: Vec<AwsObservation>,
}

#[derive(Debug, Deserialize)]
pub struct AwsObservation {
    pub id: String,
    pub wmo: String,
    pub sname: String,
    pub lat: String,
    pub lon: String,
    pub alt: String,
    #[serde(rename = "wvSensor")]
    pub wv_sensor: String,
    pub online: String,
    pub regions: String,
    /// UTC timestamp as YYYYMMDDHHmm
    pub sectime: String,

    // wind direction (degrees)
    pub s00a: Option<String>,
    // wind speed (knots)
    pub s01a: Option<String>,
    // temperature (°C)
    pub s02a: Option<String>,
    // daily precipitation accumulator (mm)
    pub s03m: Option<String>,
    // station pressure (hPa)
    pub s04a: Option<String>,
    // sea level pressure QFF (hPa)
    pub s04n: Option<String>,
    // humidity (%)
    pub s05a: Option<String>,
    /// weather code
    pub s06m: Option<String>,
    // visibility (m), 0.0 = null sentinel
    pub s07a: Option<String>,

    // rainfall accumulators
    pub r01m: Option<String>,
    pub r15m: Option<String>,
    pub r30m: Option<String>,
    pub r01h: Option<String>,
    pub r02h: Option<String>,
    pub r03h: Option<String>,
    pub r06h: Option<String>,
    pub r12h: Option<String>,
    pub r24h: Option<String>,

    // unknown fields
    pub _ws: Option<String>,
    pub _wd: Option<String>,
    pub _a: Option<String>,
    pub _b: Option<String>,
    pub _c: Option<String>,
    pub _d: Option<String>,
    pub _e: Option<String>,
    pub _f: Option<String>,
}

impl AwsObservation {
    pub fn wind_dir(&self) -> Option<f64> {
        self.s00a.as_deref()?.parse().ok()
    }
    pub fn wind_speed_knots(&self) -> Option<f64> {
        self.s01a.as_deref()?.parse().ok()
    }
    pub fn temperature(&self) -> Option<f64> {
        self.s02a.as_deref()?.parse().ok()
    }
    pub fn precip_daily(&self) -> Option<f64> {
        self.s03m.as_deref()?.parse().ok()
    }
    pub fn pressure(&self) -> Option<f64> {
        self.s04a.as_deref()?.parse().ok()
    }
    pub fn pressure_sea_level(&self) -> Option<f64> {
        self.s04n.as_deref()?.parse().ok()
    }
    pub fn humidity(&self) -> Option<f64> {
        self.s05a.as_deref()?.parse().ok()
    }
    pub fn visibility(&self) -> Option<f64> {
        let v: f64 = self.s07a.as_deref()?.parse().ok()?;
        if v == 0.0 { None } else { Some(v) }
    }

    pub fn rain_1min(&self) -> Option<f64> {
        self.r01m.as_deref()?.parse().ok()
    }
    pub fn rain_15min(&self) -> Option<f64> {
        self.r15m.as_deref()?.parse().ok()
    }
    pub fn rain_30min(&self) -> Option<f64> {
        self.r30m.as_deref()?.parse().ok()
    }
    pub fn rain_1hour(&self) -> Option<f64> {
        self.r01h.as_deref()?.parse().ok()
    }
    pub fn rain_2hour(&self) -> Option<f64> {
        self.r02h.as_deref()?.parse().ok()
    }
    pub fn rain_3hour(&self) -> Option<f64> {
        self.r03h.as_deref()?.parse().ok()
    }
    pub fn rain_6hour(&self) -> Option<f64> {
        self.r06h.as_deref()?.parse().ok()
    }
    pub fn rain_12hour(&self) -> Option<f64> {
        self.r12h.as_deref()?.parse().ok()
    }
    pub fn rain_24hour(&self) -> Option<f64> {
        self.r24h.as_deref()?.parse().ok()
    }

    pub fn timestamp(&self) -> Option<chrono::DateTime<chrono::Utc>> {
        chrono::NaiveDateTime::parse_from_str(&self.sectime, "%Y%m%d%H%M")
            .ok()
            .map(|dt| dt.and_utc())
    }
}

/// Request headers for AWS API
struct AwsHeaders;

impl AwsHeaders {
    fn build() -> reqwest::header::HeaderMap {
        let mut headers = reqwest::header::HeaderMap::new();

        headers.insert(
            "ajax-forward",
            reqwest::header::HeaderValue::from_static("ajax"),
        );
        headers.insert(
            reqwest::header::HeaderName::from_static("x-requested-with"),
            reqwest::header::HeaderValue::from_static("XMLHttpRequest"),
        );
        headers.insert(
            reqwest::header::REFERER,
            reqwest::header::HeaderValue::from_static(
                "http://www.aws-observation.tmd.go.th/aws/awsAlphn",
            ),
        );

        headers
    }
}

/// Form data for AWS API query
#[derive(Debug, serde::Serialize)]
struct AwsQueryParams {
    /// Start datetime as YYYYMMDDHHmmss
    sdate: String,
    /// End datetime as YYYYMMDDHHmmss
    fdate: String,
    /// Region ID
    regions: String,
    /// Station ID
    station: String,
    /// Data types to fetch
    types: String,
    /// Input date as YYYY/MM/DD
    indate: String,
    /// Hour
    shr: String,
    /// Minute
    smin: String,
}

impl AwsQueryParams {
    fn from_timestamp(timestamp: chrono::DateTime<chrono::Utc>, station: u32) -> Self {
        let datetime_str = timestamp.format("%Y%m%d%H%M00").to_string();

        Self {
            sdate: datetime_str.clone(),
            fdate: datetime_str,
            regions: "1".to_string(),
            station: station.to_string(),
            types: "all".to_string(),
            indate: timestamp.format("%Y/%m/%d").to_string(),
            shr: timestamp.format("%H").to_string(),
            smin: timestamp.format("%M").to_string(),
        }
    }
}

pub struct AwsClient {
    client: reqwest::Client,
    station: u32,
}

impl AwsClient {
    pub fn new(station: u32) -> Self {
        Self {
            client: reqwest::Client::new(),
            station,
        }
    }

    pub async fn get_observation_at(
        &self,
        timestamp: chrono::DateTime<chrono::Utc>,
    ) -> Result<Option<AwsObservation>> {
        let params = AwsQueryParams::from_timestamp(timestamp, self.station);
        let headers = AwsHeaders::build();

        let response = self
            .client
            .post(AWS_ALPHANUMERIC_ENDPOINT)
            .headers(headers)
            .form(&params)
            .send()
            .await?;

        let aws_response: AwsResponse = response.json().await?;

        // Return the first observation if available
        Ok(aws_response.data.list.into_iter().next())
    }

    pub async fn get_observation_now(&self) -> Result<Option<AwsObservation>> {
        let timestamp = chrono::Utc::now();
        self.get_observation_at(timestamp).await
    }
}
