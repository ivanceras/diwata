extern crate diwata_intel;
extern crate webview;

use diwata_intel::data_container::AppData;
use sauron::Component;

fn main() {
    let app = webview::make_app(AppData::default());
    let view = app.view();
    println!(
        r#"

<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1">
    <link rel="stylesheet" type="text/css" href="/style.css"/>
    <title>Diwata</title>
</head>
<body style='margin: 0; padding: 0; width: 100%; height: 100%;'>
  <div id="web-app" style='width: 100%; height: 100%;'>
    "#
    );
    println!("{}", view);
    println!(
        r#"

  </div>
</body>
</html>
    "#
    );
}
