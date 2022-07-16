# Placeholder

A Placeholder Templating Engine without the complexity

You're probably familiar with how `printf!()`, `format!()`, and friends use
`{<named>}` placeholders. This module uses that same style within its templates
to give you that same feel but for use within a complete text templating engine.

# Example 1

```
use placeholder::render;
use std::collections::HashMap;

fn main() {
  let template = String::from("<h1>{greet} {name}</h1><p>Do you like {food}?</p>");

  let mut values = HashMap::new();
  values.insert(String::from("greet"), String::from("Hello"));
  values.insert(String::from("name"), String::from("Homer"));
  values.insert(String::from("food"), String::from("Donuts"));

  assert!(render(&template, &values)
    == Ok(String::from("<h1>Hello Homer</h1><p>Do you like Donuts?</p>")));

}
```

# Example 2 (missing placeholder values)

```
use placeholder::render;
use std::collections::HashMap;

fn main() {
  let template = String::from("<h1>{greet} {name}</h1>");

  let mut values = HashMap::new();
  values.insert(String::from("greet"), String::from("Hello"));

  assert!(render(&template, &values)
    == Err(String::from("name")));
}
```

# Support

Please report any bugs or feature requests at:

* [https://github.com/alfiedotwtf/placeholder/issues](https://github.com/alfiedotwtf/placeholder/issues)

Feel free to fork the repository and submit pull requests :)

# Author

[Alfie John](https://www.alfie.wtf) &lt;[alfie@alfie.wtf](mailto:alfie@alfie.wtf)&gt;

# Warranty

IT COMES WITHOUT WARRANTY OF ANY KIND.

# Copyright and License

Copyright (C) 2022 by Alfie John

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU General Public License and GNU Free Documentation License
as published by the Free Software Foundation, either version 3 of the GPL or
1.3 of the GFDL, or (at your option) any later version.

This program is distributed in the hope that it will be useful, but WITHOUT ANY
WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A
PARTICULAR PURPOSE. See the GNU General Public License for more details.

You should have received a copy of the GNU General Public License along with
this program. If not, see [https://www.gnu.org/licenses/](https://www.gnu.org/licenses/).
