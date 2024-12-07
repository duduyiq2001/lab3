# CSE 542 Fall 2024 Lab 3

## Section 1. Introduction

1. The number of the lab:

   CSE 542 Fall 2024 Lab 3

2. The names and e-mail addresses of all team members:

   Yiqun Du
   <d.yiqun@wustl.edu>

   Yimu Liu
   <l.yimu@wustl.edu>

3. An overview of how your program was designed.

   1.Step by step following the lab instruction.

   2.the process config of both play and scenefragments are parallelized using threads

   3.the client process always append a "\n" after filename, the "\n" acts as a delimiter to inform the server that: I am done writing, please respond,

   and the detection of which is done by the read_line function in the server side

4. types of program error defined in client:

   ```// Return codes
   pub const SUCCESS: u8 = 0;
   pub const ERR_BAD_COMMAND: u8 = 1;
   
   /// indicating the error stems from script generation, while the command format is correct
   pub const ERR_SCRIPT_GENERATION_FAIL: u8 = 2;
   /// indicating that the error that happens before any config scripts were obtained
   /// mostly IO/network/read/write error, we call it IO_ERROR
   pub const IO_ERROR: u8 = 3;
   
   ```

5. types of error for the server program

```rust
   pub const BINDING_ERROR: u8 = 1;
      pub const FILE_ERROR: u8 = 2; // not returned by main, only used internally
```

## Section 2

1. Unzip or otherwise unpack your files.
2. Build your program(s).
3. Run your program(s) on the CEC Linux Lab machines where your lab solutions will be evaluated.

## Section 3. Development and Test

### Development details

### Test

We take some the testfile from `cse542_fl24_lab2_Du_Liu_Xiong/target/debug`, and the introduction of Lab 3.

We also add some testcase by ourselves.

We write a script `testit.sh` in `lab3client` folder to test all the testcase easily by using simple command.

A brief detail of the testcases in `testit.sh` are showing below.

```bash
# Define test case details in a list
TEST_CASE_DETAILS=(
    "Test case 0: Test 'partial_hamlet_act_ii_script.txt', check against 'baseline_ham.out'."
    "Test case 1: Test 'partial_hamlet_act_ii_script.txt'. with argument 'whinge'."
    "Test case 2: Test 'partial_macbeth_act_i_script.txt'. check against 'baseline_mac.out'."
    "Test case 3: Test 'partial_macbeth_act_i_script.txt'. with argument 'whinge'."
    "Test case 4: Test 'partial_macbeth_act_i_script_mod.txt'. check against 'baseline_mac_mod.out'."
    "Test case 5: Test 'partial_macbeth_act_i_script_mod.txt'. with argument 'whinge'."
    "Test case 6: Test 'partial_mac_script_mod2.txt'."
    "Test case 7: Test 'partial_mac_script_mod3.txt'."
)
```

You can run the test by using `./testit.sh [test_id 0~9]`.

We also write a script `testit_server.sh` in `lab3client` folder to test all the testcase easily by using simple command.

You can run the test by using `./testit_server.sh [test_id 0~2]` after running the server in `lab3server` folder by `cargo run 127.0.0.1:1024`.

A brief detail of the testcases in `testit_server.sh` are showing below.

```bash
# Define test case details in a list
TEST_CASE_DETAILS=(
    "Test case 0: Test 'partial_hamlet_act_ii_script.txt' from server, check against 'baseline_ham.out'."
    "Test case 1: Multiple clients connect to the server."
    "Test case 2: Testing File doesn't exist on server."
)
```

Here is some of the bug we found using the testcase.

#### Local test case 4

```bash
./testit.sh 4
```

Differences 1:

```plaintext
[test_mac_mod.out]                                              [baseline_mac_mod.out]
...                                                             ...
SOLDIER.                                                        SOLDIER.
Doubtful it stood;                                              Doubtful it stood;
As two spent swimmers that do cling together                    As two spent swimmers that do cling together
And choke their art. The merciless Macdonwald,--                And choke their art. The merciless Macdonwald,--
Worthy to be a rebel,--for to that                            <
The multiplying villainies of nature                            The multiplying villainies of nature
                                                              > Worthy to be a rebel,--for to that
Do swarm upon him,--from the Western isles                      Do swarm upon him,--from the Western isles
Of kerns and gallowglasses is supplied;                         Of kerns and gallowglasses is supplied;
...                                                             ...
```

base on the script in `SOLDIER_macbeth_i_2a_mod.txt`, these two lines using same line number 13.

```plaintext
13 Worthy to be a rebel,--for to that
13 The multiplying villainies of nature
```

So the order of the output is different from the baseline, we just ignore this tiny difference.

Differences 2:

```plaintext
[test_mac_mod.out]                                              [baseline_mac_mod.out]
...                                                             ...
MALCOLM.                                                        MALCOLM.
The worthy Thane of Ross.                                       The worthy Thane of Ross.

LENNOX.                                                         LENNOX.
                                                              > What a haste looks through his eyes! So should he look
That seems to speak things strange.                             That seems to speak things strange.

ROSS.                                                           ROSS.
God save the King!                                              God save the King!
...                                                             ...
```

This is because the script line in `LENNOX_macbeth_i_2b_mod.txt` contains illegal formatting.

```plaintext
         2    What a haste looks through his eyes! So should he look 
3 That seems to speak things strange.
```

After adding an extra trim to `unparsed_line` in `add_script_line` function in `player.rs`, the output is the same as the baseline.

#### Local test case 6 and 7

```bash
./testit.sh 6
./testit.sh 7
```

test case 6 got a nonexist file in script `partial_mac_script_mod2.txt`.

test case 7 got a nonexist file in config file `mac_i_1_hh.txt` from `partial_mac_script_mod3.txt`.

The assumption from the project description is that those non-existent config file would cause one of the threads to panic

and then the panic will be captured by the main thread in the form of an error, which is exactly what happens in those two test cases.

these two test case show that the program can handle the error gracefully.
