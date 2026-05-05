use prometheus_exporter::prometheus::{GaugeVec, Registry, register_gauge_vec_with_registry};
use stable_eyre::eyre::Result;
use std::net::SocketAddr;
use std::sync::MutexGuard;

use crate::aws::AwsObservation;

pub struct PrometheusExporter {
    pub exporter: prometheus_exporter::Exporter,
    pub registry: Registry,

    // Weather metrics
    pub temperature: GaugeVec,
    pub humidity: GaugeVec,
    pub pressure_station: GaugeVec,
    pub pressure_sea_level: GaugeVec,
    pub wind_direction: GaugeVec,
    pub wind_speed: GaugeVec,
    pub visibility: GaugeVec,
    pub precipitation_daily: GaugeVec,

    // Rainfall accumulators
    pub rain_1min: GaugeVec,
    pub rain_15min: GaugeVec,
    pub rain_30min: GaugeVec,
    pub rain_1hour: GaugeVec,
    pub rain_2hour: GaugeVec,
    pub rain_3hour: GaugeVec,
    pub rain_6hour: GaugeVec,
    pub rain_12hour: GaugeVec,
    pub rain_24hour: GaugeVec,

    // Metadata
    pub observation_time: GaugeVec,
}

impl PrometheusExporter {
    pub fn new(addr: SocketAddr) -> Result<Self> {
        // Create a custom registry with tmd_aws prefix
        let registry = Registry::new_custom(Some("tmd_aws".to_string()), None)?;

        // Create weather metrics
        let temperature = register_gauge_vec_with_registry!(
            "temperature_celsius",
            "Air temperature in Celsius",
            &["station_id", "station_name"],
            registry
        )?;

        let humidity = register_gauge_vec_with_registry!(
            "humidity_percent",
            "Relative humidity in percent",
            &["station_id", "station_name"],
            registry
        )?;

        let pressure_station = register_gauge_vec_with_registry!(
            "pressure_station_hpa",
            "Station pressure in hectopascals",
            &["station_id", "station_name"],
            registry
        )?;

        let pressure_sea_level = register_gauge_vec_with_registry!(
            "pressure_sea_level_hpa",
            "Sea level pressure (QFF) in hectopascals",
            &["station_id", "station_name"],
            registry
        )?;

        let wind_direction = register_gauge_vec_with_registry!(
            "wind_direction_degrees",
            "Wind direction in degrees",
            &["station_id", "station_name"],
            registry
        )?;

        let wind_speed = register_gauge_vec_with_registry!(
            "wind_speed_knots",
            "Wind speed in knots",
            &["station_id", "station_name"],
            registry
        )?;

        let visibility = register_gauge_vec_with_registry!(
            "visibility_meters",
            "Visibility in meters",
            &["station_id", "station_name"],
            registry
        )?;

        let precipitation_daily = register_gauge_vec_with_registry!(
            "precipitation_daily_mm",
            "Daily precipitation accumulator in millimeters",
            &["station_id", "station_name"],
            registry
        )?;

        // Rainfall accumulators
        let rain_1min = register_gauge_vec_with_registry!(
            "rain_1min_mm",
            "1-minute rainfall accumulation in millimeters",
            &["station_id", "station_name"],
            registry
        )?;

        let rain_15min = register_gauge_vec_with_registry!(
            "rain_15min_mm",
            "15-minute rainfall accumulation in millimeters",
            &["station_id", "station_name"],
            registry
        )?;

        let rain_30min = register_gauge_vec_with_registry!(
            "rain_30min_mm",
            "30-minute rainfall accumulation in millimeters",
            &["station_id", "station_name"],
            registry
        )?;

        let rain_1hour = register_gauge_vec_with_registry!(
            "rain_1hour_mm",
            "1-hour rainfall accumulation in millimeters",
            &["station_id", "station_name"],
            registry
        )?;

        let rain_2hour = register_gauge_vec_with_registry!(
            "rain_2hour_mm",
            "2-hour rainfall accumulation in millimeters",
            &["station_id", "station_name"],
            registry
        )?;

        let rain_3hour = register_gauge_vec_with_registry!(
            "rain_3hour_mm",
            "3-hour rainfall accumulation in millimeters",
            &["station_id", "station_name"],
            registry
        )?;

        let rain_6hour = register_gauge_vec_with_registry!(
            "rain_6hour_mm",
            "6-hour rainfall accumulation in millimeters",
            &["station_id", "station_name"],
            registry
        )?;

        let rain_12hour = register_gauge_vec_with_registry!(
            "rain_12hour_mm",
            "12-hour rainfall accumulation in millimeters",
            &["station_id", "station_name"],
            registry
        )?;

        let rain_24hour = register_gauge_vec_with_registry!(
            "rain_24hour_mm",
            "24-hour rainfall accumulation in millimeters",
            &["station_id", "station_name"],
            registry
        )?;

        // Metadata
        let observation_time = register_gauge_vec_with_registry!(
            "observation_time_timestamp",
            "Timestamp of the observation in Unix epoch seconds",
            &["station_id", "station_name"],
            registry
        )?;

        // Create the exporter with the custom registry
        let mut builder = prometheus_exporter::Builder::new(addr);
        builder.with_registry(registry.clone());
        let exporter = builder.start()?;

        Ok(Self {
            exporter,
            registry,
            temperature,
            humidity,
            pressure_station,
            pressure_sea_level,
            wind_direction,
            wind_speed,
            visibility,
            precipitation_daily,
            rain_1min,
            rain_15min,
            rain_30min,
            rain_1hour,
            rain_2hour,
            rain_3hour,
            rain_6hour,
            rain_12hour,
            rain_24hour,
            observation_time,
        })
    }

