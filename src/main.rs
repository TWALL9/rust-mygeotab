use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct AuthenticationParams {
    #[serde(rename = "userName")]
    user_name: String,
    password: String,
    database: String
}

#[derive(Debug, Serialize, Deserialize)]
struct AuthenticateRequest {
    method: String,
    params: AuthenticationParams
}

#[derive(Debug, Serialize, Deserialize)]
struct Credentials {
    #[serde(rename = "userName")]
    username: String,
    #[serde(rename = "sessionId")]
    session_id: String,
    database: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct AuthResultBody {
    credentials: Credentials,
    path: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct AuthResult {
    result: AuthResultBody
}

#[derive(Debug, Serialize, Deserialize)]
struct DeviceSearch {
    #[serde(rename = "serialNumber")]
    serial_number: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct DeviceRequestParams {
    #[serde(rename = "typeName")]
    type_name: String,
    search: DeviceSearch,
    credentials: Credentials,
}

#[derive(Debug, Serialize, Deserialize)]
struct DeviceRequest
{
    method: String,
    params: DeviceRequestParams,
}

fn main() {
    let login = AuthenticateRequest {
        method: String::from("Authenticate"),
        params: AuthenticationParams {
            user_name: String::from("usernamegoeshere@dummyemailcom"),
            password: String::from("F@ncyPassw0rd"),
            database: String::from("db_goes_here")
        }
    };

    let url = "https://my.geotab.com/apiv1";
    let client = reqwest::blocking::Client::new();
    let auth_res = client.post(url).json(&login).send().unwrap();
    let res_json: AuthResult = auth_res.json().unwrap();
    let mut url = String::from("https://");
    url.push_str(&res_json.result.path);
    url.push_str("/apiv1");
    let creds = res_json.result.credentials;

    let dev_req = DeviceRequest {
        method: String::from("Get"),
        params: DeviceRequestParams {
            type_name: String::from("Device"),
            search: DeviceSearch {
                serial_number: String::from("G9DEADBEEF")
            },
            credentials: creds
        }
    };
    let device_result = client.post(&url).json(&dev_req).send().unwrap();
    println!("{}", device_result.text().unwrap());
}
