# CSE 542 Fall 2024 Lab 2

## Section 1. Introduction

1. The number of the lab:

   CSE 542 Fall 2024 Lab 2

2. The names and e-mail addresses of all team members:

   Tianzhuang Xiong
   <t.xiong@wustl.edu>

   Yimu Liu
   <l.yimu@wustl.edu>

   Yiqun Du
   <d.yiqun@wustl.edu>

3. An overview of how your program was designed.

## Section 2. Design

1. Unzip or otherwise unpack your files.
2. Build your program(s).
3. Run your program(s) on the CEC Linux Lab machines where your lab solutions will be evaluated.

## Section 3. Development and Testing

Provide a reasonably detailed description of how you developed and tested your solution, including each stage of how you refactored and extended your Lab 1 solution to implement this lab assignment.

Please also describe the kinds of script, configuration, and character part files you used and their formats (including well-formed and badly-formed content to test how your program handled those variations), and any other scenarios that you tested that you consider important.

Please also describe the kinds of script, configuration, and character part files you used and their formats (including well formed and badly formed content to test how your program handled those variations),

and any other scenarios that you tested that you consider important.

### Subsection: Modules

- Created the `lab2` directory, moving `declarations.rs` and `script_gen.rs` to `lab2`.
- Declared the `lab2` module in `main.rs`.
- Created `mod.rs` under `lab2`, declaring:

  ```rust
  pub mod declarations;
  pub mod script_gen;
  ```

- Moved some variables in `declarations.rs` into `script_gen.rs` and `main.rs`.

### Subsection: Structs

- Step 5:

  Refactor the `script_gen`.

  - `prepare` method: Add line tuples to the player's container. Call the `sort` method to order its entries in ascending line number order once all line tuples have been added to the player.
  - `speak` method: Print the current script line of the current player, and switch the current player if the owner of the current script line and the current player do not match.
  - `next_line` method: Return the valid script line number of the player, return `None` if the line number is invalid.

- Step 7:

  Move `grab_trimmed_file_lines` from `script_gen.rs` to `play.rs`.

### Subsection: Return Wrapper

- Step 9:

  Create `return_wrapper.rs` under `lab2`.

  Modify all the `return` statements in `main.rs` to return a `ReturnWrapper` instance.

### Subsection: Scene Fragments

In the implemention of `scene_fragments.rs` and it's `read_config` function:

```rust
for line in &config_lines[FIRST_LINE_INDEX..] {
    SceneFragment::add_config(line, scene_frag_config);
}
```

skip the frist line in each config_lines, which is beacuse the first lines are the titles in the Lab1,

and we forgot to remove this section initially, so we met some issues regarding this section.

Finally we find it and solve the issue.

```
please summarize how you extended your solution to manage multiple consecutive scene fragments, including any design challenges you encountered and how you addressed those.
```

answer: We followed your instructions to use the function 'enter'and 'exit' to manage transitions between scene fragments. We iterated through the players in the current scene fragment checked if the 'other' scene fragment contains each player, if not we print the entrance of such player. we implemented our own 'contain' method because we realized that we can not use the contains method of the vector because our implementation of Eq/PartialEq does not apply to this specific senario.

Some designs:

We choose a more complex manner to deal with the error situations. To handle the debug and the recite function in Scene_fragment.rs, we create a find_next_player function, find the sole player which contain the line number we have.

Due to we don't want to modify the partial_equal, so we define the contain_name function in Scene_fragment.rs, which is used to check whether the passed strings are included in the self.players. This function is implied in the enter and exit section.

### Subsection: Testing

#### Testing the examples

usage:

```bash
cargo run rsrc/lab2_test/partial_hamlet_act_ii_script.txt <whinge>
```

The code runs succese and will print

`WHINGE: MISSING AT LINE NUMBER 0`

#### Testing the missing and duplicated lines

usage:

```bash
cargo run rsrc/lab2_test_missing_duplicated_line_num/script.txt <whinge>
```

This test case is to test the missing and duplicated line number.

#### Testing the script file

usage:

```bash
cargo run rsrc/lab2_test_script/script.txt
```

This test case is to test the script file.
The Code will print the error code and the line in the script file.
