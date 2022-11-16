use crate::config::MLConfig;
use crate::models::MLModel;
use lightgbm;
use yummy_core::encoding::EntityValue;

pub struct LightgbmModel {
    pub model: lightgbm::Booster,
    pub feature_names: Vec<String>,
    pub num_features: i32,
}

impl LightgbmModel {
    pub fn new(config: MLConfig) -> LightgbmModel {
        let model_data = match config.flavors.lightgbm {
            Some(c) => c.data,
            _ => panic!("Wrong lightgbm config"),
        };

        let model_path = format!("{}/{}", config.base_path.unwrap(), model_data);

        let model = lightgbm::Booster::from_file(model_path.as_str()).unwrap();
        let feature_names = model.feature_name().unwrap();
        let num_features = model.num_feature().unwrap();

        LightgbmModel {
            model,
            feature_names,
            num_features,
        }
    }

    fn validate(&self, numeric_features: &Vec<Vec<f64>>) {
        if numeric_features.len() == 0 {
            panic!("Please provide numeric");
        }

        if numeric_features.first().unwrap().len() != self.num_features as usize {
            panic!(
                "Wrong number of numeric features (required {})",
                &self.num_features
            );
        }
    }

    pub fn reorder_all(
        &self,
        columns: Vec<String>,
        mut numeric_features: Vec<Vec<f64>>,
    ) -> Vec<Vec<f64>> {
        let num_rows = numeric_features.len();
        for n in 0..num_rows {
            numeric_features[n] = self.reorder(columns.clone(),numeric_features[n].clone());
        }

        numeric_features
    }

    fn find_swap(&self, columns: &Vec<String>) -> Option<(usize,usize)> {
        let num = columns.len();
        for i in 0..num {
            let col = &columns[i];
            if col != &self.feature_names[i] {
                for j in i..num {
                    if col == &self.feature_names[j] {
                        return Some((i,j));
                    }
                }
            }
        }

        None
    }

    pub fn reorder(&self, mut columns: Vec<String>, mut numeric_features: Vec<f64>) -> Vec<f64> {
        while let Some(swap) = self.find_swap(&columns) {
            columns.swap(swap.0,swap.1);
            numeric_features.swap(swap.0,swap.1);
        }

        numeric_features
    }
}

impl MLModel for LightgbmModel {
    fn predict(&self, _columns: Vec<String>, data: Vec<Vec<EntityValue>>) -> Vec<Vec<f64>> {
        let mut numeric_features: Vec<Vec<f64>> = Vec::new();
        let mut categorical_features: Vec<Vec<String>> = Vec::new();

        let num = (&data).len();
        for f in data {
            let mut num: Vec<f64> = Vec::new();
            let mut cat: Vec<String> = Vec::new();
            f.iter().for_each(|x| {
                match x {
                    EntityValue::INT32(v) => num.push(v.to_owned() as f64),
                    EntityValue::INT64(v) => num.push(v.to_owned() as f64),
                    EntityValue::FLOAT32(v) => num.push(v.to_owned() as f64),
                    EntityValue::FLOAT64(v) => num.push(v.to_owned() as f64),
                    EntityValue::BOOL(v) => num.push(v.to_owned() as i32 as f64),
                    EntityValue::STRING(v) => cat.push(v.to_owned()),
                    _ => panic!("Can't convert type"),
                };
            });

            numeric_features.push(num);
            categorical_features.push(cat);
        }

        self.validate(&numeric_features);

        let predictions = self.model.predict(numeric_features).unwrap();

        let num_pred = (&predictions[0]).len();
        if num_pred == num {
            predictions[0].iter().map(|x| vec![x.to_owned()]).collect()
        } else {
            predictions
                .iter()
                .map(|x| x.iter().map(|v| v.to_owned()).collect())
                .collect()
        }
    }
}

#[test]
fn test_feature_names() {
    let path = "../tests/mlflow/lightgbm_model/lightgbm_wine_model".to_string();
    //let path = "../tests/mlflow/catboost_model/iris_my_model".to_string();
    let config = MLConfig::new(&path);
    println!("{:?}", config);
    let lgb_model = LightgbmModel::new(config);
    let features = lgb_model.model.feature_name().unwrap();
    let nfeatures = lgb_model.model.num_feature();
    println!("{:?}", nfeatures);
    println!("{:?}", features);
}

