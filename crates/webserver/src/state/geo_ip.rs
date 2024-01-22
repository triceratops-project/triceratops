use error_stack::{Context, Report, Result, ResultExt};
use flate2::read::GzDecoder;
use maxminddb::Reader as MaxMindReader;
use std::{
    fmt::Display,
    fs::{self, File},
    io::{self, BufWriter},
    path::Path,
};
use tar::Archive;

#[derive(Debug)]
pub struct GeoIpError;

impl Display for GeoIpError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("GeoIp module error")
    }
}

impl Context for GeoIpError {}

// Make config or something...
static MAX_MIND_DB: &'static str = "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";

#[derive(Debug)]
pub struct GeoIp(MaxMindReader<Vec<u8>>);

impl GeoIp {
    pub async fn new() -> Result<Option<Self>, GeoIpError> {
        let path = Path::new("./storage/GeoLite2-City.mmdb");

        if !path.exists() {
            Self::download()
                .await
                .attach_printable("Failed to download MaxMindDB database")?;
        }

        let buf = fs::read("./storage/GeoLite2-City.mmdb")
            .map_err(Report::from)
            .attach_printable("Failed to read GeoLite2-City from ./storage/GeoLite2-City.mmdb")
            .change_context(GeoIpError)?;

        let a = MaxMindReader::from_source(buf)
            .map_err(Report::from)
            .attach_printable("Failed to parse MaxMindDB")
            .change_context(GeoIpError)?;

        Ok(Some(Self(a)))
    }

    async fn download() -> Result<(), GeoIpError> {
        let request_url = format!("https://download.maxmind.com/app/geoip_download?edition_id=GeoLite2-City&license_key={}&suffix=tar.gz", MAX_MIND_DB);

        let mm_db_bytes = reqwest::get(request_url)
            .await
            .unwrap()
            .bytes()
            .await
            .unwrap();

        fs::write("./storage/GeoLite2-City.tar.gz", mm_db_bytes).unwrap();

        let tar_gz = File::open("./storage/GeoLite2-City.tar.gz").unwrap();
        let tar = GzDecoder::new(tar_gz);
        let mut archive = Archive::new(tar);

        for entry in archive.entries().unwrap() {
            let mut entry = entry.unwrap();

            if entry
                .path()
                .unwrap()
                .as_os_str()
                .to_str()
                .unwrap()
                .to_string()
                .ends_with(".mmdb")
            {
                let fuck = File::create("./storage/GeoLite2-City.mmdb").unwrap();
                let mut out = BufWriter::new(fuck);

                io::copy(&mut entry, &mut out).unwrap();
            }
        }

        fs::remove_file("./storage/GeoLite2-City.tar.gz").unwrap();

        Ok(())
    }

    pub fn database(&self) -> &MaxMindReader<Vec<u8>> {
        let GeoIp(inner) = self;

        inner
    }
}
