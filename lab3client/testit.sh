
cargo build --release

./target/release/lab_3 partial_hamlet_act_ii_script.txt 2>&1 | tee test_ham.out 

./target/release/lab_3 partial_macbeth_act_i_script.txt 2>&1 | tee test_mac.out

./target/release/lab_3 partial_macbeth_act_i_script_mod.txt 2>&1 | tee test_mac_mod.out

./target/release/lab_3 partial_macbeth_act_i_script_mod.txt whinge 2>&1 | tee test_mac_mod_whinge.out

emacs baseline_ham.out test_ham.out

emacs baseline_mac.out test_mac.out

emacs test_mac_mod_whinge.out

echo "diffing mac and modified mac test output"
diff test_mac_mod.out test_mac.out