#[test]
fn load_model_and_predict() {
    let path = "../tests/mlflow/lightgbm_model/lightgbm_my_model".to_string();
    //let path = "../tests/mlflow/catboost_model/iris_my_model".to_string();
    let config = MLConfig::new(&path);
    println!("{:?}", config);
    let lgb_model = LightgbmModel::new(config);

    let mut columns = Vec::new();
    let mut data = Vec::new();

    columns.push("age".to_string());

    let mut d1 = Vec::new();
    d1.push(EntityValue::INT32(8));
    data.push(d1);

    let mut d2 = Vec::new();
    d2.push(EntityValue::INT32(2));
    data.push(d2);

    println!("{:?}", data);

    let predictions = lgb_model.predict(columns, data);

    println!("{:?}", predictions);
}

#[test]
fn test_reorder() {
    let path = "../tests/mlflow/lightgbm_model/lightgbm_wine_model/".to_string();
    //let path = "../tests/mlflow/catboost_model/iris_my_model".to_string();
    let config = MLConfig::new(&path);
    println!("{:?}", config);
    let lgb_model = LightgbmModel::new(config);

    let mut columns = Vec::new();

    columns.push("12".to_string());
    columns.push("1".to_string());
    columns.push("2".to_string());
    columns.push("3".to_string());
    columns.push("5".to_string());
    columns.push("6".to_string());
    columns.push("7".to_string());
    columns.push("8".to_string());
    columns.push("9".to_string());
    columns.push("4".to_string());
    columns.push("10".to_string());
    columns.push("11".to_string());
    columns.push("0".to_string());

    let mut data = Vec::new();
    let mut d1 = Vec::new();
    d1.push(-1.2);
    d1.push(-0.1);
    d1.push(-0.2);
    d1.push(-0.3);
    d1.push(-0.4);
    d1.push(-0.5);
    d1.push(-0.6);
    d1.push(-0.7);
    d1.push(-0.8);
    d1.push(-0.9);
    d1.push(-1.0);
    d1.push(-1.1);
    d1.push(-0.0);
    data.push(d1.clone());
    data.push(d1.clone());

    let d2 = lgb_model.reorder_all(columns, data);
    println!("{:?}", d2);
}

#[test]
fn load_model_and_predict_multiclass() {
    let path = "../tests/mlflow/lightgbm_model/lightgbm_wine_model/".to_string();
    //let path = "../tests/mlflow/catboost_model/iris_my_model".to_string();
    let config = MLConfig::new(&path);
    println!("{:?}", config);
    let lgb_model = LightgbmModel::new(config);

    let mut columns = Vec::new();
    let mut data = Vec::new();

    columns.push("0".to_string());
    columns.push("1".to_string());
    columns.push("2".to_string());
    columns.push("3".to_string());
    columns.push("4".to_string());
    columns.push("5".to_string());
    columns.push("6".to_string());
    columns.push("7".to_string());
    columns.push("8".to_string());
    columns.push("9".to_string());
    columns.push("10".to_string());
    columns.push("11".to_string());
    columns.push("12".to_string());

    let mut d1 = Vec::new();
    d1.push(EntityValue::FLOAT32(0.913333));
    d1.push(EntityValue::FLOAT32(-0.598156));
    d1.push(EntityValue::FLOAT32(-0.425909));
    d1.push(EntityValue::FLOAT32(-0.929365));
    d1.push(EntityValue::FLOAT32(1.281985));
    d1.push(EntityValue::FLOAT32(0.488531));
    d1.push(EntityValue::FLOAT32(0.874184));
    d1.push(EntityValue::FLOAT32(-1.223610));
    d1.push(EntityValue::FLOAT32(0.050988));
    d1.push(EntityValue::FLOAT32(0.342557));
    d1.push(EntityValue::FLOAT32(-0.164303));
    d1.push(EntityValue::FLOAT32(0.830961));
    d1.push(EntityValue::FLOAT32(0.997086));
    data.push(d1.clone());
    data.push(d1.clone());

    println!("{:?}", data);

    let predictions = lgb_model.predict(columns, data);

    println!("{:?}", predictions);
}
