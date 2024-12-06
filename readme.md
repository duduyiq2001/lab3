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

    Step by step following the lab instruction.

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

The whole detail are showing below.

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
    "Test case 7: Test 'partial_mac_script_mod2.txt'. with argument 'whinge'."
    "Test case 8: Test 'partial_mac_script_mod3.txt'."
    "Test case 9: Test 'partial_mac_script_mod3.txt'. with argument 'whinge'."
)
```

Here is some of the bug we found using the testcase.

#### test case 4

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
