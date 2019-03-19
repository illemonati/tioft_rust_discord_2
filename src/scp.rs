use reqwest;
use select::document::Document;
use select::predicate::{Attr, Class, Name, Predicate};
use regex::Regex;

pub struct SCP {
    pub url: String,
    pub item_n: String,
    pub object_class: String,
    pub description: String,
    pub procedure: String,
}

impl SCP {
    pub fn new(n: i64) -> SCP {
        let number: &str;
        if (&n < &100i64) && &n > &9i64 {
            number = &format!("0{}", &n);
        }else{
            number = &format!("{}", &n);
        }
        let mut scp_object = SCP{
            url: format!("http://www.scp-wiki.net/scp-{}", &n),
            item_n: format!("SCP-{}", number),
            object_class: String::new(),
            description: String::new(),
            procedure: String::new(),
        };
        scp_object.object_class = (&scp_object).get_object_class_inner();
        scp_object.procedure =  (&scp_object).get_procedure_inner();
        scp_object.description =  (&scp_object).get_description_inner();

        return scp_object;
    }

    fn get_object_class_inner(&self) -> String {
        let body = reqwest::get((self.url).as_str()).unwrap().text().unwrap();
        let mut document= Document::from(body.as_str());
        let mut object_class: String = String::from("Not Found");
        for node in document.find(Name("strong")) {
            if node.inner_html() != "Object Class:" {
                continue;
            }
            let parent_html = node.parent().unwrap().inner_html();
            let re = Regex::new("[^>]*$").unwrap();
            let res = re.find(parent_html.as_str()).unwrap().as_str();
            object_class = String::from(res);
        }
        return object_class;
    }

    fn get_description_inner(&self) -> String {
        let body = reqwest::get((self.url).as_str()).unwrap().text().unwrap();
        let mut document= Document::from(body.as_str());
        let mut description: String = String::from("Not Found");
        for node in document.find(Name("strong")) {
            if node.inner_html() != "Description:" {
                continue;
            }
            let parent_html = node.parent().unwrap().inner_html();
            let re = Regex::new("[^>]*$").unwrap();
            let res = re.find(parent_html.as_str()).unwrap().as_str();
            description = String::from(res);
        }
        return description;
    }

     fn get_procedure_inner(&self) -> String {
        let body = reqwest::get((self.url).as_str()).unwrap().text().unwrap();
        let mut document= Document::from(body.as_str());
        let mut procedure: String = String::from("Not Found");
        for node in document.find(Name("strong")) {
            if node.inner_html() != "Special Containment Procedures:" {
                continue;
            }
            let parent_html = node.parent().unwrap().inner_html();
            let re = Regex::new("[^>]*$").unwrap();
            let res = re.find(parent_html.as_str()).unwrap().as_str();
            procedure = String::from(res);
        }
        return procedure;
    }

    pub fn get_description_short(&self) -> String {
        if self.description.len() < 2048 {
            return self.description.clone();
        }
        let slice = &self.description.as_str()[..2047];
        return String::from(slice);
    }

    pub fn get_procedure_short(&self) -> String {
        if self.procedure.len() < 2048 {
            return self.procedure.clone();
        }
        let slice = &self.procedure.as_str()[..2047];
        return String::from(slice);
    }
}

