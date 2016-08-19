
use std::str::FromStr;
use std::error::Error;
use hyper::{Client, Url};
use hyper::mime::Mime;
use hyper::header::ContentType;
use serde_json as json;
use serde::{Serialize, Serializer};

const URL: &'static str = "https://api.apigw.smt.docomo.ne.jp/dialogue/v1/dialogue";

#[derive(Deserialize, Serialize, Debug, Clone, Copy)]
pub enum Mode {
    #[serde(rename="dialog")]
    Dialog,
    #[serde(rename="srtr")]
    Srtr,
}

#[derive(Serialize, Debug, Clone, Copy)]
pub enum Sex {
    #[serde(rename="男")]
    Man,
    #[serde(rename="女")]
    Woman,
}

#[derive(Serialize, Debug, Clone, Copy)]
pub enum Bloodtype {
    A,
    B,
    AB,
    O,
}

#[derive(Serialize, Debug, Clone, Copy)]
pub enum Constellations {
    #[serde(rename="牡羊座")]
    Aries,
    #[serde(rename="牡牛座")]
    Taurus,
    #[serde(rename="双子座")]
    Gemini,
    #[serde(rename="蟹座")]
    Cancer,
    #[serde(rename="獅子座")]
    Leo,
    #[serde(rename="乙女座")]
    Virgo,
    #[serde(rename="天秤座")]
    Libra,
    #[serde(rename="蠍座")]
    Scorpion,
    #[serde(rename="射手座")]
    Sagittarius,
    #[serde(rename="山羊座")]
    Capricorn,
    #[serde(rename="水瓶座")]
    Aquarius,
    #[serde(rename="魚座")]
    Pisces,
}

#[derive(Serialize, Debug, Clone, Copy)]
pub enum Type {
    Zero = 0,
    Sakurako = 20,
    Hayate = 30,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Response {
    pub utt: String,
    pub yomi: String,
    pub mode: Mode,
    pub da: u32,
    pub context: String,
}

#[derive(Serialize, Debug, Clone)]
pub struct Request<'a> {
    pub utt: &'a str,
    pub context: Option<String>,
    pub nickname: Option<String>,
    pub nickname_y: Option<String>,
    pub sex: Option<Sex>,
    pub bloodtype: Option<Bloodtype>,
    #[serde(rename="birthdateY")]
    pub birthdate_y: Option<u16>,
    #[serde(rename="birthdateM")]
    pub birthdate_m: Option<u8>,
    #[serde(rename="birthdateD")]
    pub birthdate_d: Option<u8>,
    pub age: Option<u8>,
    pub constellations: Option<Constellations>,
    pub place: Option<String>,
    pub mode: Option<Mode>,
    #[serde(serialize_with="type_to_u8")]
    pub t: Type,
}

fn type_to_u8<S>(x: &Type, s: &mut S) -> Result<(), S::Error>
    where S: Serializer
{
    let ret = match *x {
        Type::Zero => None,
        _ => Some(x.clone() as u8),
    };

    Serialize::serialize(&ret, s)
}

impl<'a> Request<'a> {
    pub fn new(str: &'a str, chat: &Chat) -> Self {
        Request {
            utt: str,
            context: chat.context.clone(),
            nickname: None,
            nickname_y: None,
            sex: None,
            bloodtype: None,
            birthdate_y: None,
            birthdate_m: None,
            birthdate_d: None,
            age: None,
            constellations: None,
            place: None,
            mode: chat.mode,
            t: chat.t,
        }
    }
}

#[derive(Debug)]
pub struct Chat {
    cli: Client,
    url: Url,
    mode: Option<Mode>,
    context: Option<String>,
    t: Type,
}

impl Chat {
    pub fn new(id: &str, t: Type) -> Self {
        let mut url = Url::from_str(URL).unwrap();
        url.set_query(Some(&("APIKEY=".to_string() + id)));
        Chat {
            cli: Client::new(),
            url: url,
            mode: None,
            context: None,
            t: t,
        }
    }

    pub fn request(&mut self, req: &Request) -> Result<Response, Box<Error>> {
        let mime = try!(Mime::from_str("application/json").map_err(|_| "mime error"));
        let req = try!(json::to_string(req));
        let res = try!(self.cli
            .post(self.url.clone())
            .header(ContentType(mime))
            .body(&req)
            .send());
        let res: Response = try!(json::from_reader(res));

        self.mode = Some(res.mode);
        self.context = Some(res.context.clone());
        Ok(res)
    }
}
