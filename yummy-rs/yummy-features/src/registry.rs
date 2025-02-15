use crate::config::Config;
use crate::types::Registry as RegistryProto;
use protobuf::Message;
use std::error::Error;
use std::fs;
use yummy_core::config::read_config_bytes;
//use tokio::prelude::Future;

#[derive(Debug, Clone)]
pub struct Registry {
    pub feature_views: Vec<FeatureView>,
    pub feature_services: Vec<FeatureService>,
}

#[derive(Debug, Clone)]
pub struct FeatureView {
    pub project: String,
    pub name: String,
    pub features: Vec<String>,
    pub full_feature_names: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct FeatureService {
    pub name: String,
    pub project: String,
    pub full_feature_names: Vec<String>,
}

impl Registry {
    pub async fn new(config: Config) -> Result<Self, Box<dyn Error>> {
        let data = read_config_bytes(&config.registry).await?;
        let registry_proto: RegistryProto::Registry = Message::parse_from_bytes(&data)?;
        let feature_views = Registry::read_feature_views(&registry_proto);
        let feature_services = Registry::read_read_feature_services(&registry_proto);

        Ok(Registry {
            feature_services,
            feature_views,
        })
    }

    fn read_read_feature_services(registry_proto: &RegistryProto::Registry) -> Vec<FeatureService> {
        let mut feature_services: Vec<FeatureService> = Vec::new();

        (&registry_proto.feature_services)
            .into_iter()
            .for_each(|fs| {
                let spec = &fs.spec;
                let name = spec.name.clone();
                let project = spec.project.clone();

                let mut full_feature_names: Vec<String> = Vec::new();

                let features = &spec.features;

                for f in features {
                    for c in &f.feature_columns {
                        full_feature_names.push(format!("{}:{}", f.feature_view_name, c.name));
                    }
                }

                feature_services.push(FeatureService {
                    name,
                    project,
                    full_feature_names,
                });
            });

        feature_services
    }

    fn read_feature_views(registry_proto: &RegistryProto::Registry) -> Vec<FeatureView> {
        let mut feature_views: Vec<FeatureView> = Vec::new();

        (&registry_proto.feature_views).into_iter().for_each(|fv| {
            let spec = &fv.spec;
            let ft = &spec.features;
            let project = spec.project.clone();
            let name = spec.name.clone();

            let features: Vec<String> = ft.into_iter().map(|f| f.name.clone()).collect();
            let full_feature_names: Vec<String> = ft
                .into_iter()
                .map(|f| format!("{}:{}", name.clone(), f.name.clone()))
                .collect();
            feature_views.push(FeatureView {
                name,
                project,
                features,
                full_feature_names,
            })
        });

        feature_views
    }

    pub fn get_feature_service(&self, name: String, project: String) -> Vec<String> {
        match (&self.feature_services)
            .into_iter()
            .filter(|fs| fs.name == name && fs.project == project)
            .last()
        {
            Some(f) => f.full_feature_names.clone(),
            None => Vec::new(),
        }
    }

    pub fn check_features(&self, features: Vec<String>) -> bool {
        for feature in features {
            let split: Vec<&str> = feature.split(":").collect();
            let feature_view_name = split[0];
            let feature_name = split[1];

            if let Some(f) = (&self.feature_views)
                .into_iter()
                .filter(|fv| fv.name == feature_view_name)
                .last()
            {
                if !(&f.features).into_iter().any(|n| n == &feature_name) {
                    return false;
                }
            } else {
                return false;
            }
        }

        true
    }
}

#[tokio::test]
async fn read_registry_test() -> Result<(), Box<dyn Error>> {
    let path = "../tests/feature_store.yaml".to_string();
    let config = Config::new(&path).await?;
    println!("{config:?}");
    let registry = Registry::new(config).await?;
    //println!("{:?}", registry);

    let features = registry.get_feature_service(
        "driver_activity_basic".to_string(),
        "adjusted_drake".to_string(),
    );

    println!("{features:?}");

    let check = registry.check_features(features);
    assert_eq!(check, true);

    //read_feature_service_spec("./tests/registry.db".to_string(), "test".to_string());
    Ok(())
}
