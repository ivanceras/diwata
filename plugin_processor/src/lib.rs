
#![feature(type_ascription)] 
extern crate toml;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate diwata_user_role;
extern crate hyper;


use toml::Value;
use std::fs::File;
use std::io::Read;
use std::fs;
use hyper::Request;

macro_rules! handle_request {
    ($x:ident, $req:ident) => {
        {
        let ret = $x::handle_request($req: Request);
        }
    };
}


#[derive(Debug, Deserialize)]
struct Plugin{
    package: Package,
}

#[derive(Debug, Deserialize)]
struct Package{
    name: String,
    version: String,
    description: String,
    license: String,
    authors: Vec<String>,
}

fn scan_plugins() -> std::io::Result<Vec<Plugin>> {
    let mut plugins = vec![];
    for entry in fs::read_dir("../installed_plugins/")? {
        let dir = entry?;
        println!("{:?}", dir.path());
        let mut plug_path = dir.path();
        if plug_path.is_dir(){
            plug_path.push("Cargo.toml");
            println!("plug path: {:?}", plug_path);
            let mut file = File::open(plug_path).expect("file not found");
            let mut contents = String::new();
                file.read_to_string(&mut contents)
                    .expect("something went wrong reading the file");
            let plugin:Plugin = toml::from_str(&contents).unwrap();
            plugins.push(plugin)
        }
    }
    Ok(plugins)
}

/// create a project crate that enumerate the plugins and call their handler
fn plugin_crate_writer() -> std::io::Result<String>{
    let mut buff = String::new();
    let plugins = scan_plugins()?;
    buff += "fn handle_request(head: &str, req: Request){\n";
    for (i,plugin) in plugins.iter().enumerate(){
        if i == 0 {
            buff += "    if ";
        }
        else{
            buff += "    else if ";
        }
        buff += &format!("head == \"{}\"{{\n",plugin.package.name);
        buff += &format!("        plugin::handle_request({},req);\n", plugin.package.name);
        buff += "    }\n";
    }
    buff += "}\n";
    Ok(buff)
}

fn generate_plugin_stub()->std::io::Result<()>{
    let content = plugin_crate_writer()?;
    let mut file = File::create("../plugins/src/generated.rs")?;
    file.write_all(content.as_bytes())?;
    Ok(())
}

fn handle_request(head: &str, req: Request){
    if head == "diwata_user_role"{
        plugin!(diwata_user_role, req);
    }
}


#[cfg(test)]
mod test{
    
    #[test]
    fn test_scan_plugins(){
        super::scan_plugins();
        panic!();
    }

    #[test]
    fn test_plugin_crate_writer(){
        let code = super::plugin_crate_writer().unwrap();
        println!("{}", code);
        panic!();
    }

}
