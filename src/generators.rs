use std::{fs::File, io::Write, path::Path};

use crate::parse::ParseResult;

pub fn json_generator(
    output_location: &Path,
    parse_results: Vec<ParseResult>,
) -> Result<(), Box<dyn std::error::Error>> {
    let j = serde_json::to_string_pretty(&parse_results);
    match j {
        Ok(json) => write_file(&output_location.join("output.json"), &json)?,
        Err(e) => Err(e)?,
    };
    Ok(())
}

pub fn html_generator(
    output_location: &Path,
    parse_results: Vec<ParseResult>,
) -> Result<(), Box<dyn std::error::Error>> {
    // Write CSS
    let css = "*,
::before,
::after {
  box-sizing: border-box;
}
html {
  font-family: system-ui, 'Segoe UI', Roboto, Helvetica, Arial, sans-serif,
    'Apple Color Emoji', 'Segoe UI Emoji';
  line-height: 1.15;
  -webkit-text-size-adjust: 100%;
  -moz-tab-size: 4;
  tab-size: 4;
}
body {
  margin: 0;
}

#header {
  background-color: #161b22;
  color: white;
  padding: 0.1px;
  border-bottom: 6px solid orange;
}

#header h2 {
  margin-left: 10px;
}

#main {
  margin: 10px;
}

.api-item {
  display: flex;
  align-items: center;
  background-color: rgb(203, 228, 237);
  border: 1px solid blue;
  height: 60px;
  border-radius: 4px;
  padding: 10px;
  margin-bottom: 10px;
  font-weight: bold;
}

.api-item span {
  margin-inline: 10px;
}

.api-item:has(:nth-child(3)) :last-child {
  color: gray;
  font-weight: 400;
}

.api-method {
  font-size: 16px;
  box-shadow: 0 4px 8px 0 rgba(0, 0, 0, 0.2), 0 6px 20px 0 rgba(0, 0, 0, 0.19);
  padding: 10px;
  border-radius: 4px;
}

.api-method.GET {
  background-color: lightgreen;
}

.api-method.POST {
  background-color: lightskyblue;
}

.api-method.DELETE {
  background-color: lightcoral;
}

.api-method.PATCH {
  background-color: rgb(214, 186, 186);
}

.api-method.PUT {
  background-color: rgb(250, 250, 142);
}
";
    write_file(&output_location.join("style.css"), css)?;

    let mut html = String::from(
        "<!DOCTYPE html>
<html lang='en'>
  <head>
    <meta charset='UTF-8' />
    <meta name='viewport' content='width=device-width, initial-scale=1.0' />
    <title>API Reference</title>

    <link rel='stylesheet' href='style.css' />
    <style></style>
  </head>
  <body>
    <div id='header'>
      <h2>API Reference</h2>
    </div>
    <div id='main'>\n",
    );

    let mut lines: Vec<String> = vec![];

    for item in parse_results {
        for method in &item.method_metadata {
            lines.push("\t\t<div class='api-item'>\n".to_owned());
            let method_type = &method.method_type;
            let comment = &method.comment;
            lines.push(format!(
                "\t\t\t<span class='api-method {method_type}'>{method_type}</span>\n",
            ));
            lines.push(format!("\t\t\t<span>{}</span>\n", item.path));

            if let Some(docstring) = comment {
                if !docstring.is_empty() {
                    lines.push(format!("\t\t\t<span>{}</span>\n", docstring[0].to_owned()));
                }
            }
            lines.push("\t\t</div>\n".to_owned());
        }
    }
    lines.push("\t</body>\n</html>".to_owned());
    html.push_str(lines.concat().as_str());

    write_file(&output_location.join("index.html"), html)?;

    Ok(())
}

fn write_file<S: AsRef<str>>(output_location: &Path, content: S) -> std::io::Result<()> {
    let mut file = File::create(output_location)?;
    file.write(content.as_ref().as_bytes())?;
    Ok(())
}