    /// Wait for a Prometheus scrape request and hold the guard
    pub fn wait_request(&self) -> MutexGuard<'_, ()> {
        self.exporter.wait_request()
    }

    /// Update metrics from an AWS observation
    pub fn update_from_observation(&self, obs: &AwsObservation) {
        let station_id = obs.id.as_str();
        let station_name = obs.sname.as_str();
        let labels = &[station_id, station_name];

        // Update core weather metrics
        if let Some(temp) = obs.temperature() {
            self.temperature.with_label_values(labels).set(temp);
        }

        if let Some(humidity) = obs.humidity() {
            self.humidity.with_label_values(labels).set(humidity);
        }

        if let Some(pressure) = obs.pressure() {
            self.pressure_station
                .with_label_values(labels)
                .set(pressure);
        }

        if let Some(pressure_sl) = obs.pressure_sea_level() {
            self.pressure_sea_level
                .with_label_values(labels)
                .set(pressure_sl);
        }

        if let Some(wind_dir) = obs.wind_dir() {
            self.wind_direction.with_label_values(labels).set(wind_dir);
        }

        if let Some(wind_speed) = obs.wind_speed_knots() {
            self.wind_speed.with_label_values(labels).set(wind_speed);
        }

        if let Some(visibility) = obs.visibility() {
            self.visibility.with_label_values(labels).set(visibility);
        }

        if let Some(precip) = obs.precip_daily() {
            self.precipitation_daily
                .with_label_values(labels)
                .set(precip);
        }

        // Update rainfall accumulators
        if let Some(rain) = obs.rain_1min() {
            self.rain_1min.with_label_values(labels).set(rain);
        }

        if let Some(rain) = obs.rain_15min() {
            self.rain_15min.with_label_values(labels).set(rain);
        }

        if let Some(rain) = obs.rain_30min() {
            self.rain_30min.with_label_values(labels).set(rain);
        }

        if let Some(rain) = obs.rain_1hour() {
            self.rain_1hour.with_label_values(labels).set(rain);
        }

        if let Some(rain) = obs.rain_2hour() {
            self.rain_2hour.with_label_values(labels).set(rain);
        }

        if let Some(rain) = obs.rain_3hour() {
            self.rain_3hour.with_label_values(labels).set(rain);
        }

        if let Some(rain) = obs.rain_6hour() {
            self.rain_6hour.with_label_values(labels).set(rain);
        }

        if let Some(rain) = obs.rain_12hour() {
            self.rain_12hour.with_label_values(labels).set(rain);
        }

        if let Some(rain) = obs.rain_24hour() {
            self.rain_24hour.with_label_values(labels).set(rain);
        }

        // Update observation timestamp
        if let Some(timestamp) = obs.timestamp() {
            self.observation_time
                .with_label_values(labels)
                .set(timestamp.timestamp() as f64);
        }
    }
}
