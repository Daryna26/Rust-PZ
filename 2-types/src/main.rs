mod part_1;
mod part_2;

fn main() {
    println!("Assignment 2 - Types examples");

    // Typestate
    let new_post = part_1::NewPost {
        content: "Hello Rust".into(),
    };
    let unmod = new_post.publish();
    let published = unmod.allow();
    let deleted = published.delete();

    println!("Deleted post content: {}", deleted.content);

    // JSON - TOML
    let json_data = r#"
    {
      "type": "success",
      "stream": {
        "user_id": "8d234120-0bda-49b2-b7e0-fbd3912f6cbf",
        "is_private": false,
        "settings": 45345,
        "shard_url": "https://n3.example.com/sapi",
        "public_tariff": {
          "id": 1,
          "price": 100,
          "duration": "1h",
          "description": "test public tariff"
        },
        "private_tariff": {
          "client_price": 250,
          "duration": "1m",
          "description": "test private tariff"
        }
      },
      "gifts": [
        {
          "id": 1,
          "price": 2,
          "description": "Gift 1"
        },
        {
          "id": 2,
          "price": 3,
          "description": "Gift 2"
        }
      ],
      "debug": {
        "duration": "234ms",
        "at": "2019-06-28T08:35:46+00:00"
      }
    }
    "#;

    let toml_str = part_2::json_to_toml(json_data);
    println!("\nTOML output:\n{}", toml_str);
}
