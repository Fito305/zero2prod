use std::convert::TryInto;


#[derive(serde::Deserialize, Debug)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub application_port: u16
}

#[derive(serde::Deserialize, Debug)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: String,
    pub port: u16,
    pub host: String,
    pub database_name: String,
}

impl DatabaseSettings {
    pub fn connection_string(&self) -> String {
        println!("Username: {}", self.username);
        println!("Password: {}", self.password);
        println!("Host: {}", self.host);
        println!("Port: {}", self.port);
        println!("DBName: {}", self.database_name);
        format!(
            "postgres://{}:{}@{}:{}/{}",
                self.username, 
                self.password, 
                self.host, 
                self.port, 
                self.database_name
        )
    }
}

impl TryInto<Settings> for config::Config {
    type Error = config::ConfigError;

    fn try_into(self) -> Result<Settings, Self::Error> {
        let database = self.get("database")?; // .try_into()?; to convert the retrieved "database"
                                              // value (assumed to be another config::Config) into
                                              // a DatabaseSettings struct?
        let application_port = self.get("application_port")?;
        println!("{:?}:{:?}", database, application_port);
        Ok(Settings {
            database,
            application_port,
        })
    }
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    // Initialise our configuration reader
     let mut settings = config::Config::default();

    // Add configuration values from a file named `configuration`.
    // It will look for any top-level file with an extension
    // that `config` knows how to parse: yaml, json, etc.
     settings.merge(config::File::with_name("configuration"))?;

    // Try to convert the configuration values it read into
    // our Settings type
    settings.try_into()
}

